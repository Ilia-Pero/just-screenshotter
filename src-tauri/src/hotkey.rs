//! Keys that start actions.

use screenshotter_store::{Trigger, Triggers};
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::act::Act;

/// What starts actions until a person chooses.
#[cfg(target_os = "macos")]
pub fn initial() -> Triggers {
    Triggers {
        area: Trigger::key("Cmd+Shift+9"),
        film: Trigger::key("Cmd+Shift+8"),
        ..Triggers::default()
    }
}

/// What starts actions until a person chooses.
#[cfg(not(target_os = "macos"))]
pub fn initial() -> Triggers {
    Triggers {
        area: Trigger::key("PrintScreen"),
        screen: Trigger::key("Shift+PrintScreen"),
        window: Trigger::key("Alt+PrintScreen"),
        film: Trigger::key("Ctrl+Shift+PrintScreen"),
    }
}

/// Whether the program can assign keys by itself. It cannot on Wayland.
///
/// @todo #1 On Wayland a person assigns the key in the system settings: the program takes
///  a screenshot when it is started for the second time. Assign the keys from the program
///  through the portal `org.freedesktop.portal.GlobalShortcuts` where there is one.
pub fn assignable() -> bool {
    !(cfg!(target_os = "linux") && std::env::var_os("WAYLAND_DISPLAY").is_some())
}

/// Assigns the keys instead of the old ones. Returns the actions whose keys are taken by other
/// programs.
pub fn assign(app: &AppHandle, triggers: &Triggers) -> Vec<Act> {
    release(app);
    if !assignable() {
        return Vec::new();
    }
    let shortcuts = app.global_shortcut();
    Act::ALL
        .into_iter()
        .zip(triggers.all())
        .filter(|(act, trigger)| act.possible() && !trigger.key.is_empty())
        .filter(|(act, trigger)| {
            let act = *act;
            shortcuts
                .on_shortcut(trigger.key.as_str(), move |app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        act.start(app);
                    }
                })
                .is_err()
        })
        .map(|(act, _)| act)
        .collect()
}

/// Lets the keys go.
pub fn release(app: &AppHandle) {
    // A key that cannot be let go belongs to somebody else already.
    let _ = app.global_shortcut().unregister_all();
}
