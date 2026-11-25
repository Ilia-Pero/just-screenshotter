//! Filming on Windows with what the system itself has: the screen comes from
//! Windows.Graphics.Capture, sound from WASAPI, and Media Foundation makes an MP4 file of them.

#![allow(unsafe_code)]

mod clock;
mod ear;
mod eye;
mod fence;
mod movie;
mod studio;

pub use studio::Studio;
