//! Keys that start actions.

use std::sync::{Mutex, MutexGuard, PoisonError};

#[cfg(target_os = "linux")]
use screenshotter_keys::{Grab, Key};
use screenshotter_store::{Trigger, Triggers};
use tauri::{AppHandle, Builder, Wry};
#[cfg(not(target_os = "linux"))]
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

/// The keyboard: the keys the program has taken, and the actions whose keys are taken by other
/// programs.
#[derive(Default)]
pub struct Keyboard {
    busy: Mutex<Vec<Act>>,
    #[cfg(target_os = "linux")]
    grab: Mutex<Option<Grab>>,
}

impl Keyboard {
    /// Assigns the keys instead of the old ones. Returns the actions whose keys are taken by
    /// other programs.
    pub fn assign(&self, app: &AppHandle, triggers: &Triggers) -> Vec<Act> {
        self.release(app);
        if !assignable() {
            return Vec::new();
        }
        let wanted: Vec<(Act, &str)> = Act::ALL
            .into_iter()
            .zip(triggers.all())
            .filter(|(act, trigger)| act.possible() && !trigger.key.is_empty())
            .map(|(act, trigger)| (act, trigger.key.as_str()))
            .collect();
        let busy = self.take(app, &wanted);
        open(&self.busy).clone_from(&busy);
        busy
    }

    /// Whether another program uses the key of the action.
    pub fn busy(&self, act: Act) -> bool {
        open(&self.busy).contains(&act)
    }

    /// Lets the keys go.
    pub fn release(&self, app: &AppHandle) {
        open(&self.busy).clear();
        self.leave(app);
    }

    #[cfg(not(target_os = "linux"))]
    #[allow(clippy::unused_self)]
    fn take(&self, app: &AppHandle, wanted: &[(Act, &str)]) -> Vec<Act> {
        let shortcuts = app.global_shortcut();
        wanted
            .iter()
            .filter(|(act, key)| {
                let act = *act;
                shortcuts
                    .on_shortcut(*key, move |app, _shortcut, event| {
                        if event.state() == ShortcutState::Pressed {
                            act.start(app);
                        }
                    })
                    .is_err()
            })
            .map(|(act, _)| *act)
            .collect()
    }

    #[cfg(not(target_os = "linux"))]
    #[allow(clippy::unused_self)]
    fn leave(&self, app: &AppHandle) {
        // A key that cannot be let go belongs to somebody else already.
        let _ = app.global_shortcut().unregister_all();
    }

    #[cfg(target_os = "linux")]
    fn take(&self, app: &AppHandle, wanted: &[(Act, &str)]) -> Vec<Act> {
        let known: Vec<(Act, Key)> = wanted
            .iter()
            .filter_map(|(act, key)| Key::spelled(key).map(|key| (*act, key)))
            .collect();
        let keys: Vec<Key> = known.iter().map(|(_, key)| *key).collect();
        let acts: Vec<Act> = known.iter().map(|(act, _)| *act).collect();
        let app = app.clone();
        let taken = Grab::start(&keys, {
            let acts = acts.clone();
            move |number| acts[number].start(&app)
        });
        let free: Vec<Act> = match taken {
            Ok((grab, busy)) => {
                *open(&self.grab) = Some(grab);
                (0..acts.len())
                    .filter(|number| !busy.contains(number))
                    .map(|number| acts[number])
                    .collect()
            }
            Err(_) => Vec::new(),
        };
        wanted
            .iter()
            .map(|(act, _)| *act)
            .filter(|act| !free.contains(act))
            .collect()
    }

    #[cfg(target_os = "linux")]
    fn leave(&self, _app: &AppHandle) {
        // Keys are let go when nobody keeps the grab.
        *open(&self.grab) = None;
    }
}

fn open<T>(drawer: &Mutex<T>) -> MutexGuard<'_, T> {
    drawer.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The plugin for keys. On Linux the program does without it, see `keys`.
#[cfg(not(target_os = "linux"))]
pub fn with_keys(builder: Builder<Wry>) -> Builder<Wry> {
    builder.plugin(tauri_plugin_global_shortcut::Builder::new().build())
}

/// The plugin for keys. On Linux the program does without it, see `keys`.
#[cfg(target_os = "linux")]
pub fn with_keys(builder: Builder<Wry>) -> Builder<Wry> {
    builder
}
