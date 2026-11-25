//! What a person can start.

use std::thread;

use tauri::{AppHandle, Manager};

use crate::stage::Stage;
use crate::studio::Studio;
use crate::tell;

/// An action, in the order of triggers in the settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    /// A screenshot of an area.
    Area,
    /// A screenshot of the whole screen.
    Screen,
    /// A screenshot of the active window.
    Window,
    /// A film of an area.
    Film,
}

impl Act {
    /// All the actions.
    pub const ALL: [Act; 4] = [Act::Area, Act::Screen, Act::Window, Act::Film];

    /// The name of the action for windows of the program.
    pub fn code(self) -> &'static str {
        match self {
            Act::Area => "area",
            Act::Screen => "screen",
            Act::Window => "window",
            Act::Film => "film",
        }
    }

    /// Whether the program can do it in this system.
    ///
    /// @todo #1 Only Windows tells where the active window is. On macOS `CGWindowListCopyWindowInfo`
    ///  knows it, on X11 `_NET_ACTIVE_WINDOW` does. Wayland does not tell it to programs.
    pub fn possible(self) -> bool {
        self != Act::Window || cfg!(target_os = "windows")
    }

    /// Starts the action in a thread of its own: on Windows a window cannot be created from an
    /// event handler. If a film is being made, stops it instead.
    pub fn start(self, app: &AppHandle) {
        let app = app.clone();
        thread::spawn(move || {
            if app.state::<Studio>().stop(&app) {
                return;
            }
            if let Err(failure) = app.state::<Stage>().open(&app, self) {
                tell(&app, &failure.to_string());
            }
        });
    }
}
