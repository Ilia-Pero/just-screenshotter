//! Filming: the process that does it, and the window with the time and the button that stops it.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};
use std::thread;
use std::time::Instant;

use screenshotter_film::{Ears, Ending, Order, Reel};
use screenshotter_phrase::{Language, Phrase};
use screenshotter_store::Failure;
use serde::Serialize;
use tauri::{AppHandle, Manager, PhysicalPosition, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

use crate::{stage, tell, tray};

/// The label of the window with the time.
const WATCH: &str = "watch";

/// The size of that window, in logical pixels.
const SIZE: (f64, f64) = (212.0, 44.0);

/// The distance from the window to the area, in logical pixels.
const GAP: f64 = 10.0;

/// The kind of the files.
#[cfg(target_os = "windows")]
pub const KIND: &str = "mp4";

/// The kind of the files.
#[cfg(target_os = "macos")]
pub const KIND: &str = "mov";

/// The kind of the files.
#[cfg(target_os = "linux")]
pub const KIND: &str = "webm";

/// What is being filmed now.
struct Film {
    reel: Reel,
    ears: Ears,
    began: Instant,
}

/// What the window with the time shows.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Progress {
    /// How long filming has been going on, in milliseconds.
    time: u64,
    /// Whether the sound of the computer was asked for but cannot be heard.
    deaf: bool,
}

/// The place where films are made, one at a time.
#[derive(Default)]
pub struct Studio {
    film: Mutex<Option<Film>>,
}

impl Studio {
    /// Starts filming. Does nothing if a film is already being made.
    pub fn start(&self, app: &AppHandle, order: &Order) -> Result<(), Failure> {
        let mut film = self.film.lock().unwrap_or_else(PoisonError::into_inner);
        if film.is_some() {
            return Ok(());
        }
        let (reel, ending, ears) = Reel::start(order, Language::current().code)?;
        *film = Some(Film {
            reel,
            ears,
            began: Instant::now(),
        });
        drop(film);
        await_ending(app, ending, order.file.clone());
        tray::refresh(app)?;
        watch(app, order)
    }

    /// Stops filming. False if there was nothing to stop.
    pub fn stop(&self) -> bool {
        let film = self
            .film
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        let filmed = film.is_some();
        if let Some(film) = film {
            film.reel.stop();
        }
        filmed
    }

    /// Whether a film is being made.
    pub fn busy(&self) -> bool {
        self.film
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .is_some()
    }

    /// How filming is going.
    pub fn progress(&self, sound: bool) -> Progress {
        self.film
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .as_ref()
            .map_or_else(Progress::default, |film| Progress {
                time: u64::try_from(film.began.elapsed().as_millis()).unwrap_or(u64::MAX),
                deaf: sound && !film.ears.sound,
            })
    }
}

/// Waits in a thread of its own until the file is ready, and shows it.
fn await_ending(app: &AppHandle, ending: Ending, file: PathBuf) {
    let app = app.clone();
    thread::spawn(move || {
        let ended = ending.wait();
        app.state::<Studio>().stop();
        let shown = ended
            .map_err(|reason| Phrase::new("error.film").with("reason", reason).into())
            .and(close(&app))
            .and(tray::refresh(&app))
            .and_then(|()| reveal(&app, &file));
        if let Err(failure) = shown {
            tell(&app, &failure.to_string());
        }
    });
}

/// Shows the file in its folder.
fn reveal(app: &AppHandle, file: &Path) -> Result<(), Failure> {
    app.opener().reveal_item_in_dir(file)?;
    Ok(())
}

fn close(app: &AppHandle) -> Result<(), Failure> {
    if let Some(window) = app.get_webview_window(WATCH) {
        window.destroy()?;
    }
    Ok(())
}

/// Opens the window with the time. Wayland does not let a program choose a place for its window,
/// so on Linux there is none: the menu of the icon and the shortcut stop filming.
fn watch(app: &AppHandle, order: &Order) -> Result<(), Failure> {
    if cfg!(target_os = "linux") {
        return Ok(());
    }
    let builder = WebviewWindowBuilder::new(app, WATCH, WebviewUrl::App("watch.html".into()))
        .title(Phrase::new("film").to_string())
        .inner_size(SIZE.0, SIZE.1)
        .decorations(false)
        .resizable(false)
        .skip_taskbar(true)
        .always_on_top(true)
        // It must not get into the film.
        .content_protected(true)
        .focused(false)
        .visible(false);
    let window = stage::observed(builder).build()?;
    window.set_position(spot(order))?;
    window.show()?;
    Ok(())
}

/// Where the window stands on the desktop.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn spot(order: &Order) -> PhysicalPosition<i32> {
    let physical = |length: f64| (length * order.scale).round() as u32;
    let (left, top) = order.area.beside(
        (order.monitor.width, order.monitor.height),
        (physical(SIZE.0), physical(SIZE.1)),
        physical(GAP),
    );
    PhysicalPosition::new(order.monitor.left + left, order.monitor.top + top)
}
