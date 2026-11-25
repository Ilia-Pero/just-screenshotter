//! Keys that start actions on X11. The usual library asks the server twenty times a second whether
//! a key is pressed; here a thread sleeps until the server tells it.

use std::error::Error;

#[cfg(target_os = "linux")]
mod grab;
mod key;

#[cfg(target_os = "linux")]
pub use grab::Grab;
pub use key::Key;

/// A failure to take keys. It travels between threads, hence `Send + Sync`.
pub type Failure = Box<dyn Error + Send + Sync>;
