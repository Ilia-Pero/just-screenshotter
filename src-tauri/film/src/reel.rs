//! Filming in a process of its own.
//!
//! The program starts itself with the argument `--film` and an order. The second process films
//! until its input is closed, so it finishes the file even if the program is gone. It answers with
//! two lines at most: what it hears, and then what has gone wrong, if anything has.

use std::env;
use std::ffi::OsString;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitCode, Stdio};
use std::sync::mpsc;
use std::thread;

use serde::{Deserialize, Serialize};

use crate::{Camera, Failure, Order};

/// The argument that makes the program film.
const FLAG: &str = "--film";

/// What is heard while filming.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ears {
    /// What is heard from the computer.
    pub sound: bool,
    /// The microphone.
    pub voice: bool,
}

/// What the filming process says.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Line {
    Filming(Ears),
    Failed(String),
}

/// A film that is being made by another process.
pub struct Reel {
    input: ChildStdin,
}

/// The end of the film, which is yet to come.
pub struct Ending {
    child: Child,
    output: BufReader<ChildStdout>,
}

impl Reel {
    /// Starts filming. Returns when filming has begun. `code` is the language to complain in.
    ///
    /// # Errors
    ///
    /// The process has not started, or it cannot film.
    pub fn start(order: &Order, code: &str) -> Result<(Reel, Ending, Ears), Failure> {
        let mut child = Command::new(env::current_exe()?)
            .arg(FLAG)
            .arg(code)
            .arg(serde_json::to_string(order)?)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        let input = child.stdin.take().ok_or("The process has no input")?;
        let output = child.stdout.take().ok_or("The process has no output")?;
        let mut ending = Ending {
            child,
            output: BufReader::new(output),
        };
        match ending.line()? {
            Some(Line::Filming(ears)) => Ok((Reel { input }, ending, ears)),
            Some(Line::Failed(reason)) => Err(reason.into()),
            None => Err("The filming process has ended at once".into()),
        }
    }

    /// Asks to stop. The file is ready when the ending comes.
    pub fn stop(self) {
        drop(self.input);
    }
}

impl Ending {
    /// Waits until filming is over, takes no processor time meanwhile.
    ///
    /// # Errors
    ///
    /// Something has gone wrong, and the file may be spoiled.
    pub fn wait(mut self) -> Result<(), Failure> {
        let last = self.line();
        let status = self.child.wait()?;
        match last? {
            Some(Line::Failed(reason)) => Err(reason.into()),
            _ if status.success() => Ok(()),
            _ => Err(format!("The filming process has crashed: {status}").into()),
        }
    }

    fn line(&mut self) -> Result<Option<Line>, Failure> {
        let mut text = String::new();
        self.output.read_line(&mut text)?;
        if text.is_empty() {
            return Ok(None);
        }
        Ok(Some(serde_json::from_str(&text)?))
    }
}

/// Films, if the arguments ask for it. `prepare` is called with the code of the language first.
pub fn called(
    arguments: &[OsString],
    camera: &dyn Camera,
    prepare: impl Fn(&str),
) -> Option<ExitCode> {
    match arguments {
        [flag, code, order] if flag == FLAG => {
            prepare(&code.to_string_lossy());
            Some(shoot(
                &order.to_string_lossy(),
                camera,
                io::stdin(),
                io::stdout(),
            ))
        }
        _ => None,
    }
}

/// Films until there is no more input.
fn shoot(
    order: &str,
    camera: &dyn Camera,
    input: impl Read + Send + 'static,
    mut output: impl Write,
) -> ExitCode {
    let filmed = || -> Result<(), Failure> {
        let order: Order = serde_json::from_str(order)?;
        let (bell, rung) = mpsc::channel();
        let take = camera.film(&order, bell.clone())?;
        say(&mut output, &Line::Filming(take.ears()))?;
        thread::spawn(move || {
            let mut input = input;
            // Whatever comes is not needed; the end of the input is what matters.
            let _ = io::copy(&mut input, &mut io::sink());
            let _ = bell.send(());
        });
        let rested = take.rest(&rung);
        take.finish().and(rested)
    };
    match filmed() {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            // If there is nobody to tell, the code of the exit tells.
            let _ = say(&mut output, &Line::Failed(failure.to_string()));
            ExitCode::FAILURE
        }
    }
}

fn say(output: &mut impl Write, line: &Line) -> Result<(), Failure> {
    let mut text = serde_json::to_string(line)?;
    text.push('\n');
    output.write_all(text.as_bytes())?;
    output.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{self, Cursor};
    use std::path::PathBuf;
    use std::process::ExitCode;
    use std::sync::mpsc::Sender;

    use super::{called, shoot, Ears};
    use crate::{Area, Camera, Failure, Order, Take};

    /// A camera that hears the microphone and fails at the end with the reason, if there is one.
    struct Fake {
        reason: &'static str,
    }

    struct FakeTake {
        reason: &'static str,
    }

    impl Camera for Fake {
        fn film(&self, _order: &Order, _bell: Sender<()>) -> Result<Box<dyn Take>, Failure> {
            Ok(Box::new(FakeTake {
                reason: self.reason,
            }))
        }
    }

    impl Take for FakeTake {
        fn ears(&self) -> Ears {
            Ears {
                sound: false,
                voice: true,
            }
        }

        fn finish(self: Box<Self>) -> Result<(), Failure> {
            if self.reason.is_empty() {
                Ok(())
            } else {
                Err(self.reason.into())
            }
        }
    }

    fn order() -> String {
        let area = Area {
            left: 0,
            top: 0,
            width: 2,
            height: 2,
        };
        serde_json::to_string(&Order {
            monitor: area,
            scale: 1.0,
            area,
            sound: true,
            voice: true,
            file: PathBuf::from("film.mp4"),
        })
        .unwrap()
    }

    /// What the process says when it films with the camera by the order.
    fn said(order: &str, reason: &'static str) -> String {
        let mut output = Vec::new();
        let _ = shoot(
            order,
            &Fake { reason },
            Cursor::new(Vec::new()),
            &mut output,
        );
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn tells_what_is_heard() {
        assert_eq!(
            said(&order(), ""),
            "{\"Filming\":{\"sound\":false,\"voice\":true}}\n"
        );
    }

    #[test]
    fn tells_what_has_gone_wrong() {
        assert!(said(&order(), "no disk").ends_with("{\"Failed\":\"no disk\"}\n"));
    }

    #[test]
    fn does_not_film_by_a_broken_order() {
        assert!(said("{", "").starts_with("{\"Failed\""));
    }

    #[test]
    fn ends_with_a_code_of_failure() {
        assert_eq!(
            format!(
                "{:?}",
                shoot(
                    &order(),
                    &Fake { reason: "no disk" },
                    Cursor::new(Vec::new()),
                    io::sink()
                )
            ),
            format!("{:?}", ExitCode::FAILURE)
        );
    }

    #[test]
    fn does_not_film_when_not_asked() {
        assert!(called(&["--other".into()], &Fake { reason: "" }, |_| {}).is_none());
    }
}
