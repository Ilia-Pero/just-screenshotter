//! The clipboard.

use screenshotter_store::Failure;
use tauri::WebviewWindow;

/// Puts the picture on the Windows clipboard. Everything is prepared in advance, and the
/// clipboard is open for an instant: while it is open, other programs may spoil the copy.
#[cfg(target_os = "windows")]
pub fn put(window: &WebviewWindow, png: &[u8]) -> Result<(), Failure> {
    use clipboard_win::{formats::CF_DIB, raw};
    use screenshotter_capture::Picture;

    let dib = Picture::new(png.to_vec()).pixels()?.dib()?;
    let format = raw::register_format("PNG")
        .ok_or("Windows has no PNG format")?
        .get();
    open(window, || {
        raw::empty()?;
        raw::set_without_clear(format, png)?;
        raw::set_without_clear(CF_DIB, &dib)
    })
}

/// Puts the text on the Windows clipboard.
#[cfg(target_os = "windows")]
pub fn put_text(window: &WebviewWindow, text: &str) -> Result<(), Failure> {
    open(window, || clipboard_win::raw::set_string(text))
}

/// Opens the clipboard and writes to it. If it is busy, tries again for about a second.
#[cfg(target_os = "windows")]
fn open(
    window: &WebviewWindow,
    write: impl Fn() -> clipboard_win::SysResult<()>,
) -> Result<(), Failure> {
    use std::thread;
    use std::time::Duration;

    use screenshotter_phrase::Phrase;

    let owner = window.hwnd()?.0;
    for _ in 0..20 {
        if clipboard_win::Clipboard::new_attempts_for(owner, 5)
            .and_then(|_open| write())
            .is_ok()
        {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }
    Err(Phrase::new("error.clipboard").into())
}

/// Puts the picture on the clipboard.
#[cfg(not(target_os = "windows"))]
pub fn put(window: &WebviewWindow, png: &[u8]) -> Result<(), Failure> {
    use tauri::image::Image;
    use tauri::Manager;
    use tauri_plugin_clipboard_manager::ClipboardExt;

    window
        .app_handle()
        .clipboard()
        .write_image(&Image::from_bytes(png)?)?;
    Ok(())
}

/// Puts the text on the clipboard.
#[cfg(not(target_os = "windows"))]
pub fn put_text(window: &WebviewWindow, text: &str) -> Result<(), Failure> {
    use tauri::Manager;
    use tauri_plugin_clipboard_manager::ClipboardExt;

    window.app_handle().clipboard().write_text(text)?;
    Ok(())
}
