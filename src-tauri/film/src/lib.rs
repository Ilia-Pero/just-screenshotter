//! Filming a part of the screen with sound.
//!
//! Filming goes on in a process of its own, see `reel.rs`: when it is over, all the memory it
//! took returns to the system, and a crash of a video driver does not touch the program.

use std::error::Error;
use std::sync::mpsc::{Receiver, Sender};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod mac;
#[cfg(any(test, target_os = "windows"))]
mod mix;
#[cfg(any(test, target_os = "windows"))]
mod nv12;
mod order;
#[cfg(any(test, target_os = "linux"))]
mod pipeline;
mod reel;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::Studio as Native;
#[cfg(target_os = "macos")]
pub use mac::Studio as Native;
#[cfg(target_os = "windows")]
pub use windows::Studio as Native;

pub use order::{Area, Order};
pub use reel::{called, Ears, Ending, Reel};

/// A failure to film. It travels between threads, hence `Send + Sync`.
pub type Failure = Box<dyn Error + Send + Sync>;

/// A camera that looks at the screen.
pub trait Camera {
    /// Starts filming. The bell rings when filming comes to an end by itself.
    ///
    /// # Errors
    ///
    /// The system does not show the screen, or there is nothing to encode video with.
    fn film(&self, order: &Order, bell: Sender<()>) -> Result<Box<dyn Take>, Failure>;
}

/// What is being filmed now.
pub trait Take {
    /// What is heard.
    fn ears(&self) -> Ears;

    /// Waits until it is time to finish.
    ///
    /// # Errors
    ///
    /// There is nobody left to ring the bell.
    fn rest(&self, bell: &Receiver<()>) -> Result<(), Failure> {
        bell.recv()?;
        Ok(())
    }

    /// Stops filming and finishes the file.
    ///
    /// # Errors
    ///
    /// Something went wrong while filming, or the file cannot be finished.
    fn finish(self: Box<Self>) -> Result<(), Failure>;
}
