//! Moments by the clock of the computer, `RATE` of them in a second.

use objc2_core_media::{CMClock, CMTime, CMTimeRoundingMethod};

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

#[cfg(test)]
mod tests {
    use super::{moment, time};

    #[test]
    fn gives_the_same_moment_back() {
        assert_eq!(moment(time(12_345)), 12_345);
    }
}
