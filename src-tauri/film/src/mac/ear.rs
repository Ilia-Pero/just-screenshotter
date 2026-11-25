//! The microphone.

use std::sync::Arc;

use dispatch2::DispatchQueue;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObjectProtocol, ProtocolObject};
use objc2::{define_class, msg_send, AllocAnyThread, DefinedClass};
use objc2_av_foundation::{
    AVCaptureAudioDataOutput, AVCaptureAudioDataOutputSampleBufferDelegate, AVCaptureConnection,
    AVCaptureDevice, AVCaptureDeviceInput, AVCaptureOutput, AVCaptureSession, AVMediaTypeAudio,
};
use objc2_core_audio_types::kAudioFormatLinearPCM;
use objc2_core_media::CMSampleBuffer;
use objc2_foundation::{NSDictionary, NSNumber, NSObject, NSString};
use screenshotter_phrase::Phrase;

use crate::mix::{CHANNELS, RATE};
use crate::Failure;

use super::booth::Booth;
use super::clock::Session;
use super::eye::samples;

/// A microphone that is listened to.
pub struct Ear {
    session: Session,
    _witness: Retained<Witness>,
}

/// Who listens, and what they are to the desk.
struct Listener {
    booth: Arc<Booth>,
    source: usize,
    session: Session,
}

define_class!(
    // SAFETY: NSObject has no requirements for those who inherit from it.
    #[unsafe(super(NSObject))]
    #[name = "JustScreenshotterEar"]
    #[ivars = Listener]
    /// The one to whom the system gives what the microphone hears.
    struct Witness;

    unsafe impl NSObjectProtocol for Witness {}

    unsafe impl AVCaptureAudioDataOutputSampleBufferDelegate for Witness {
        #[unsafe(method(captureOutput:didOutputSampleBuffer:fromConnection:))]
        fn given(
            &self,
            _output: &AVCaptureOutput,
            buffer: &CMSampleBuffer,
            _connection: &AVCaptureConnection,
        ) {
            let listener = self.ivars();
            if let Some(samples) = samples(buffer) {
                let moment = listener.session.moment(buffer);
                listener.booth.hear(listener.source, moment, &samples);
            }
        }
    }
);

impl Witness {
    fn new(listener: Listener) -> Retained<Self> {
        let this = Self::alloc().set_ivars(listener);
        // SAFETY: the initialiser of NSObject has no requirements.
        unsafe { msg_send![super(this), init] }
    }
}

impl Ear {
    /// Listens to the microphone the system uses at the moment.
    ///
    /// # Errors
    ///
    /// There is no microphone, or a person has not allowed to use it.
    pub fn open(booth: Arc<Booth>, source: usize, queue: &DispatchQueue) -> Result<Ear, Failure> {
        // SAFETY: the objects are alive, and the queue lives as long as the output keeps it.
        unsafe {
            let session = Session::new(AVCaptureSession::new());
            let voice = voice()?;
            if !session.canAddInput(&voice) {
                return Err(unheard("AVCaptureSession"));
            }
            session.addInput(&voice);
            let output = AVCaptureAudioDataOutput::new();
            output.setAudioSettings(Some(&plain()));
            let witness = Witness::new(Listener {
                booth,
                source,
                session: session.clone(),
            });
            output.setSampleBufferDelegate_queue(
                Some(ProtocolObject::from_ref(&*witness)),
                Some(queue),
            );
            if !session.canAddOutput(&output) {
                return Err(unheard("AVCaptureAudioDataOutput"));
            }
            session.addOutput(&output);
            session.startRunning();
            Ok(Ear {
                session,
                _witness: witness,
            })
        }
    }
}

impl Drop for Ear {
    /// Listens no more.
    fn drop(&mut self) {
        // SAFETY: the object is alive.
        unsafe { self.session.stopRunning() };
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

/// The sound as the desk wants it: two channels of 16-bit integers that take turns.
fn plain() -> Retained<NSDictionary<NSString, AnyObject>> {
    let keys = [
        "AVFormatIDKey",
        "AVSampleRateKey",
        "AVNumberOfChannelsKey",
        "AVLinearPCMBitDepthKey",
        "AVLinearPCMIsFloatKey",
        "AVLinearPCMIsBigEndianKey",
        "AVLinearPCMIsNonInterleaved",
    ]
    .map(NSString::from_str);
    let numbers = [
        kAudioFormatLinearPCM,
        u32::try_from(RATE).unwrap_or_default(),
        u32::try_from(CHANNELS).unwrap_or_default(),
        16,
        0,
        0,
        0,
    ]
    .map(NSNumber::new_u32);
    NSDictionary::from_slices(
        &keys.each_ref().map(|key| &**key),
        &numbers.each_ref().map(|number| {
            let object: &AnyObject = number.as_ref();
            object
        }),
    )
}
