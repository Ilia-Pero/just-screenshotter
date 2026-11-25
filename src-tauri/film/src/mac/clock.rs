//! Moments by the clock of the computer, `RATE` of them in a second.

use std::ops::Deref;

use objc2::rc::Retained;
use objc2::{msg_send, sel};
use objc2_av_foundation::AVCaptureSession;
use objc2_core_foundation::CFRetained;
use objc2_core_media::{CMClock, CMSampleBuffer, CMSyncConvertTime, CMTime, CMTimeRoundingMethod};
use objc2_foundation::NSObjectProtocol;

use crate::mix::RATE;

/// Moments in a second, the way the system wants them.
#[allow(clippy::cast_possible_truncation)]
pub const SCALE: i32 = RATE as i32;

/// The moment now.
pub fn now() -> i64 {
    // SAFETY: the clock of the computer is always there.
    moment(unsafe { CMClock::host_time_clock().time() })
}

/// The moment of a time.
pub fn moment(time: CMTime) -> i64 {
    // SAFETY: the calculation has no requirements.
    let scaled =
        unsafe { time.convert_scale(SCALE, CMTimeRoundingMethod::RoundTowardNegativeInfinity) };
    scaled.value
}

/// The time of a moment.
pub fn time(moment: i64) -> CMTime {
    // SAFETY: the scale is not zero.
    unsafe { CMTime::new(moment, SCALE) }
}

/// A session of `AVFoundation`, which has a clock of its own.
#[derive(Clone)]
pub struct Session(Retained<AVCaptureSession>);

// SAFETY: in other threads the session is asked only for its clock.
unsafe impl Send for Session {}
// SAFETY: see above.
unsafe impl Sync for Session {}

impl Session {
    pub fn new(session: Retained<AVCaptureSession>) -> Self {
        Self(session)
    }

    /// The moment of the buffer. The film goes by the clock of the computer.
    pub fn moment(&self, buffer: &CMSampleBuffer) -> i64 {
        // SAFETY: the buffer and the clocks are alive.
        moment(unsafe {
            let own = buffer.presentation_time_stamp();
            self.clock().map_or(own, |clock| {
                let computer = CMClock::host_time_clock();
                CMSyncConvertTime(own, (*clock).as_ref(), (*computer).as_ref())
            })
        })
    }

    /// The clock of the session. Before macOS 12.3 it had another name.
    fn clock(&self) -> Option<CFRetained<CMClock>> {
        // SAFETY: each of the messages is sent to a system that knows it, and gives a clock.
        unsafe {
            let clock: *mut CMClock = if self.0.respondsToSelector(sel!(synchronizationClock)) {
                msg_send![&*self.0, synchronizationClock]
            } else {
                msg_send![&*self.0, masterClock]
            };
            clock.as_ref().map(CFRetained::from)
        }
    }
}

impl Deref for Session {
    type Target = AVCaptureSession;

    fn deref(&self) -> &AVCaptureSession {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{moment, time};

    #[test]
    fn gives_the_same_moment_back() {
        assert_eq!(moment(time(12_345)), 12_345);
    }
}
