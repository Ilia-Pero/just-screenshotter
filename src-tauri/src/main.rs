//! A screenshot tool. At rest it is one process with a tray icon: no windows, no timers.
//! The editor window is created when a screenshot is taken and destroyed when it is not needed.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod act;
mod album;
mod clipboard;
mod commands;
mod files;
mod hotkey;
mod mice;
mod panel;
mod stage;
mod studio;
mod tray;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process::ExitCode;

use screenshotter_film::Native;
use screenshotter_phrase::{Language, Phrase};
use screenshotter_store::{Failure, Settings};
use tauri::{AppHandle, Builder, Manager, RunEvent, Wry};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::act::Act;
use crate::album::Album;
use crate::mice::Mice;
use crate::stage::Stage;
use crate::studio::Studio;

/// Shows a person what has gone wrong.
fn tell(app: &AppHandle, text: &str) {
    app.dialog()
        .message(text)
        .title(Phrase::new("name").to_string())
        .kind(MessageDialogKind::Error)
        .show(|_| {});
}

/// Gets the program ready: reads the settings, installs the icon, assigns keys and buttons.
fn setup(app: &AppHandle) -> Result<(), Failure> {
    let chosen = match files::settings(app)?.read() {
        Ok(chosen) => {
            if let Err(failure) = settle(app, &chosen) {
                tell(app, &failure.to_string());
            }
            chosen
        }
        Err(failure) => {
            tell(
                app,
                &Phrase::new("error.settings")
                    .with("reason", failure)
                    .to_string(),
            );
            files::initial(app)?
        }
    };
    Language::choose(&chosen.language);
    tray::install(app, &chosen)?;
    // The other keys are extra: if they are taken, the window with the settings shows it.
    if hotkey::assign(app, &chosen.triggers).contains(&Act::Area) {
        let title = chosen.triggers.area.title(cfg!(target_os = "macos"));
        tell(
            app,
            &Phrase::new("error.key").with("key", title).to_string(),
        );
    }
    app.state::<Mice>().follow(app, &chosen.triggers)
}

/// Makes the program start with the system. It does so once, and then a person decides.
fn settle(app: &AppHandle, chosen: &Settings) -> Result<(), Failure> {
    if chosen.settled || cfg!(debug_assertions) {
        return Ok(());
    }
    app.autolaunch().enable()?;
    files::settings(app)?.write(&chosen.settled())
}

/// The plugin for the clipboard. On Windows the program does without it, see `clipboard.rs`.
#[cfg(target_os = "windows")]
fn with_clipboard(builder: Builder<Wry>) -> Builder<Wry> {
    builder
}

/// The plugin for the clipboard. On Windows the program does without it, see `clipboard.rs`.
#[cfg(not(target_os = "windows"))]
fn with_clipboard(builder: Builder<Wry>) -> Builder<Wry> {
    builder.plugin(tauri_plugin_clipboard_manager::init())
}

fn main() -> ExitCode {
    let arguments: Vec<OsString> = env::args_os().skip(1).collect();
    if let Some(code) = screenshotter_film::called(&arguments, &Native, Language::choose) {
        return code;
    }
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            eprintln!("{failure}");
            ExitCode::FAILURE
        }
    }
}

/// Runs the program until a person quits.
fn run() -> Result<(), Box<dyn Error>> {
    with_clipboard(Builder::default())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            Act::Area.start(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(Album::default())
        .manage(Mice::default())
        .manage(Stage::default())
        .manage(Studio::default())
        .invoke_handler(tauri::generate_handler![
            commands::shot,
            commands::opening,
            commands::reveal,
            commands::copy,
            commands::save,
            commands::shares,
            commands::share,
            commands::leave,
            commands::words,
            commands::listens,
            commands::film,
            commands::progress,
            commands::cease,
            commands::drag,
            commands::languages,
            commands::speak,
            commands::server,
            commands::appoint,
            panel::panel,
            panel::arrange,
            panel::hold,
            panel::launch,
            panel::hear,
            panel::relocate,
            panel::folder,
            panel::uploading
        ])
        .setup(|app| {
            // No icon in the Dock: the program lives in the menu bar only.
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            setup(app.handle()).map_err(|failure| -> Box<dyn Error> { failure })
        })
        .build(tauri::generate_context!())?
        .run(|_app, event| {
            // The program outlives windows of the editor: it stays in the tray.
            if let RunEvent::ExitRequested {
                api, code: None, ..
            } = event
            {
                api.prevent_exit();
            }
        });
    Ok(())
}
