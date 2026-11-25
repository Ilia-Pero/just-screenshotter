//! The screen the picture is taken from.
//!
//! The crate does not depend on Tauri, so the compiler checks its macOS and Linux code on any
//! machine: `cargo clippy -p screenshotter-capture --target x86_64-unknown-linux-gnu`.

use std::error::Error;

#[cfg(target_os = "windows")]
mod front;
#[cfg(target_os = "windows")]
mod gdi;
mod patch;
mod pixels;
#[cfg(target_os = "linux")]
mod portal;
#[cfg(target_os = "macos")]
mod quartz;

#[cfg(target_os = "windows")]
pub use front::front;
#[cfg(target_os = "windows")]
pub use gdi::Gdi as Native;
#[cfg(target_os = "linux")]
pub use portal::Portal as Native;
#[cfg(target_os = "macos")]
pub use quartz::Quartz as Native;

pub use patch::Patch;
pub use pixels::{Picture, Pixels};

/// A failure to take the picture. It travels between threads, hence `Send + Sync`.
pub type Failure = Box<dyn Error + Send + Sync>;

/// Where the window a person works with is. Only Windows tells it.
#[cfg(not(target_os = "windows"))]
#[must_use]
pub fn front() -> Patch {
    Patch::default()
}

/// A screen that gives away pictures of its rectangular areas.
pub trait Screen {
    /// One PNG for each area, in the same order.
    ///
    /// # Errors
    ///
    /// The system has not let the program see the screen, or has given something else.
    fn pngs(&self, patches: &[Patch]) -> Result<Vec<Vec<u8>>, Failure>;
}
