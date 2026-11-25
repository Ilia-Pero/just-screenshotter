//! Filming on every macOS: `AVFoundation` films the screen and the microphone. It can do so since
//! 10.7, and it does not hear the computer.

use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use objc2::rc::Retained;
use objc2::runtime::{NSObjectProtocol, ProtocolObject};
use objc2::{define_class, msg_send, AllocAnyThread, DefinedClass};
use objc2_av_foundation::{
    AVCaptureConnection, AVCaptureFileOutput, AVCaptureFileOutputRecordingDelegate,
    AVCaptureMovieFileOutput, AVCaptureScreenInput, AVCaptureSession,
};
use objc2_core_media::CMTime;
use objc2_foundation::{
    NSArray, NSDate, NSDefaultRunLoopMode, NSError, NSObject, NSRunLoop, NSURL,
};
use screenshotter_phrase::Phrase;

use crate::{Ears, Failure, Order, Take};

use super::ear::{unheard, voice};
use super::place::{display, from_bottom, url};

/// Pictures in a second.
const PACE: i32 = 30;

/// How long to wait for the file to be finished.
const PATIENCE: Duration = Duration::from_secs(30);

/// How often to look whether it is time to stop.
const GLANCE: f64 = 0.2;

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

/// Starts filming.
///
/// # Errors
///
/// The system does not show the screen, or the microphone cannot be used.
pub fn film(order: &Order, bell: Sender<()>) -> Result<Box<dyn Take>, Failure> {
    let display = display(order)?;
    let file = url(&order.file)?;
    // SAFETY: the objects are alive, they are used in this thread only, and the constants of
    // time have a scale that is not zero.
    unsafe {
        let session = AVCaptureSession::new();
        let screen =
            AVCaptureScreenInput::initWithDisplayID(AVCaptureScreenInput::alloc(), display)
                .ok_or_else(|| Phrase::new("error.permission"))?;
        screen.setCropRect(from_bottom(&order.monitor, &order.area, order.scale));
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
