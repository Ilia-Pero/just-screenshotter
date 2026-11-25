//! Spelling mistakes in texts a person types over a screenshot: the system finds them.

use screenshotter_store::Failure;
use tauri::{AppHandle, WebviewWindow};

/// Gets the system ready, before the first window.
#[cfg(not(target_os = "linux"))]
pub fn wake(app: &AppHandle) -> Result<(), Failure> {
    use tauri::Manager;
    screenshotter_spelling::Spelling::new(app.path().app_local_data_dir()?).wake()
}

/// On Linux every window has to be told separately.
#[cfg(target_os = "linux")]
#[allow(clippy::unnecessary_wraps)]
pub fn wake(_app: &AppHandle) -> Result<(), Failure> {
    Ok(())
}

/// Makes the window find mistakes.
#[cfg(not(target_os = "linux"))]
#[allow(clippy::unnecessary_wraps)]
pub fn teach(_window: &WebviewWindow) -> Result<(), Failure> {
    Ok(())
}

/// Makes the window find mistakes in the languages whose dictionaries are installed.
#[cfg(target_os = "linux")]
pub fn teach(window: &WebviewWindow) -> Result<(), Failure> {
    use webkit2gtk::{WebContextExt, WebViewExt};
    Ok(window.with_webview(|view| {
        if let Some(context) = view.inner().context() {
            context.set_spell_checking_languages(&["ru_RU", "en_US"]);
            context.set_spell_checking_enabled(true);
        }
    })?)
}
