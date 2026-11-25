//! Windows of the editor over the monitors.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use screenshotter_capture::{front, Failure, Native, Patch, Screen};
use tauri::{AppHandle, Manager, Monitor, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Wry};

use crate::act::Act;
use crate::album::{Album, Opening};
use crate::spelling;

/// How labels of editor windows begin.
const OVERLAY: &str = "overlay-";

/// How long to wait for a hidden window to draw itself before taking it for a hung one.
const PATIENCE: Duration = Duration::from_secs(10);

/// The stage with windows of the editor, one for each monitor.
#[derive(Default)]
pub struct Stage {
    opened: Mutex<Option<Instant>>,
    serial: AtomicUsize,
}

impl Stage {
    /// Takes a screenshot of all monitors and opens the editor over them, ready for the action.
    /// Does nothing if the editor is already open.
    pub fn open(&self, app: &AppHandle, act: Act) -> Result<(), Failure> {
        let Ok(mut opened) = self.opened.try_lock() else {
            return Ok(());
        };
        let standing = overlays(app);
        let fresh = opened.is_some_and(|moment| moment.elapsed() < PATIENCE);
        let shown = standing
            .iter()
            .any(|window| window.is_visible().unwrap_or(false));
        if !standing.is_empty() && (fresh || shown) {
            return Ok(());
        }
        close(app)?;
        *opened = Some(Instant::now());
        let raised = self.raise(app, act);
        if raised.is_err() {
            close(app)?;
        }
        raised
    }

    /// Creates hidden windows while the screenshot is being taken: both take time, and a window
    /// shows itself only after it has got its screenshot.
    fn raise(&self, app: &AppHandle, act: Act) -> Result<(), Failure> {
        let monitors = app.available_monitors()?;
        let patches: Vec<Patch> = monitors.iter().map(patch).collect();
        let chosen = match act {
            Act::Screen => patch(&under(app)?),
            Act::Window => front(),
            Act::Area | Act::Film => Patch::default(),
        };
        let serial = self.serial.fetch_add(1, Ordering::Relaxed);
        let label = |index: usize| format!("{OVERLAY}{serial}-{index}");
        thread::scope(|scope| {
            let shots = scope.spawn(|| Native.pngs(&patches));
            for (index, monitor) in monitors.iter().enumerate() {
                if staged(app, monitor)? {
                    let opening = opening(&chosen, &patches[index], act);
                    app.state::<Album>().plan(&label(index), opening);
                    overlay(app, &label(index), monitor)?;
                }
            }
            let shots = shots
                .join()
                .map_err(|_| "Taking the screenshot has crashed")??;
            for (index, png) in shots.into_iter().enumerate() {
                app.state::<Album>().keep(&label(index), png);
            }
            Ok(())
        })
    }
}

/// Closes the editor on all monitors and forgets the screenshots.
pub fn close(app: &AppHandle) -> Result<(), Failure> {
    app.state::<Album>().clear();
    for window in overlays(app) {
        window.destroy()?;
    }
    Ok(())
}

fn overlays(app: &AppHandle) -> Vec<WebviewWindow> {
    app.webview_windows()
        .into_iter()
        .filter(|(label, _)| label.starts_with(OVERLAY))
        .map(|(_, window)| window)
        .collect()
}

fn patch(monitor: &Monitor) -> Patch {
    Patch {
        left: monitor.position().x,
        top: monitor.position().y,
        width: monitor.size().width,
        height: monitor.size().height,
    }
}

/// What the window on the monitor begins with.
fn opening(chosen: &Patch, monitor: &Patch, act: Act) -> Opening {
    let seen = chosen.within(monitor);
    Opening {
        left: seen.left,
        top: seen.top,
        width: seen.width,
        height: seen.height,
        film: act == Act::Film,
    }
}

/// The monitor with the cursor.
fn under(app: &AppHandle) -> Result<Monitor, Failure> {
    let cursor = app.cursor_position()?;
    Ok(match app.monitor_from_point(cursor.x, cursor.y)? {
        Some(found) => found,
        None => app.primary_monitor()?.ok_or("There are no monitors")?,
    })
}

/// Whether this monitor needs a window of the editor.
#[cfg(not(target_os = "linux"))]
#[allow(clippy::unnecessary_wraps)]
fn staged(_app: &AppHandle, _monitor: &Monitor) -> Result<bool, Failure> {
    Ok(true)
}

/// Whether this monitor needs a window of the editor: on Linux, only the one with the cursor.
///
/// @todo #1 Wayland does not let a program choose a monitor for its window, so on Linux the
///  editor opens on one monitor, and the desktop chooses it. Check on two monitors whether it is
///  the one with the cursor, and if not, ask the window where it is after it is shown.
#[cfg(target_os = "linux")]
fn staged(app: &AppHandle, monitor: &Monitor) -> Result<bool, Failure> {
    Ok(patch(&under(app)?) == patch(monitor))
}

/// Opens a hidden window of the editor. The editor shows it when the screenshot is drawn.
fn overlay(app: &AppHandle, label: &str, monitor: &Monitor) -> Result<(), Failure> {
    let builder = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title("Just Screenshotter")
        .decorations(false)
        .resizable(false)
        .shadow(false)
        .skip_taskbar(true)
        .always_on_top(true)
        .visible(false);
    let window = observed(builder).build()?;
    spelling::teach(&window)?;
    cover(&window, monitor)
}

/// The window through which a test drives the program without touching the mouse and the keyboard.
/// Debug builds only. The first argument repeats what Tauri sets by itself.
#[cfg(all(debug_assertions, target_os = "windows"))]
pub fn observed(
    builder: WebviewWindowBuilder<'_, Wry, AppHandle>,
) -> WebviewWindowBuilder<'_, Wry, AppHandle> {
    match std::env::var("SCREENSHOTTER_DEBUG_PORT") {
        Ok(port) => builder.additional_browser_args(&format!(
            "--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection \
             --remote-debugging-port={port}"
        )),
        Err(_) => builder,
    }
}

/// The window as it is: tests drive debug builds only.
#[cfg(not(all(debug_assertions, target_os = "windows")))]
pub fn observed(
    builder: WebviewWindowBuilder<'_, Wry, AppHandle>,
) -> WebviewWindowBuilder<'_, Wry, AppHandle> {
    builder
}

/// Stretches the window over the monitor.
#[cfg(target_os = "windows")]
fn cover(window: &WebviewWindow, monitor: &Monitor) -> Result<(), Failure> {
    window.set_position(*monitor.position())?;
    window.set_size(*monitor.size())?;
    Ok(())
}

/// Stretches the window over the monitor, the menu bar and the Dock.
#[cfg(target_os = "macos")]
fn cover(window: &WebviewWindow, monitor: &Monitor) -> Result<(), Failure> {
    window.set_position(*monitor.position())?;
    window.set_size(*monitor.size())?;
    window.set_simple_fullscreen(true)?;
    Ok(())
}

/// Stretches the window over the monitor the desktop has put it on.
#[cfg(target_os = "linux")]
fn cover(window: &WebviewWindow, _monitor: &Monitor) -> Result<(), Failure> {
    window.set_fullscreen(true)?;
    Ok(())
}
