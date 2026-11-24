//! Screenshots that windows of the editor pick up.

use std::collections::HashMap;
use std::sync::{Condvar, Mutex, PoisonError};
use std::time::Duration;

/// How long a window waits for its screenshot.
const PATIENCE: Duration = Duration::from_secs(20);

/// Screenshots by labels of editor windows. A window picks its screenshot up only once.
/// It may come before the screenshot is ready, then it waits.
#[derive(Default)]
pub struct Album {
    shots: Mutex<HashMap<String, Vec<u8>>>,
    arrived: Condvar,
}

impl Album {
    /// Keeps the screenshot for the window with the label.
    pub fn keep(&self, label: &str, png: Vec<u8>) {
        self.shots
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(label.to_owned(), png);
        self.arrived.notify_all();
    }

    /// Gives the screenshot away and forgets it, so that it does not stay in memory.
    pub fn take(&self, label: &str) -> Result<Vec<u8>, String> {
        let shots = self.shots.lock().unwrap_or_else(PoisonError::into_inner);
        self.arrived
            .wait_timeout_while(shots, PATIENCE, |shots| !shots.contains_key(label))
            .unwrap_or_else(PoisonError::into_inner)
            .0
            .remove(label)
            .ok_or_else(|| format!("There is no screenshot for {label}"))
    }

    /// Forgets all screenshots.
    pub fn clear(&self) {
        self.shots
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clear();
    }
}
