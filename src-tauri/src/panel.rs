//! The window with the settings, and the commands it calls.

use std::path::PathBuf;
use std::thread;

use screenshotter_mouse::available;
use screenshotter_phrase::Phrase;
use screenshotter_release::Latest;
use screenshotter_store::{Failure, Listened, Trigger, Triggers};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

use crate::act::Act;
use crate::commands::Listens;
use crate::hotkey::Keyboard;
use crate::mice::Mice;
use crate::{files, hotkey, stage, tell, tray};

/// The label of the window.
const PANEL: &str = "panel";

/// The size of the window, in logical pixels.
const SIZE: (f64, f64) = (720.0, 700.0);

/// Opens the window. In a thread of its own: on Windows a window cannot be created from an event
/// handler.
pub fn open(app: &AppHandle) {
    let app = app.clone();
    thread::spawn(move || {
        let opened = match app.get_webview_window(PANEL) {
            Some(window) => stage::raise(&window),
            None => created(&app),
        };
        if let Err(failure) = opened {
            tell(&app, &failure.to_string());
        }
    });
}

/// Creates the window. It may be closed while it waits for keys, so then they are taken back.
fn created(app: &AppHandle) -> tauri::Result<()> {
    let window = stage::observed(
        WebviewWindowBuilder::new(app, PANEL, WebviewUrl::App("panel.html".into()))
            .title(Phrase::new("settings").to_string())
            .inner_size(SIZE.0, SIZE.1)
            .min_inner_size(420.0, 360.0)
            .center(),
    )
    .build()?;
    let app = app.clone();
    window.on_window_event(move |event| {
        if matches!(event, WindowEvent::Destroyed) {
            let app = app.clone();
            thread::spawn(move || {
                if let Err(failure) = resume(&app) {
                    tell(&app, &failure.to_string());
                }
            });
        }
    });
    Ok(())
}

/// Takes the keys and the buttons the settings name.
fn resume(app: &AppHandle) -> Result<(), Failure> {
    let triggers = files::settings(app)?.read()?.triggers;
    app.state::<Keyboard>().assign(app, &triggers);
    app.state::<Mice>().follow(app, &triggers)
}

/// The page of the program on GitHub.
const HOME: &str = env!("CARGO_PKG_REPOSITORY");

/// The installer for this system.
const INSTALLER: &str = if cfg!(target_os = "windows") {
    "just-screenshotter-windows.exe"
} else if cfg!(target_os = "macos") {
    "just-screenshotter-macos.dmg"
} else {
    "just-screenshotter-ubuntu.deb"
};

/// An action and what starts it.
#[derive(Serialize)]
struct Row {
    code: &'static str,
    key: String,
    /// The key for a person.
    title: String,
    mouse: String,
    /// Whether another program uses the key.
    busy: bool,
}

/// Everything the window shows.
#[derive(Serialize)]
pub struct Panel {
    acts: Vec<Row>,
    /// Whether the program can assign keys by itself.
    assignable: bool,
    /// Whether buttons of the mouse can start actions.
    mice: bool,
    folder: PathBuf,
    ears: Listens,
    /// Whether the program starts with the system.
    launch: bool,
    version: String,
}

/// What a person has chosen for an action.
#[derive(Deserialize)]
pub struct Chosen {
    code: String,
    key: String,
    mouse: String,
}

fn state(app: &AppHandle) -> Result<Panel, Failure> {
    let settings = files::settings(app)?.read()?;
    let keyboard = app.state::<Keyboard>();
    let acts = Act::ALL
        .into_iter()
        .zip(settings.triggers.all())
        .filter(|(act, _)| act.possible())
        .map(|(act, trigger)| Row {
            code: act.code(),
            key: trigger.key.clone(),
            title: trigger.title(cfg!(target_os = "macos")),
            mouse: trigger.mouse.clone(),
            busy: keyboard.busy(act),
        })
        .collect();
    Ok(Panel {
        acts,
        assignable: hotkey::assignable(),
        mice: available(),
        folder: settings.folder,
        ears: Listens::to(settings.ears),
        launch: app.autolaunch().is_enabled().unwrap_or(false),
        version: app.package_info().version.to_string(),
    })
}

/// What the window shows after something is done.
fn after(app: &AppHandle, done: Result<(), Failure>) -> Result<Panel, String> {
    done.and_then(|()| state(app))
        .map_err(|failure| failure.to_string())
}

/// What the window shows.
#[tauri::command]
pub fn panel(app: AppHandle) -> Result<Panel, String> {
    after(&app, Ok(()))
}

/// Remembers what starts actions. If another program uses a key, nothing changes.
#[tauri::command(async)]
pub fn arrange(app: AppHandle, chosen: Vec<Chosen>) -> Result<Panel, String> {
    let arranged = rearranged(&app, &chosen);
    let resumed = resume(&app);
    let shown = tray::refresh(&app);
    after(&app, arranged.and(resumed).and(shown))
}

/// Writes down what starts actions, if nothing is in the way.
fn rearranged(app: &AppHandle, chosen: &[Chosen]) -> Result<(), Failure> {
    let stored = files::settings(app)?;
    let before = stored.read()?;
    let trigger = |code: &str, old: &Trigger| {
        chosen
            .iter()
            .find(|one| one.code == code)
            .map_or(old.clone(), |one| Trigger {
                key: one.key.clone(),
                mouse: one.mouse.clone(),
            })
    };
    let [area, screen, window, film] = before.triggers.all();
    let triggers = Triggers {
        area: trigger(Act::Area.code(), area),
        screen: trigger(Act::Screen.code(), screen),
        window: trigger(Act::Window.code(), window),
        film: trigger(Act::Film.code(), film),
    };
    if triggers.clash() {
        return Err(Phrase::new("error.clash").into());
    }
    let busy = app.state::<Keyboard>().assign(app, &triggers);
    let taken = Act::ALL
        .into_iter()
        .zip(triggers.all().into_iter().zip(before.triggers.all()))
        .find(|(act, (new, old))| busy.contains(act) && new.key != old.key);
    if let Some((_, (new, _))) = taken {
        return Err(Phrase::new("error.busy")
            .with("key", new.title(cfg!(target_os = "macos")))
            .into());
    }
    stored.write(&before.with_triggers(triggers))
}

/// Lets keys and buttons go while a person presses the ones they want, and takes them back.
#[tauri::command(async)]
pub fn hold(app: AppHandle, on: bool) -> Result<(), String> {
    if on {
        app.state::<Keyboard>().release(&app);
        app.state::<Mice>().rest();
        return Ok(());
    }
    resume(&app).map_err(|failure| failure.to_string())
}

/// Starts the program with the system, or does not any more.
#[tauri::command]
pub fn launch(app: AppHandle, on: bool) -> Result<Panel, String> {
    let launch = app.autolaunch();
    let done = if on {
        launch.enable()
    } else {
        launch.disable()
    };
    after(&app, done.map_err(Failure::from))
}

/// Remembers what to record together with video.
#[tauri::command]
pub fn hear(app: AppHandle, sound: bool, voice: bool) -> Result<Panel, String> {
    let written = files::settings(&app)
        .and_then(|file| file.write(&file.read()?.with_ears(Listened { sound, voice })));
    after(&app, written)
}

/// Asks a person for another folder. Nothing changes if they do not choose.
#[tauri::command(async)]
pub fn relocate(app: AppHandle) -> Result<Panel, String> {
    let moved = || -> Result<(), Failure> {
        let file = files::settings(&app)?;
        let chosen = file.read()?;
        let picked = app
            .dialog()
            .file()
            .set_title(Phrase::new("menu.choose").to_string())
            .set_directory(&chosen.folder)
            .blocking_pick_folder();
        match picked {
            Some(path) => file.write(&file.read()?.with_folder(path.into_path()?)),
            None => Ok(()),
        }
    };
    after(&app, moved())
}

/// Shows the folder.
#[tauri::command]
pub fn folder(app: AppHandle) -> Result<(), String> {
    tray::reveal(&app).map_err(|failure| failure.to_string())
}

/// Opens the window with the server address.
#[tauri::command]
pub fn uploading(app: AppHandle) {
    tray::appoint(&app);
}

/// The latest release.
#[derive(Serialize)]
pub struct Release {
    version: String,
    /// Whether it is newer than this program.
    newer: bool,
}

/// Looks for a newer version. The program does this only when a person asks.
#[tauri::command(async)]
pub fn update(app: AppHandle) -> Result<Release, String> {
    released(&app).map_err(|failure| {
        Phrase::new("error.update")
            .with("reason", failure)
            .to_string()
    })
}

fn released(app: &AppHandle) -> Result<Release, Failure> {
    let latest = Latest::new(HOME).version()?;
    Ok(Release {
        version: latest.to_string(),
        newer: latest > app.package_info().version.to_string().parse()?,
    })
}

/// Downloads the installer of the latest release with the browser.
#[tauri::command]
pub fn download(app: AppHandle) -> Result<(), String> {
    browse(&app, Latest::new(HOME).file(INSTALLER))
}

/// Shows what is new in the latest release.
#[tauri::command]
pub fn news(app: AppHandle) -> Result<(), String> {
    browse(&app, Latest::new(HOME).page())
}

fn browse(app: &AppHandle, url: String) -> Result<(), String> {
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|failure| failure.to_string())
}
