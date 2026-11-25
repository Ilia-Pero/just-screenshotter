//! Spelling mistakes in texts a person types. The system finds them: the program has no dictionaries
//! of its own and sends nothing anywhere.

use std::error::Error;

#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "windows")]
mod profile;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub use mac::Spelling;
#[cfg(target_os = "windows")]
pub use windows::Spelling;

/// A failure to ask the system. It travels between threads, hence `Send + Sync`.
pub type Failure = Box<dyn Error + Send + Sync>;
