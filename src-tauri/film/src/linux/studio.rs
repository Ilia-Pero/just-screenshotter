//! Everything that takes part in filming.

use std::env;
use std::sync::mpsc::Sender;

use screenshotter_phrase::Phrase;

use crate::pipeline::{Pipeline, Scene};
use crate::{Camera, Ears, Failure, Order, Take};

use super::launch::Launch;
use super::portal::Portal;

/// The camera of Linux.
///
/// @todo #1 Nobody has filmed under Wayland yet: the tests of GitHub film the X server only. Check
///  on Ubuntu 26.04 that the film shows the chosen area, also with a fractional scale and with
///  two monitors, where a person may choose another monitor in the window of the desktop.
pub struct Studio;

/// Filming that goes on.
struct Shoot {
    launch: Launch,
    portal: Option<Portal>,
    ears: Ears,
}

impl Camera for Studio {
    fn film(&self, order: &Order, bell: Sender<()>) -> Result<Box<dyn Take>, Failure> {
        let area = order.area.within(order.monitor.width, order.monitor.height);
        if area.blank() {
            return Err("There is nothing to film outside the screen".into());
        }
        let portal = env::var_os("WAYLAND_DISPLAY")
            .map(|_| Portal::open(bell.clone()))
            .transpose()?;
        let mut pipeline = Pipeline {
            scene: portal.as_ref().map_or(Scene::Display, Portal::scene),
            monitor: order.monitor,
            area,
            ears: Ears {
                sound: order.sound,
                voice: order.voice,
            },
            file: &order.file,
        };
        let mut launched = Launch::start(&pipeline.words());
        if launched.is_err() && order.sound {
            // There may be no sound device: then there is nothing to hear.
            pipeline.ears.sound = false;
            launched = Launch::start(&pipeline.words());
        }
        let launch = launched.map_err(|failure| -> Failure {
            if order.voice && failure.to_string().contains("pulse") {
                Phrase::new("error.voice").with("reason", failure).into()
            } else {
                failure
            }
        })?;
        launch.watch(bell);
        Ok(Box::new(Shoot {
            launch,
            portal,
            ears: pipeline.ears,
        }))
    }
}

impl Take for Shoot {
    fn ears(&self) -> Ears {
        self.ears
    }

    fn finish(self: Box<Self>) -> Result<(), Failure> {
        let finished = self.launch.finish();
        if let Some(portal) = self.portal {
            portal.close();
        }
        finished
    }
}
