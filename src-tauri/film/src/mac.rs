//! Filming on macOS: `AVFoundation` films the screen and the microphone. It can do so since 10.7.

#![allow(unsafe_code)]

use std::path::Path;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use objc2::rc::Retained;
use objc2::runtime::{NSObjectProtocol, ProtocolObject};
use objc2::{define_class, msg_send, AllocAnyThread, DefinedClass};
use objc2_av_foundation::{
    AVCaptureConnection, AVCaptureDevice, AVCaptureDeviceInput, AVCaptureFileOutput,
    AVCaptureFileOutputRecordingDelegate, AVCaptureMovieFileOutput, AVCaptureScreenInput,
    AVCaptureSession, AVMediaTypeAudio,
};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_core_graphics::{CGDirectDisplayID, CGError, CGGetDisplaysWithPoint};
use objc2_core_media::CMTime;
use objc2_foundation::{
    NSArray, NSDate, NSDefaultRunLoopMode, NSError, NSObject, NSRunLoop, NSString, NSURL,
};
use screenshotter_phrase::Phrase;

use crate::{Area, Camera, Ears, Failure, Order, Take};

/// Pictures in a second.
const PACE: i32 = 30;

/// How long to wait for the file to be finished.
const PATIENCE: Duration = Duration::from_secs(30);

/// How often to look whether it is time to stop.
const GLANCE: f64 = 0.2;

/// The camera of macOS.
///
/// @todo #1 Nobody has filmed on a Mac yet. Check on High Sierra and on the current macOS that
///  the film shows the chosen area on each of two monitors and on a Retina screen, and that the
///  microphone is heard. The tests of GitHub only film its virtual screen.
///
/// @todo #1 On macOS nothing shows what area is being filmed. Put a frame around it, as
///  `windows/fence.rs` does: four windows of `NSWindow` that let the mouse through and have
///  `sharingType` set to none.
pub struct Studio;

/// Filming that goes on.
struct Shoot {
    session: Retained<AVCaptureSession>,
    output: Retained<AVCaptureMovieFileOutput>,
    outcome: Arc<Outcome>,
    ears: Ears,
    _witness: Retained<Witness>,
}

/// How filming has ended, when it has.
struct Outcome {
    ended: Mutex<Option<Result<(), String>>>,
    bell: Sender<()>,
}

define_class!(
    // SAFETY: NSObject has no requirements for those who inherit from it.
    #[unsafe(super(NSObject))]
    #[name = "JustScreenshotterWitness"]
    #[ivars = Arc<Outcome>]
    /// The one to whom the system tells that the file is ready.
    struct Witness;

    unsafe impl NSObjectProtocol for Witness {}

    unsafe impl AVCaptureFileOutputRecordingDelegate for Witness {
        #[unsafe(method(captureOutput:didFinishRecordingToOutputFileAtURL:fromConnections:error:))]
        fn finished(
            &self,
            _output: &AVCaptureFileOutput,
            _file: &NSURL,
            _connections: &NSArray<AVCaptureConnection>,
            error: Option<&NSError>,
        ) {
            let outcome = self.ivars();
            *outcome
                .ended
                .lock()
                .unwrap_or_else(PoisonError::into_inner) = Some(match error {
                None => Ok(()),
                Some(error) => Err(error.localizedDescription().to_string()),
            });
            // If nobody listens to the bell, filming is being finished already.
            let _ = outcome.bell.send(());
        }
    }
);

impl Witness {
    fn new(outcome: Arc<Outcome>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(outcome);
        // SAFETY: the initialiser of NSObject has no requirements.
        unsafe { msg_send![super(this), init] }
    }
}

impl Camera for Studio {
    fn film(&self, order: &Order, bell: Sender<()>) -> Result<Box<dyn Take>, Failure> {
        let display = display(order)?;
        let file = url(&order.file)?;
        // SAFETY: the objects are alive, they are used in this thread only, and the constants of
        // time have a scale that is not zero.
        unsafe {
            let session = AVCaptureSession::new();
            let screen =
                AVCaptureScreenInput::initWithDisplayID(AVCaptureScreenInput::alloc(), display)
                    .ok_or_else(|| Phrase::new("error.permission"))?;
            screen.setCropRect(crop(&order.monitor, &order.area, order.scale));
            screen.setMinFrameDuration(CMTime::new(1, PACE));
            screen.setCapturesCursor(true);
            if !session.canAddInput(&screen) {
                return Err(Phrase::new("error.permission").into());
            }
            session.addInput(&screen);
            if order.voice {
                let voice = voice()?;
                if !session.canAddInput(&voice) {
                    return Err(unheard("AVCaptureSession"));
                }
                session.addInput(&voice);
            }
            let output = AVCaptureMovieFileOutput::new();
            // The file is made of fragments of a second, so what has already been written is not
            // lost if the computer is switched off.
            output.setMovieFragmentInterval(CMTime::new(1, 1));
            if !session.canAddOutput(&output) {
                return Err("macOS does not write films".into());
            }
            session.addOutput(&output);
            session.startRunning();
            let outcome = Arc::new(Outcome {
                ended: Mutex::new(None),
                bell,
            });
            let witness = Witness::new(Arc::clone(&outcome));
            output.startRecordingToOutputFileURL_recordingDelegate(
                &file,
                ProtocolObject::from_ref(&*witness),
            );
            Ok(Box::new(Shoot {
                session,
                output,
                outcome,
                ears: Ears {
                    sound: false,
                    voice: order.voice,
                },
                _witness: witness,
            }))
        }
    }
}

impl Take for Shoot {
    fn ears(&self) -> Ears {
        self.ears
    }

    /// The system tells about the end of filming in the loop of the main thread, so the loop has
    /// to go on.
    fn rest(&self, bell: &Receiver<()>) -> Result<(), Failure> {
        while bell.try_recv() == Err(TryRecvError::Empty) {
            spin();
        }
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Failure> {
        let asked = Instant::now();
        // SAFETY: the objects are alive and are used in this thread only.
        unsafe {
            if self.output.isRecording() {
                self.output.stopRecording();
            }
        }
        let ended = loop {
            let ended = self
                .outcome
                .ended
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .take();
            match ended {
                Some(ended) => break ended,
                None if asked.elapsed() > PATIENCE => {
                    break Err("macOS does not finish the file".to_owned())
                }
                None => spin(),
            }
        };
        // SAFETY: see above.
        unsafe { self.session.stopRunning() };
        ended.map_err(Failure::from)
    }
}

/// Lets the loop of the thread do its work for a moment.
fn spin() {
    // SAFETY: the constant is a string that lives as long as the process.
    let mode = unsafe { NSDefaultRunLoopMode };
    let until = NSDate::dateWithTimeIntervalSinceNow(GLANCE);
    if !NSRunLoop::currentRunLoop().runMode_beforeDate(mode, &until) {
        // There is nothing in the loop to wait for.
        thread::sleep(Duration::from_secs_f64(GLANCE));
    }
}

/// The microphone the system uses at the moment.
fn voice() -> Result<Retained<AVCaptureDeviceInput>, Failure> {
    // SAFETY: the constant and the device are alive.
    unsafe {
        let kind = AVMediaTypeAudio.ok_or_else(|| unheard("AVMediaTypeAudio"))?;
        let device = AVCaptureDevice::defaultDeviceWithMediaType(kind)
            .ok_or_else(|| unheard("AVCaptureDevice"))?;
        AVCaptureDeviceInput::deviceInputWithDevice_error(&device)
            .map_err(|error| unheard(&error.localizedDescription().to_string()))
    }
}

fn unheard(reason: &str) -> Failure {
    Phrase::new("error.voice").with("reason", reason).into()
}

/// The display where the monitor is. Points of the desktop of macOS are logical.
fn display(order: &Order) -> Result<CGDirectDisplayID, Failure> {
    let scale = order.scale.max(1.0);
    let inside = CGPoint {
        x: f64::from(order.monitor.left) / scale + 1.0,
        y: f64::from(order.monitor.top) / scale + 1.0,
    };
    let mut display = 0;
    let mut count = 0;
    // SAFETY: there is a place for one display and for the count, and both outlive the call.
    let found = unsafe { CGGetDisplaysWithPoint(inside, 1, &raw mut display, &raw mut count) };
    if found == CGError::Success && count > 0 {
        Ok(display)
    } else {
        Err("There is no display".into())
    }
}

fn url(file: &Path) -> Result<Retained<NSURL>, Failure> {
    let path = file.to_str().ok_or("The name of the file is not Unicode")?;
    Ok(NSURL::fileURLWithPath(&NSString::from_str(path)))
}

/// The area in logical points, counting from the lower left corner of the monitor.
fn crop(monitor: &Area, area: &Area, scale: f64) -> CGRect {
    let scale = scale.max(1.0);
    let bottom = f64::from(area.top) + f64::from(area.height);
    CGRect {
        origin: CGPoint {
            x: f64::from(area.left) / scale,
            y: (f64::from(monitor.height) - bottom) / scale,
        },
        size: CGSize {
            width: f64::from(area.width) / scale,
            height: f64::from(area.height) / scale,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::crop;
    use crate::Area;

    const MONITOR: Area = Area {
        left: 0,
        top: 0,
        width: 2880,
        height: 1800,
    };
    const AREA: Area = Area {
        left: 200,
        top: 100,
        width: 800,
        height: 600,
    };

    #[test]
    fn counts_from_the_bottom() {
        assert!((crop(&MONITOR, &AREA, 2.0).origin.y - 550.0).abs() < 0.01);
    }

    #[test]
    fn counts_in_logical_points() {
        assert!((crop(&MONITOR, &AREA, 2.0).size.width - 400.0).abs() < 0.01);
    }
}
