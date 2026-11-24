//! The key that takes a screenshot.

use screenshotter_phrase::Phrase;
use screenshotter_store::Failure;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

/// Keys offered in the menu: the spelling of Tauri and the title for a person.
#[cfg(target_os = "macos")]
pub const CHOICES: &[(&str, &str)] = &[
    ("Cmd+Shift+9", "⌘⇧9"),
    ("Cmd+Shift+2", "⌘⇧2"),
    ("Ctrl+Shift+S", "⌃⇧S"),
    ("F9", "F9"),
];

/// Keys offered in the menu: the spelling of Tauri and the title for a person.
#[cfg(not(target_os = "macos"))]
pub const CHOICES: &[(&str, &str)] = &[
    ("PrintScreen", "Print Screen"),
    ("Ctrl+PrintScreen", "Ctrl + Print Screen"),
    ("Shift+PrintScreen", "Shift + Print Screen"),
    ("Ctrl+Shift+S", "Ctrl + Shift + S"),
    ("Ctrl+Shift+X", "Ctrl + Shift + X"),
    ("F9", "F9"),
];

/// The key until another one is chosen.
pub const INITIAL: &str = CHOICES[0].0;

/// Whether the program can assign the key by itself. It cannot on Wayland.
///
/// @todo #1 On Wayland a person assigns the key in the system settings: the program takes
///  a screenshot when it is started for the second time. Assign the key from the program
///  through the portal `org.freedesktop.portal.GlobalShortcuts` where there is one.
pub fn assignable() -> bool {
    !(cfg!(target_os = "linux") && std::env::var_os("WAYLAND_DISPLAY").is_some())
}

/// The title of the key for a person.
pub fn title(hotkey: &str) -> &str {
    CHOICES
        .iter()
        .find(|(code, _)| *code == hotkey)
        .map_or(hotkey, |(_, title)| title)
}

/// Assigns the key instead of the old one. If it is taken, the old one comes back.
pub fn replace(app: &AppHandle, before: &str, after: &str) -> Result<(), Failure> {
    let shortcuts = app.global_shortcut();
    shortcuts.unregister_all()?;
    if shortcuts.register(after).is_ok() {
        return Ok(());
    }
    shortcuts.register(before)?;
    Err(Phrase::new("error.kept")
        .with("key", title(after))
        .with("kept", title(before))
        .into())
}
