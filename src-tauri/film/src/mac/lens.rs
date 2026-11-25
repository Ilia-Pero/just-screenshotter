//! What the screen shows, by `AVFoundation`. It can do so since macOS 10.7, and it does not hear
//! the computer.

use std::sync::Arc;

use dispatch2::DispatchQueue;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObjectProtocol, ProtocolObject};
use objc2::{define_class, msg_send, AllocAnyThread, DefinedClass};
use objc2_av_foundation::{
    AVCaptureConnection, AVCaptureOutput, AVCaptureScreenInput, AVCaptureSession,
    AVCaptureVideoDataOutput, AVCaptureVideoDataOutputSampleBufferDelegate,
};
use objc2_core_media::{CMSampleBuffer, CMTime};
use objc2_foundation::{NSDictionary, NSNumber, NSObject, NSString};
use screenshotter_phrase::Phrase;

use crate::{Failure, Order};

use super::booth::Booth;
use super::clock::Session;
use super::movie::{Picture, NV12, PACE};
use super::place::{display, from_bottom};

/// A lens that looks at the screen.
pub struct Lens {
    session: Session,
    _witness: Retained<Witness>,
}

/// Who looks.
struct Looker {
    booth: Arc<Booth>,
    session: Session,
}

define_class!(
    // SAFETY: NSObject has no requirements for those who inherit from it.
    #[unsafe(super(NSObject))]
    #[name = "JustScreenshotterLens"]
    #[ivars = Looker]
    /// The one to whom the system gives what the lens sees.
    struct Witness;

    unsafe impl NSObjectProtocol for Witness {}

    unsafe impl AVCaptureVideoDataOutputSampleBufferDelegate for Witness {
        #[unsafe(method(captureOutput:didOutputSampleBuffer:fromConnection:))]
        fn given(
            &self,
            _output: &AVCaptureOutput,
            buffer: &CMSampleBuffer,
            _connection: &AVCaptureConnection,
        ) {
            let looker = self.ivars();
            if let Some(picture) = Picture::new(buffer) {
                looker.booth.see(picture, looker.session.moment(buffer));
            }
        }
    }
);

impl Witness {
    fn new(looker: Looker) -> Retained<Self> {
        let this = Self::alloc().set_ivars(looker);
        // SAFETY: the initialiser of NSObject has no requirements.
        unsafe { msg_send![super(this), init] }
    }
}

impl Lens {
    /// Looks at the area of the screen.
    ///
    /// # Errors
    ///
    /// A person has not allowed to record the screen.
    pub fn open(order: &Order, booth: Arc<Booth>, queue: &DispatchQueue) -> Result<Lens, Failure> {
        let display = display(order)?;
        // SAFETY: the objects are alive, the queue lives as long as the output keeps it, and the
        // constants of time have a scale that is not zero.
        unsafe {
            let session = AVCaptureSession::new();
            let screen =
                AVCaptureScreenInput::initWithDisplayID(AVCaptureScreenInput::alloc(), display)
                    .ok_or_else(|| Phrase::new("error.permission"))?;
            screen.setCropRect(from_bottom(&order.monitor, &order.area, order.scale));
            screen.setMinFrameDuration(CMTime::new(1, i32::try_from(PACE)?));
            screen.setCapturesCursor(true);
            if !session.canAddInput(&screen) {
                return Err(Phrase::new("error.permission").into());
            }
            session.addInput(&screen);
            let output = AVCaptureVideoDataOutput::new();
            let key = NSString::from_str("PixelFormatType");
            let form = NSNumber::new_u32(NV12);
            let form: &AnyObject = form.as_ref();
            output.setVideoSettings(Some(&NSDictionary::from_slices(&[&*key], &[form])));
            output.setAlwaysDiscardsLateVideoFrames(true);
            let session = Session::new(session);
            let witness = Witness::new(Looker {
                booth,
                session: session.clone(),
            });
            output.setSampleBufferDelegate_queue(
                Some(ProtocolObject::from_ref(&*witness)),
                Some(queue),
            );
            if !session.canAddOutput(&output) {
                return Err("macOS does not give pictures of the screen".into());
            }
            session.addOutput(&output);
            session.startRunning();
            Ok(Lens {
                session,
                _witness: witness,
            })
        }
    }
}

impl Drop for Lens {
    /// Looks no more.
    fn drop(&mut self) {
        // SAFETY: the object is alive.
        unsafe { self.session.stopRunning() };
    }
}
