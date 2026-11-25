//! Filming on macOS with what the system itself has. Since macOS 13 `ScreenCaptureKit` gives the
//! sound of the computer together with the screen. Before that `AVFoundation` gives the screen.
//! The microphone and the file belong to `AVFoundation` on all systems.

#![allow(unsafe_code)]

use std::sync::mpsc::Sender;

use objc2_foundation::{NSOperatingSystemVersion, NSProcessInfo};

use crate::{Camera, Failure, Order, Take};

mod booth;
mod clock;
mod ear;
mod eye;
mod lens;
mod movie;
mod place;
mod shoot;

/// The system that gives the sound of the computer.
const HEARING: NSOperatingSystemVersion = NSOperatingSystemVersion {
    majorVersion: 13,
    minorVersion: 0,
    patchVersion: 0,
};

/// The camera of macOS before 13.
pub struct Veteran;

impl Camera for Veteran {
    fn hears(&self) -> bool {
        false
    }

    fn film(&self, order: &Order, bell: Sender<()>) -> Result<Box<dyn Take>, Failure> {
        let order = Order {
            sound: false,
            ..order.clone()
        };
        shoot::film(&order, bell, shoot::lens)
    }
}

/// The camera of macOS.
///
/// @todo #1 Nobody has filmed on a Mac yet. Check on High Sierra and on the current macOS that
///  the film shows the chosen area on each of two monitors and on a Retina screen, and that the
///  microphone and, since macOS 13, the sound of the computer are heard. The tests of GitHub only
///  film its virtual screen.
///
/// @todo #1 On macOS nothing shows what area is being filmed. Put a frame around it, as
///  `windows/fence.rs` does: four windows of `NSWindow` that let the mouse through and have
///  `sharingType` set to none.
pub struct Studio;

impl Camera for Studio {
    fn hears(&self) -> bool {
        NSProcessInfo::processInfo().isOperatingSystemAtLeastVersion(HEARING)
    }

    fn film(&self, order: &Order, bell: Sender<()>) -> Result<Box<dyn Take>, Failure> {
        if self.hears() {
            shoot::film(order, bell, shoot::eye)
        } else {
            Veteran.film(order, bell)
        }
    }
}
