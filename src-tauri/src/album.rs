//! Screenshots that windows of the editor pick up.

use std::collections::HashMap;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use serde::Serialize;

/// How long a window waits for its screenshot.
const PATIENCE: Duration = Duration::from_secs(20);

/// What a window of the editor begins with: the area that is selected already, on its monitor, in
/// physical pixels, and whether a film is wanted.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Opening {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
    pub film: bool,
}

/// Screenshots by labels of editor windows. A window picks its screenshot up only once.
/// It may come before the screenshot is ready, then it waits.
#[derive(Default)]
pub struct Album {
    shots: Mutex<HashMap<String, Vec<u8>>>,
    arrived: Condvar,
    openings: Mutex<HashMap<String, Opening>>,
}

impl Album {
    /// Remembers what the window with the label begins with.
    pub fn plan(&self, label: &str, opening: Opening) {
        open(&self.openings).insert(label.to_owned(), opening);
    }

    /// What the window with the label begins with.
    pub fn opening(&self, label: &str) -> Opening {
        open(&self.openings).get(label).copied().unwrap_or_default()
    }

    /// Keeps the screenshot for the window with the label.
    pub fn keep(&self, label: &str, png: Vec<u8>) {
        open(&self.shots).insert(label.to_owned(), png);
        self.arrived.notify_all();
    }

    /// Gives the screenshot away and forgets it, so that it does not stay in memory.
    pub fn take(&self, label: &str) -> Result<Vec<u8>, String> {
        self.arrived
            .wait_timeout_while(open(&self.shots), PATIENCE, |shots| {
                !shots.contains_key(label)
            })
            .unwrap_or_else(PoisonError::into_inner)
            .0
            .remove(label)
            .ok_or_else(|| format!("There is no screenshot for {label}"))
    }

    /// Forgets all screenshots.
    pub fn clear(&self) {
        open(&self.shots).clear();
        open(&self.openings).clear();
    }
}

fn open<T>(shelf: &Mutex<T>) -> MutexGuard<'_, T> {
    shelf.lock().unwrap_or_else(PoisonError::into_inner)
}
