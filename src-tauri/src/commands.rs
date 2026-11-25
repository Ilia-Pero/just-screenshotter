//! Commands that windows of the program call.

use std::collections::HashMap;
use std::thread;

use chrono::Local;
use screenshotter_film::{Area, Order};
use screenshotter_phrase::{Language, Phrase, LANGUAGES};
use screenshotter_store::{Failure, Listened};
use screenshotter_upload::Server;
use serde::{Deserialize, Serialize};
use tauri::ipc::{InvokeBody, Request, Response};
use tauri::{AppHandle, Manager, State, WebviewWindow};

use crate::album::Album;
use crate::studio::{Progress, Studio, KIND};
use crate::{clipboard, files, stage, tell, tray};

/// The picture sent by the editor. Old `WebKit` does not pass the body of a request,
/// and then Tauri sends the bytes as a list of numbers.
fn picture(request: &Request<'_>) -> Result<Vec<u8>, String> {
    match request.body() {
        InvokeBody::Raw(bytes) => Ok(bytes.clone()),
        InvokeBody::Json(list) => list
            .as_array()
            .and_then(|numbers| {
                numbers
                    .iter()
                    .map(|number| number.as_u64().and_then(|byte| u8::try_from(byte).ok()))
                    .collect()
            })
            .ok_or_else(|| "The editor has sent no picture".to_owned()),
    }
}

/// The screenshot for the window that asks for it. It may have to wait, so it does not hold
/// the main thread.
#[tauri::command(async)]
pub fn shot(window: WebviewWindow, album: State<'_, Album>) -> Result<Response, String> {
    album.take(window.label()).map(Response::new)
}

/// Shows the editor: it calls this command when the screenshot is already drawn.
#[tauri::command]
pub fn reveal(window: WebviewWindow) -> Result<(), String> {
    window.show().map_err(|failure| failure.to_string())?;
    window.set_focus().map_err(|failure| failure.to_string())
}

/// Puts the picture on the clipboard.
#[tauri::command]
pub fn copy(window: WebviewWindow, request: Request<'_>) -> Result<(), String> {
    clipboard::put(&window, &picture(&request)?).map_err(|failure| failure.to_string())
}

/// Saves the picture to the folder with screenshots.
#[tauri::command]
pub fn save(app: AppHandle, request: Request<'_>) -> Result<(), String> {
    let png = picture(&request)?;
    files::folder(&app)
        .and_then(|folder| folder.store(&png, Local::now()))
        .map(|_| ())
        .map_err(|failure| failure.to_string())
}

/// Whether there is where to upload.
#[tauri::command]
pub fn shares(app: AppHandle) -> bool {
    files::server(&app)
        .and_then(|file| file.read())
        .is_ok_and(|server| server.ready())
}

/// Uploads the picture and puts the link on the clipboard.
#[tauri::command(async)]
pub fn share(app: AppHandle, window: WebviewWindow, request: Request<'_>) -> Result<(), String> {
    let png = picture(&request)?;
    files::server(&app)
        .and_then(|file| file.read())
        .and_then(|server| server.link(&png))
        .and_then(|link| clipboard::put_text(&window, &link))
        .map_err(|failure| failure.to_string())
}

/// What windows say to a person, in the current language.
#[tauri::command]
pub fn words() -> HashMap<String, String> {
    Language::current().texts()
}

/// What can be recorded together with video, and what was chosen the last time.
#[derive(Serialize)]
pub struct Listens {
    /// Whether the sound of the computer can be recorded.
    hears: bool,
    sound: bool,
    voice: bool,
}

/// What to offer before filming.
///
/// @todo #1 On macOS only the microphone is recorded. Since macOS 13 `ScreenCaptureKit` gives
///  the sound of the computer too: film through it there, and offer the sound.
#[tauri::command]
pub fn listens(app: AppHandle) -> Result<Listens, String> {
    let ears = files::settings(&app)
        .and_then(|file| file.read())
        .map_err(|failure| failure.to_string())?
        .ears;
    let hears = !cfg!(target_os = "macos");
    Ok(Listens {
        hears,
        sound: hears && ears.sound,
        voice: ears.voice,
    })
}

/// What a person wants to film.
#[derive(Deserialize)]
pub struct Wish {
    /// The area on the screenshot.
    area: Area,
    /// The size of the screenshot.
    sheet: (u32, u32),
    sound: bool,
    voice: bool,
}

/// Closes the editor and films the area. What goes wrong is told by the program itself, because
/// there is no editor any more.
#[tauri::command(async)]
pub fn film(app: AppHandle, window: WebviewWindow, wish: Wish) -> Result<(), String> {
    let order = order(&app, &window, &wish).map_err(|failure| failure.to_string())?;
    thread::spawn(move || {
        let started = stage::close(&app).and_then(|()| app.state::<Studio>().start(&app, &order));
        // There is nothing to tell if a person has changed their mind.
        if let Some(text) = started
            .err()
            .map(|failure| failure.to_string())
            .filter(|text| !text.is_empty())
        {
            tell(&app, &text);
        }
    });
    Ok(())
}

/// The order for filming on the monitor where the window is. Remembers what to record.
fn order(app: &AppHandle, window: &WebviewWindow, wish: &Wish) -> Result<Order, Failure> {
    let file = files::settings(app)?;
    file.write(&file.read()?.with_ears(Listened {
        sound: wish.sound,
        voice: wish.voice,
    }))?;
    let monitor = window.current_monitor()?.ok_or("There is no monitor")?;
    let size = (monitor.size().width, monitor.size().height);
    Ok(Order {
        monitor: Area {
            left: monitor.position().x,
            top: monitor.position().y,
            width: size.0,
            height: size.1,
        },
        scale: monitor.scale_factor(),
        area: wish.area.scaled(wish.sheet, size),
        sound: wish.sound,
        voice: wish.voice,
        file: files::films(app)?.vacant(KIND, Local::now())?,
    })
}

/// How filming is going.
#[tauri::command]
pub fn progress(app: AppHandle, studio: State<'_, Studio>) -> Progress {
    let sound = files::settings(&app)
        .and_then(|file| file.read())
        .is_ok_and(|settings| settings.ears.sound);
    studio.progress(sound && !cfg!(target_os = "macos"))
}

/// Stops filming.
#[tauri::command]
pub fn cease(studio: State<'_, Studio>) {
    studio.stop();
}

/// Lets a person drag the window.
#[tauri::command]
pub fn drag(window: WebviewWindow) -> Result<(), String> {
    window
        .start_dragging()
        .map_err(|failure| failure.to_string())
}

/// A language as the editor sees it. An empty code means the language of the system.
#[derive(Serialize)]
pub struct Tongue {
    code: &'static str,
    title: String,
    on: bool,
}

/// Languages to choose from.
#[tauri::command]
pub fn languages(app: AppHandle) -> Result<Vec<Tongue>, String> {
    let chosen = files::settings(&app)
        .and_then(|file| file.read())
        .map_err(|failure| failure.to_string())?
        .language;
    let system = Tongue {
        code: "",
        title: Phrase::new("menu.system").to_string(),
        on: chosen.is_empty(),
    };
    Ok(std::iter::once(system)
        .chain(LANGUAGES.iter().map(|language| Tongue {
            code: language.code,
            title: language.text("language"),
            on: language.code == chosen,
        }))
        .collect())
}

/// Makes the whole program speak the language. Returns its words.
#[tauri::command]
pub fn speak(app: AppHandle, code: String) -> Result<HashMap<String, String>, String> {
    tray::speak(&app, &code).map_err(|failure| failure.to_string())?;
    Ok(words())
}

/// Closes the editor on all monitors.
#[tauri::command]
pub fn leave(app: AppHandle) -> Result<(), String> {
    stage::close(&app).map_err(|failure| failure.to_string())
}

/// The server as its window sees it: the key itself never gets there.
#[derive(Serialize)]
pub struct Known {
    address: String,
    token: bool,
}

/// The server known now.
#[tauri::command]
pub fn server(app: AppHandle) -> Result<Known, String> {
    files::server(&app)
        .and_then(|file| file.read())
        .map(|server| Known {
            address: server.address,
            token: !server.token.is_empty(),
        })
        .map_err(|failure| failure.to_string())
}

/// Remembers the server. An empty key keeps the old one, an empty address turns uploading off.
#[tauri::command]
pub fn appoint(app: AppHandle, address: String, token: String) -> Result<(), String> {
    let appointed = || -> Result<(), Failure> {
        let file = files::server(&app)?;
        let before = file.read()?;
        let address = address.trim().to_owned();
        let token = token.trim().to_owned();
        file.write(&if address.is_empty() {
            Server::default()
        } else if token.is_empty() {
            Server {
                address,
                token: before.token,
            }
        } else {
            Server { address, token }
        })
    };
    appointed().map_err(|failure| failure.to_string())
}
