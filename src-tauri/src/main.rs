//! A screenshot tool. At rest it is one process with a tray icon: no windows, no timers.
//! The editor window is created when a screenshot is taken and destroyed when it is not needed.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod album;
mod clipboard;
mod commands;
mod files;
mod hotkey;
mod stage;
mod tray;

use std::error::Error;
use std::thread;

use screenshotter_phrase::{Language, Phrase};
use screenshotter_store::Failure;
use tauri::{AppHandle, Builder, Manager, RunEvent, Wry};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::album::Album;
use crate::stage::Stage;

/// Shows a person what has gone wrong.
fn tell(app: &AppHandle, text: &str) {
    app.dialog()
        .message(text)
        .title(Phrase::new("name").to_string())
        .kind(MessageDialogKind::Error)
        .show(|_| {});
}

/// Takes a screenshot in a thread of its own: on Windows a window cannot be created
/// from an event handler.
fn snap(app: &AppHandle) {
    let app = app.clone();
    thread::spawn(move || {
        if let Err(failure) = app.state::<Stage>().open(&app) {
            tell(&app, &failure.to_string());
        }
    });
}

/// Gets the program ready: reads the settings, installs the icon, assigns the key.
fn setup(app: &AppHandle) -> Result<(), Failure> {
    let chosen = match files::settings(app)?.read() {
        Ok(chosen) => chosen,
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
    let title = hotkey::title(&chosen.hotkey);
    if hotkey::assignable()
        && app
            .global_shortcut()
            .register(chosen.hotkey.as_str())
            .is_err()
    {
        tell(
            app,
            &Phrase::new("error.key").with("key", title).to_string(),
        );
    }
    Ok(())
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

fn main() -> Result<(), Box<dyn Error>> {
    with_clipboard(Builder::default())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            snap(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        snap(app);
                    }
                })
                .build(),
        )
        .manage(Album::default())
        .manage(Stage::default())
        .invoke_handler(tauri::generate_handler![
            commands::shot,
            commands::reveal,
            commands::copy,
            commands::save,
            commands::shares,
            commands::share,
            commands::leave,
            commands::words,
            commands::languages,
            commands::speak,
            commands::server,
            commands::appoint
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
