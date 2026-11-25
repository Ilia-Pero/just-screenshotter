//! The program that films: `gst-launch-1.0` of `GStreamer`.

use std::io::{ErrorKind, Read};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use rustix::process::{kill_process, Pid, Signal};
use screenshotter_phrase::Phrase;

use crate::Failure;

/// How long to watch whether filming has begun: what is wrong comes out at once.
const START: Duration = Duration::from_millis(1500);

/// How often to look whether the program is still there.
const GLANCE: Duration = Duration::from_millis(100);

/// How long to wait for the file to be finished.
const PATIENCE: Duration = Duration::from_secs(30);

/// The running program, and what it says about its troubles.
pub struct Launch {
    child: Arc<Mutex<Child>>,
    troubles: JoinHandle<String>,
}

impl Launch {
    /// Starts filming and makes sure that it has begun.
    pub fn start(words: &[String]) -> Result<Launch, Failure> {
        let mut child = Command::new("gst-launch-1.0")
            .args(words)
            // Its messages are looked through, so they have to be in English.
            .env("LC_ALL", "C.UTF-8")
            .env("LANGUAGE", "C")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|failure| -> Failure {
                if failure.kind() == ErrorKind::NotFound {
                    Phrase::new("error.gstreamer").into()
                } else {
                    failure.into()
                }
            })?;
        let mut output = child
            .stderr
            .take()
            .ok_or("GStreamer cannot tell about its troubles")?;
        let troubles = thread::spawn(move || {
            let mut text = String::new();
            // What is not text does not matter.
            let _ = output.read_to_string(&mut text);
            text
        });
        let launch = Launch {
            child: Arc::new(Mutex::new(child)),
            troubles,
        };
        let began = Instant::now();
        while began.elapsed() < START {
            if launch.ended()? {
                return Err(launch.trouble());
            }
            thread::sleep(GLANCE);
        }
        Ok(launch)
    }

    /// Rings the bell when the program ends.
    pub fn watch(&self, bell: Sender<()>) {
        let child = Arc::clone(&self.child);
        thread::spawn(move || {
            while !ended(&child).unwrap_or(true) {
                thread::sleep(START);
            }
            // If nobody listens to the bell, filming is being finished already.
            let _ = bell.send(());
        });
    }

    /// Asks the program to finish the file and waits until it does.
    pub fn finish(self) -> Result<(), Failure> {
        if !self.ended()? {
            let id = i32::try_from(self.locked(|child| child.id()))?;
            kill_process(
                Pid::from_raw(id).ok_or("GStreamer has no number")?,
                Signal::INT,
            )?;
            let asked = Instant::now();
            while !self.ended()? {
                if asked.elapsed() > PATIENCE {
                    self.locked(Child::kill)?;
                    return Err("GStreamer does not finish the file".into());
                }
                thread::sleep(GLANCE);
            }
        }
        let trouble = self.trouble().to_string();
        if trouble.contains("ERROR") {
            Err(trouble.into())
        } else {
            Ok(())
        }
    }

    /// What has gone wrong.
    fn trouble(self) -> Failure {
        let text = self.troubles.join().unwrap_or_default();
        if text.contains("no element") {
            return Phrase::new("error.gstreamer").into();
        }
        text.trim().into()
    }

    fn ended(&self) -> Result<bool, Failure> {
        ended(&self.child)
    }

    fn locked<T>(&self, act: impl FnOnce(&mut Child) -> T) -> T {
        act(&mut self.child.lock().unwrap_or_else(PoisonError::into_inner))
    }
}

fn ended(child: &Mutex<Child>) -> Result<bool, Failure> {
    Ok(child
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .try_wait()?
        .is_some())
}
