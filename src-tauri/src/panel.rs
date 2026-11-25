//! The window with the settings, and the commands it calls.

use std::path::PathBuf;
use std::thread;

use screenshotter_mouse::available;
use screenshotter_phrase::Phrase;
use screenshotter_store::{Failure, Listened, Trigger, Triggers};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

use crate::act::Act;
use crate::commands::Listens;
use crate::mice::Mice;
use crate::{files, hotkey, stage, tell, tray};

/// The label of the window.
const PANEL: &str = "panel";

/// The size of the window, in logical pixels.
const SIZE: (f64, f64) = (620.0, 660.0);

/// Opens the window. In a thread of its own: on Windows a window cannot be created from an event
/// handler.
pub fn open(app: &AppHandle) {
    let app = app.clone();
    thread::spawn(move || {
        let opened = match app.get_webview_window(PANEL) {
            Some(window) => window.set_focus(),
            None => stage::observed(
                WebviewWindowBuilder::new(&app, PANEL, WebviewUrl::App("panel.html".into()))
                    .title(Phrase::new("settings").to_string())
                    .inner_size(SIZE.0, SIZE.1)
                    .min_inner_size(460.0, 360.0)
                    .center(),
            )
            .build()
            .map(|_| ()),
        };
        if let Err(failure) = opened {
            tell(&app, &failure.to_string());
        }
    });
}

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
    let shortcuts = app.global_shortcut();
    let assignable = hotkey::assignable();
    let acts = Act::ALL
        .into_iter()
        .zip(settings.triggers.all())
        .filter(|(act, _)| act.possible())
        .map(|(act, trigger)| Row {
            code: act.code(),
            key: trigger.key.clone(),
            title: trigger.title(cfg!(target_os = "macos")),
            mouse: trigger.mouse.clone(),
            busy: assignable
                && !trigger.key.is_empty()
                && !shortcuts.is_registered(trigger.key.as_str()),
        })
        .collect();
    Ok(Panel {
        acts,
        assignable,
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
pub fn arrange(
    app: AppHandle,
    mice: State<'_, Mice>,
    chosen: Vec<Chosen>,
) -> Result<Panel, String> {
    let arranged = rearranged(&app, &chosen);
    let triggers = files::settings(&app).and_then(|file| file.read());
    let resumed = triggers.and_then(|settings| {
        hotkey::assign(&app, &settings.triggers);
        mice.follow(&app, &settings.triggers)
    });
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
    let busy = hotkey::assign(app, &triggers);
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
pub fn hold(app: AppHandle, mice: State<'_, Mice>, on: bool) -> Result<(), String> {
    if on {
        hotkey::release(&app);
        mice.rest();
        return Ok(());
    }
    files::settings(&app)
        .and_then(|file| file.read())
        .and_then(|settings| {
            hotkey::assign(&app, &settings.triggers);
            mice.follow(&app, &settings.triggers)
        })
        .map_err(|failure| failure.to_string())
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
