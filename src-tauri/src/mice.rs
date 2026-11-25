//! Buttons of the mouse that start actions.

use std::sync::{Mutex, PoisonError};

use screenshotter_mouse::{available, Chord, Watch};
use screenshotter_store::{Failure, Triggers};
use tauri::AppHandle;

use crate::act::Act;

/// The watch for buttons. There is none until a person wants it, because it costs processor time
/// while the mouse moves.
#[derive(Default)]
pub struct Mice {
    watch: Mutex<Option<Watch>>,
}

impl Mice {
    /// Watches for the buttons that start actions, instead of the old ones.
    pub fn follow(&self, app: &AppHandle, triggers: &Triggers) -> Result<(), Failure> {
        self.rest();
        let wanted = Chord::named(
            triggers
                .all()
                .map(|trigger| trigger.mouse.as_str())
                .into_iter(),
        );
        if wanted.is_empty() || !available() {
            return Ok(());
        }
        let app = app.clone();
        let triggers = triggers.clone();
        let watch = Watch::start(wanted, move |chord| {
            for (act, trigger) in Act::ALL.into_iter().zip(triggers.all()) {
                if trigger.mouse == chord.code() && act.possible() {
                    act.start(&app);
                }
            }
        })?;
        *self.watch.lock().unwrap_or_else(PoisonError::into_inner) = Some(watch);
        Ok(())
    }

    /// Watches no more.
    pub fn rest(&self) {
        let watch = self
            .watch
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        drop(watch);
    }
}
