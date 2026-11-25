//! The window a person works with.

#![allow(unsafe_code)]

use std::mem::{size_of, zeroed};

use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows_sys::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

use crate::Patch;

/// Where the window is, without its shadow. The patch is blank if there is no such window.
#[must_use]
pub fn front() -> Patch {
    let size = u32::try_from(size_of::<RECT>()).unwrap_or(0);
    // SAFETY: there is a place for the answer of this size, and a null window is refused.
    let (answer, frame) = unsafe {
        let mut frame: RECT = zeroed();
        let answer = DwmGetWindowAttribute(
            GetForegroundWindow(),
            DWMWA_EXTENDED_FRAME_BOUNDS.cast_unsigned(),
            (&raw mut frame).cast(),
            size,
        );
        (answer, frame)
    };
    if answer < 0 {
        return Patch::default();
    }
    Patch {
        left: frame.left,
        top: frame.top,
        width: u32::try_from(frame.right - frame.left).unwrap_or(0),
        height: u32::try_from(frame.bottom - frame.top).unwrap_or(0),
    }
}
