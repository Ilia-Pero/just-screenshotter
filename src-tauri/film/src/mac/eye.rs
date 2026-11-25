//! What the computer shows and plays, by `ScreenCaptureKit`. It exists since macOS 12.3 and is
//! loaded only here, so the program starts on older systems and has no need of it at rest.

use std::ffi::CStr;
use std::os::unix::process::parent_id;
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::time::Duration;

use block2::RcBlock;
use dispatch2::DispatchQueue;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject, NSObjectProtocol};
use objc2::{define_class, msg_send, AllocAnyThread, DefinedClass};
use objc2_core_graphics::CGDirectDisplayID;
use objc2_core_media::{CMAudioFormatDescriptionGetStreamBasicDescription, CMSampleBuffer, CMTime};
use objc2_foundation::{NSArray, NSBundle, NSError, NSObject, NSString};
use screenshotter_phrase::Phrase;

use crate::mix::{CHANNELS, RATE};
use crate::pcm::Shape;
use crate::{Failure, Order};

use super::booth::Booth;
use super::clock::moment;
use super::movie::{size, Picture, NV12, PACE};
use super::place::{display, from_top};

/// Where the system keeps the library.
const KIT: &str = "/System/Library/Frameworks/ScreenCaptureKit.framework";

/// What a stream gives.
const SCREEN: isize = 0;
const SOUND: isize = 1;

/// The code of a failure that means: a person has not allowed to record the screen.
const DECLINED: isize = -3801;

/// How many pictures the stream keeps at once. One of them is the last one, which may be repeated.
const QUEUE: isize = 6;

/// The flags of sound.
const FLOAT: u32 = 1;
const APART: u32 = 1 << 5;

/// How long to wait for an answer. The first one may take long, the system asks a person then.
const PATIENCE: Duration = Duration::from_secs(60);

/// A stream of what the computer shows and plays.
pub struct Eye {
    stream: Retained<AnyObject>,
    _witness: Retained<Witness>,
}

define_class!(
    // SAFETY: NSObject has no requirements for those who inherit from it.
    #[unsafe(super(NSObject))]
    #[name = "JustScreenshotterEye"]
    #[ivars = Arc<Booth>]
    /// The one to whom the system gives what it sees and hears.
    struct Witness;

    unsafe impl NSObjectProtocol for Witness {}

    impl Witness {
        #[unsafe(method(stream:didOutputSampleBuffer:ofType:))]
        fn given(&self, _stream: &AnyObject, buffer: &CMSampleBuffer, kind: isize) {
            // SAFETY: the buffer is alive until the end of the call.
            let moment = moment(unsafe { buffer.presentation_time_stamp() });
            match kind {
                SCREEN => {
                    if let Some(picture) = Picture::new(buffer) {
                        self.ivars().see(picture, moment);
                    }
                }
                SOUND => {
                    if let Some(samples) = samples(buffer) {
                        self.ivars().hear(0, moment, &samples);
                    }
                }
                _ => {}
            }
        }

        #[unsafe(method(stream:didStopWithError:))]
        fn stopped(&self, _stream: &AnyObject, error: &NSError) {
            self.ivars().fail(error.localizedDescription().to_string());
        }
    }
);

impl Witness {
    fn new(booth: Arc<Booth>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(booth);
        // SAFETY: the initialiser of NSObject has no requirements.
        unsafe { msg_send![super(this), init] }
    }
}

impl Eye {
    /// Opens the eye. Returns when it sees, though there may be no picture yet.
    ///
    /// # Errors
    ///
    /// A person has not allowed to record the screen, or the system has no such library.
    pub fn open(order: &Order, booth: Arc<Booth>, queue: &DispatchQueue) -> Result<Eye, Failure> {
        let display = display(order)?;
        load()?;
        let witness = Witness::new(booth);
        // SAFETY: the messages are those of `ScreenCaptureKit` of macOS 13, the objects are alive,
        // and the queue lives as long as the stream keeps it.
        unsafe {
            let filter = filter(display)?;
            let settings = settings(order)?;
            let stream: Allocated<AnyObject> = msg_send![part(c"SCStream")?, alloc];
            let stream: Option<Retained<AnyObject>> = msg_send![
                stream,
                initWithFilter: &*filter,
                configuration: &*settings,
                delegate: &*witness,
            ];
            let stream = stream.ok_or_else(|| Phrase::new("error.permission"))?;
            let kinds = [SCREEN].into_iter().chain(order.sound.then_some(SOUND));
            for kind in kinds {
                let added: Result<(), Retained<NSError>> = msg_send![
                    &*stream,
                    addStreamOutput: &*witness,
                    type: kind,
                    sampleHandlerQueue: queue,
                    error: _,
                ];
                added.map_err(|error| reason(&error))?;
            }
            let (answer, answered) = answer();
            let () = msg_send![&*stream, startCaptureWithCompletionHandler: &*answer];
            wait(&answered)?;
            Ok(Eye {
                stream,
                _witness: witness,
            })
        }
    }
}

impl Drop for Eye {
    /// Closes the eye. It gives nothing after that.
    fn drop(&mut self) {
        let (answer, answered) = answer();
        // SAFETY: the stream is alive, and it keeps the block until it is called.
        unsafe {
            let () = msg_send![&*self.stream, stopCaptureWithCompletionHandler: &*answer];
        }
        // If it has stopped by itself already, there is nothing else to do.
        let _ = wait(&answered);
    }
}

/// A block the system calls with a failure or without one.
type Answer = RcBlock<dyn Fn(*mut NSError)>;

/// A block for an answer, and where the answer comes.
fn answer() -> (Answer, Receiver<Result<(), String>>) {
    let (sender, receiver) = mpsc::channel();
    let block = RcBlock::new(move |error: *mut NSError| {
        // SAFETY: the system gives a live failure or none.
        let answer = match unsafe { error.as_ref() } {
            None => Ok(()),
            Some(error) => Err(reason(error)),
        };
        // If nobody waits any more, there is nobody to tell.
        let _ = sender.send(answer);
    });
    (block, receiver)
}

fn wait<T>(answered: &Receiver<Result<T, String>>) -> Result<T, Failure> {
    Ok(answered
        .recv_timeout(PATIENCE)
        .map_err(|_| "macOS does not answer")??)
}

fn reason(error: &NSError) -> String {
    if error.code() == DECLINED {
        Phrase::new("error.permission").to_string()
    } else {
        error.localizedDescription().to_string()
    }
}

/// Loads the library. The system does it once.
fn load() -> Result<(), Failure> {
    NSBundle::bundleWithPath(&NSString::from_str(KIT))
        // SAFETY: this is a library of the system.
        .filter(|kit| unsafe { kit.load() })
        .map(drop)
        .ok_or_else(|| "macOS has no ScreenCaptureKit".into())
}

/// A class of the library.
fn part(name: &CStr) -> Result<&'static AnyClass, Failure> {
    AnyClass::get(name).ok_or_else(|| format!("macOS has no {}", name.to_string_lossy()).into())
}

/// What is there to look at.
struct Content(Retained<AnyObject>);

// SAFETY: the object does not change.
unsafe impl Send for Content {}

/// The display without the windows of the program itself.
///
/// # Safety
///
/// The library is loaded.
unsafe fn filter(display: CGDirectDisplayID) -> Result<Retained<AnyObject>, Failure> {
    let (sender, receiver) = mpsc::channel();
    let answer = RcBlock::new(move |content: *mut AnyObject, error: *mut NSError| {
        // SAFETY: the system gives live objects or none.
        let found = unsafe {
            match (Retained::retain(content), error.as_ref()) {
                (Some(content), _) => Ok(Content(content)),
                (None, Some(error)) => Err(reason(error)),
                (None, None) => Err(Phrase::new("error.permission").to_string()),
            }
        };
        // If nobody waits any more, there is nobody to tell.
        let _ = sender.send(found);
    });
    // SAFETY: it is promised above, the messages are those of `ScreenCaptureKit`.
    unsafe {
        let () = msg_send![
            part(c"SCShareableContent")?,
            getShareableContentExcludingDesktopWindows: false,
            onScreenWindowsOnly: false,
            completionHandler: &*answer,
        ];
        let Content(content) = wait(&receiver)?;
        let displays: Retained<NSArray<AnyObject>> = msg_send![&*content, displays];
        let seen = displays
            .iter()
            .find(|seen| {
                let number: CGDirectDisplayID = msg_send![&**seen, displayID];
                number == display
            })
            .ok_or("There is no display")?;
        let programs: Retained<NSArray<AnyObject>> = msg_send![&*content, applications];
        let own: Vec<Retained<AnyObject>> = programs
            .iter()
            .filter(|program| {
                let number: i32 = msg_send![&**program, processID];
                u32::try_from(number) == Ok(parent_id())
            })
            .collect();
        let own = NSArray::from_retained_slice(&own);
        let none = NSArray::<AnyObject>::new();
        let filter: Allocated<AnyObject> = msg_send![part(c"SCContentFilter")?, alloc];
        Ok(msg_send![
            filter,
            initWithDisplay: &*seen,
            excludingApplications: &*own,
            exceptingWindows: &*none,
        ])
    }
}

/// What the stream has to give.
///
/// # Safety
///
/// The library is loaded.
unsafe fn settings(order: &Order) -> Result<Retained<AnyObject>, Failure> {
    let (width, height) = size(&order.area);
    // SAFETY: it is promised above, the messages are those of `ScreenCaptureKit` of macOS 13.
    unsafe {
        let settings: Retained<AnyObject> = msg_send![part(c"SCStreamConfiguration")?, new];
        let () = msg_send![&*settings, setWidth: width as usize];
        let () = msg_send![&*settings, setHeight: height as usize];
        let () = msg_send![&*settings, setSourceRect: from_top(&order.area, order.scale)];
        let () = msg_send![&*settings, setScalesToFit: true];
        let pace = CMTime::new(1, i32::try_from(PACE)?);
        let () = msg_send![&*settings, setMinimumFrameInterval: pace];
        let () = msg_send![&*settings, setPixelFormat: NV12];
        let () = msg_send![&*settings, setQueueDepth: QUEUE];
        let () = msg_send![&*settings, setShowsCursor: true];
        let () = msg_send![&*settings, setCapturesAudio: order.sound];
        let () = msg_send![&*settings, setSampleRate: isize::try_from(RATE)?];
        let () = msg_send![&*settings, setChannelCount: isize::try_from(CHANNELS)?];
        Ok(settings)
    }
}

/// The sound of the buffer, if there is any.
pub fn samples(buffer: &CMSampleBuffer) -> Option<Vec<i16>> {
    // SAFETY: the buffer is alive, its description lives as long as it does, and there is enough
    // place for the bytes.
    unsafe {
        let format = buffer.format_description()?;
        let sound = CMAudioFormatDescriptionGetStreamBasicDescription(&format).as_ref()?;
        let shape = Shape {
            channels: usize::try_from(sound.mChannelsPerFrame).ok()?,
            float: sound.mFormatFlags & FLOAT != 0,
            apart: sound.mFormatFlags & APART != 0,
        };
        let block = buffer.data_buffer()?;
        let mut bytes = vec![0_u8; block.data_length()];
        let place = std::ptr::NonNull::new(bytes.as_mut_ptr())?.cast();
        let copied = block.copy_data_bytes(0, bytes.len(), place);
        (copied == 0).then(|| shape.stereo(&bytes))
    }
}
