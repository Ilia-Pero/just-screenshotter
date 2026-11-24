//! The screen of macOS.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use screenshotter_phrase::Phrase;

use crate::{Failure, Patch, Screen};

/// The screen of macOS. The system program `screencapture` takes the picture: every version
/// since 10.13 has it, and it asks for permission to record the screen where one is needed
/// (10.15 and later).
///
/// @todo #1 Nobody has run this on a Mac yet. Check it on High Sierra and on the current macOS
///  with one and two monitors: `screencapture` writes files starting with the main monitor,
///  and the order has to match the order of monitors in Tauri.
pub struct Quartz;

impl Screen for Quartz {
    fn pngs(&self, patches: &[Patch]) -> Result<Vec<Vec<u8>>, Failure> {
        let folder = tempfile::tempdir()?;
        let files: Vec<PathBuf> = (0..patches.len())
            .map(|index| folder.path().join(format!("{index}.png")))
            .collect();
        let status = Command::new("/usr/sbin/screencapture")
            .args(["-x", "-t", "png"])
            .args(&files)
            .status()?;
        if !status.success() {
            return Err(refused());
        }
        files
            .iter()
            .map(|file| fs::read(file).map_err(|_| refused()))
            .collect()
    }
}

fn refused() -> Failure {
    Phrase::new("error.permission").into()
}
