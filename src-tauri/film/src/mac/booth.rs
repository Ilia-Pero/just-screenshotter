//! The desk, where threads of the system come one by one.

use std::sync::mpsc::Sender;
use std::sync::{Mutex, MutexGuard, PoisonError};

use crate::desk::Desk;
use crate::Failure;

use super::movie::{Movie, Picture};

/// A desk in a booth with one door.
pub struct Booth {
    desk: Mutex<Desk<Movie>>,
    /// What has gone wrong first.
    fault: Mutex<Option<String>>,
    bell: Sender<()>,
}

impl Booth {
    pub fn new(desk: Desk<Movie>, bell: Sender<()>) -> Self {
        Self {
            desk: Mutex::new(desk),
            fault: Mutex::new(None),
            bell,
        }
    }

    /// Whether the film has begun.
    pub fn began(&self) -> bool {
        open(&self.desk).began()
    }

    /// Starts the film.
    pub fn start(&self, moment: i64) {
        let started = open(&self.desk).start(moment);
        self.note(started);
    }

    /// Takes a picture.
    pub fn see(&self, picture: Picture, moment: i64) {
        let seen = open(&self.desk).see(picture, moment);
        self.note(seen);
    }

    /// Takes what a source has said.
    pub fn hear(&self, source: usize, moment: i64, samples: &[i16]) {
        open(&self.desk).hear(source, moment, samples);
    }

    /// Writes down what has come up to the moment.
    pub fn turn(&self, until: i64) {
        let turned = open(&self.desk).turn(until);
        self.note(turned);
    }

    /// Filming cannot go on.
    pub fn fail(&self, reason: String) {
        self.note(Err(reason.into()));
    }

    /// Finishes the film.
    ///
    /// # Errors
    ///
    /// Something has gone wrong now or before.
    pub fn finish(&self, moment: i64) -> Result<(), Failure> {
        let finished = open(&self.desk).finish(moment);
        let fault = open(&self.fault).take();
        fault.map_or(finished, |reason| Err(reason.into()))
    }

    fn note(&self, done: Result<(), Failure>) {
        if let Err(failure) = done {
            open(&self.fault).get_or_insert(failure.to_string());
            // If nobody listens to the bell, filming is being finished already.
            let _ = self.bell.send(());
        }
    }
}

fn open<T>(door: &Mutex<T>) -> MutexGuard<'_, T> {
    door.lock().unwrap_or_else(PoisonError::into_inner)
}
