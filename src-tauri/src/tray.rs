//! The tray icon and its menu.

use std::thread;

use screenshotter_phrase::{Language, Phrase};
use screenshotter_store::{Failure, Settings};
use tauri::image::Image;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, Wry};
use tauri_plugin_opener::OpenerExt;

use crate::act::Act;
use crate::files::{folder, settings};
use crate::studio::Studio;
use crate::{hotkey, panel, stage, tell};

const TRAY: &str = "tray";

/// The label of the window with the server address.
const SERVER: &str = "server";

/// Puts the icon into the tray.
pub fn install(app: &AppHandle, chosen: &Settings) -> Result<(), Failure> {
    TrayIconBuilder::with_id(TRAY)
        .icon(
            app.default_window_icon()
                .cloned()
                .ok_or("The build has no icon")?,
        )
        .tooltip(tooltip(chosen))
        .menu(&menu(app)?)
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
                Act::Area.start(tray.app_handle());
            }
        })
        .build(app)?;
    Ok(())
}

/// Shows in the menu and on the icon what is going on now.
pub fn refresh(app: &AppHandle) -> Result<(), Failure> {
    let chosen = settings(app)?.read()?;
    if let Some(tray) = app.tray_by_id(TRAY) {
        let filming = app.state::<Studio>().busy();
        let icon = app
            .default_window_icon()
            .cloned()
            .ok_or("The build has no icon")?;
        tray.set_icon(Some(if filming { marked(&icon) } else { icon }))?;
        tray.set_menu(Some(menu(app)?))?;
        tray.set_tooltip(Some(if filming {
            said("filming")
        } else {
            tooltip(&chosen)
        }))?;
    }
    Ok(())
}

/// The icon with a red spot: a film is being made.
fn marked(icon: &Image<'_>) -> Image<'static> {
    let width = icon.width() as usize;
    let height = icon.height() as usize;
    let radius = width * 5 / 16;
    let mut pixels = icon.rgba().to_vec();
    for (index, pixel) in pixels.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        let across = (index % width.max(1) + radius).abs_diff(width);
        let down = (index / width.max(1) + radius).abs_diff(height);
        if across * across + down * down <= radius * radius {
            *pixel = [229, 51, 42, 255];
        }
    }
    Image::new_owned(pixels, icon.width(), icon.height())
}

/// Does what the menu item says.
fn act(app: &AppHandle, id: &str) -> Result<(), Failure> {
    match id {
        "shoot" => Act::Area.start(app),
        "film" => Act::Film.start(app),
        "folder" => return reveal(app),
        "settings" => panel::open(app),
        "quit" => app.exit(0),
        _ => {}
    }
    Ok(())
}

fn said(key: &'static str) -> String {
    Phrase::new(key).to_string()
}

fn tooltip(chosen: &Settings) -> String {
    if hotkey::assignable() && !chosen.triggers.area.key.is_empty() {
        format!(
            "{}: {}",
            said("name"),
            chosen.triggers.area.title(cfg!(target_os = "macos"))
        )
    } else {
        said("name")
    }
}

fn menu(app: &AppHandle) -> Result<Menu<Wry>, Failure> {
    let first = if app.state::<Studio>().busy() {
        "menu.stop"
    } else {
        "menu.shoot"
    };
    Ok(Menu::with_items(
        app,
        &[
            &item(app, "shoot", first)?,
            &item(app, "film", "film")?,
            &item(app, "folder", "menu.folder")?,
            &PredefinedMenuItem::separator(app)?,
            &item(app, "settings", "menu.settings")?,
            &PredefinedMenuItem::separator(app)?,
            &item(app, "quit", "menu.quit")?,
        ],
    )?)
}

fn item(app: &AppHandle, id: &str, key: &'static str) -> Result<MenuItem<Wry>, Failure> {
    Ok(MenuItem::with_id(app, id, said(key), true, None::<&str>)?)
}

/// Opens the window with the server address. In a thread of its own: on Windows a window
/// cannot be created from an event handler.
pub fn appoint(app: &AppHandle) {
    let app = app.clone();
    thread::spawn(move || {
        let opened = match app.get_webview_window(SERVER) {
            Some(window) => stage::raise(&window),
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

/// Shows the folder with screenshots.
pub fn reveal(app: &AppHandle) -> Result<(), Failure> {
    let folder = folder(app)?;
    app.opener()
        .open_path(folder.path()?.to_string_lossy(), None::<&str>)?;
    Ok(())
}

/// Makes the whole program speak the language with the code. An empty code returns the choice
/// to the system.
pub fn speak(app: &AppHandle, code: &str) -> Result<(), Failure> {
    let file = settings(app)?;
    file.write(&file.read()?.with_language(code))?;
    Language::choose(code);
    refresh(app)
}
