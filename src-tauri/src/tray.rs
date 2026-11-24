//! The tray icon and its menu.

use std::thread;

use screenshotter_phrase::{Language, Phrase, LANGUAGES};
use screenshotter_store::{Failure, Settings};
use tauri::menu::{CheckMenuItem, IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, Wry};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

use crate::files::{folder, settings};
use crate::{hotkey, snap, tell};

const TRAY: &str = "tray";

/// The label of the window with the server address.
const SERVER: &str = "server";

/// How names of menu items with keys begin.
const HOTKEY: &str = "hotkey:";

/// How names of menu items with languages begin.
const LANGUAGE: &str = "language:";

/// Puts the icon into the tray.
pub fn install(app: &AppHandle, chosen: &Settings) -> Result<(), Failure> {
    TrayIconBuilder::with_id(TRAY)
        .icon(
            app.default_window_icon()
                .cloned()
                .ok_or("The build has no icon")?,
        )
        .tooltip(tooltip(chosen))
        .menu(&menu(app, chosen)?)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            let done = act(app, event.id().as_ref());
            let shown = refresh(app);
            if let Err(failure) = done.and(shown) {
                tell(app, &failure.to_string());
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                snap(tray.app_handle());
            }
        })
        .build(app)?;
    Ok(())
}

/// Shows in the menu what the settings say now.
fn refresh(app: &AppHandle) -> Result<(), Failure> {
    let chosen = settings(app)?.read()?;
    if let Some(tray) = app.tray_by_id(TRAY) {
        tray.set_menu(Some(menu(app, &chosen)?))?;
        tray.set_tooltip(Some(tooltip(&chosen)))?;
    }
    Ok(())
}

/// Does what the menu item says.
fn act(app: &AppHandle, id: &str) -> Result<(), Failure> {
    if let Some(code) = id.strip_prefix(HOTKEY) {
        return assign(app, code);
    }
    if let Some(code) = id.strip_prefix(LANGUAGE) {
        return speak(app, code);
    }
    match id {
        "shoot" => snap(app),
        "folder" => return reveal(app),
        "choose" => return choose(app),
        "server" => appoint(app),
        "autostart" => return toggle(app),
        "help" => tell(app, &said("error.shortcut")),
        "quit" => app.exit(0),
        _ => {}
    }
    Ok(())
}

fn said(key: &'static str) -> String {
    Phrase::new(key).to_string()
}

fn tooltip(chosen: &Settings) -> String {
    if hotkey::assignable() {
        format!("{}: {}", said("name"), hotkey::title(&chosen.hotkey))
    } else {
        said("name")
    }
}

fn menu(app: &AppHandle, chosen: &Settings) -> Result<Menu<Wry>, Failure> {
    let autostart = check(
        app,
        "autostart",
        &said("menu.autostart"),
        app.autolaunch().is_enabled().unwrap_or(false),
    )?;
    Ok(Menu::with_items(
        app,
        &[
            &item(app, "shoot", "menu.shoot")?,
            &item(app, "folder", "menu.folder")?,
            &PredefinedMenuItem::separator(app)?,
            keys(app, chosen)?.as_ref(),
            &item(app, "choose", "menu.choose")?,
            &item(app, "server", "menu.server")?,
            &languages(app, chosen)?,
            &autostart,
            &PredefinedMenuItem::separator(app)?,
            &item(app, "quit", "menu.quit")?,
        ],
    )?)
}

fn item(app: &AppHandle, id: &str, key: &'static str) -> Result<MenuItem<Wry>, Failure> {
    Ok(MenuItem::with_id(app, id, said(key), true, None::<&str>)?)
}

fn check(app: &AppHandle, id: &str, title: &str, on: bool) -> Result<CheckMenuItem<Wry>, Failure> {
    Ok(CheckMenuItem::with_id(
        app,
        id,
        title,
        true,
        on,
        None::<&str>,
    )?)
}

fn submenu(
    app: &AppHandle,
    key: &'static str,
    items: &[CheckMenuItem<Wry>],
) -> Result<Submenu<Wry>, Failure> {
    let items: Vec<&dyn IsMenuItem<Wry>> = items
        .iter()
        .map(|item| {
            let item: &dyn IsMenuItem<Wry> = item;
            item
        })
        .collect();
    Ok(Submenu::with_items(app, said(key), true, &items)?)
}

/// Items with keys. A key written into the settings file by hand goes to the end of the list.
fn keys(app: &AppHandle, chosen: &Settings) -> Result<Box<dyn IsMenuItem<Wry>>, Failure> {
    if !hotkey::assignable() {
        return Ok(Box::new(item(app, "help", "menu.help")?));
    }
    let own = hotkey::CHOICES
        .iter()
        .all(|(code, _)| *code != chosen.hotkey)
        .then_some((chosen.hotkey.as_str(), chosen.hotkey.as_str()));
    let items = hotkey::CHOICES
        .iter()
        .copied()
        .chain(own)
        .map(|(code, title)| {
            check(
                app,
                &format!("{HOTKEY}{code}"),
                title,
                code == chosen.hotkey,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Box::new(submenu(app, "menu.key", &items)?))
}

/// Items with languages, each in its own language.
fn languages(app: &AppHandle, chosen: &Settings) -> Result<Submenu<Wry>, Failure> {
    let system = check(
        app,
        LANGUAGE,
        &said("menu.system"),
        chosen.language.is_empty(),
    );
    let items = std::iter::once(system)
        .chain(LANGUAGES.iter().map(|language| {
            check(
                app,
                &format!("{LANGUAGE}{}", language.code),
                &language.text("language"),
                language.code == chosen.language,
            )
        }))
        .collect::<Result<Vec<_>, _>>()?;
    submenu(app, "menu.language", &items)
}

/// Opens the window with the server address. In a thread of its own: on Windows a window
/// cannot be created from an event handler.
fn appoint(app: &AppHandle) {
    let app = app.clone();
    thread::spawn(move || {
        let opened = match app.get_webview_window(SERVER) {
            Some(window) => window.set_focus(),
            None => WebviewWindowBuilder::new(&app, SERVER, WebviewUrl::App("server.html".into()))
                .title(said("server.title"))
                .inner_size(460.0, 400.0)
                .resizable(false)
                .center()
                .build()
                .map(|_| ()),
        };
        if let Err(failure) = opened {
            tell(&app, &failure.to_string());
        }
    });
}

fn reveal(app: &AppHandle) -> Result<(), Failure> {
    let folder = folder(app)?;
    app.opener()
        .open_path(folder.path()?.to_string_lossy(), None::<&str>)?;
    Ok(())
}

fn choose(app: &AppHandle) -> Result<(), Failure> {
    let file = settings(app)?;
    let chosen = file.read()?;
    let after = app.clone();
    app.dialog()
        .file()
        .set_title(said("menu.choose"))
        .set_directory(&chosen.folder)
        .pick_folder(move |picked| {
            let written = picked.map_or(Ok(()), |path| {
                file.write(&chosen.with_folder(path.into_path()?))
            });
            if let Err(failure) = written {
                tell(&after, &failure.to_string());
            }
        });
    Ok(())
}

fn toggle(app: &AppHandle) -> Result<(), Failure> {
    let launch = app.autolaunch();
    if launch.is_enabled()? {
        launch.disable()?;
    } else {
        launch.enable()?;
    }
    Ok(())
}

fn assign(app: &AppHandle, code: &str) -> Result<(), Failure> {
    let file = settings(app)?;
    let before = file.read()?;
    hotkey::replace(app, &before.hotkey, code)?;
    file.write(&before.with_hotkey(code))
}

/// Makes the whole program speak the language with the code. An empty code returns the choice
/// to the system.
pub fn speak(app: &AppHandle, code: &str) -> Result<(), Failure> {
    let file = settings(app)?;
    file.write(&file.read()?.with_language(code))?;
    Language::choose(code);
    refresh(app)
}
