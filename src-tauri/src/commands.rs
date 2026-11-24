//! Commands that windows of the program call.

use std::collections::HashMap;

use chrono::Local;
use screenshotter_phrase::{Language, Phrase, LANGUAGES};
use screenshotter_store::Failure;
use screenshotter_upload::Server;
use serde::Serialize;
use tauri::ipc::{InvokeBody, Request, Response};
use tauri::{AppHandle, State, WebviewWindow};

use crate::album::Album;
use crate::{clipboard, files, stage, tray};

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
