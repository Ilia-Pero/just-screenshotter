//! The clock that both the screen and sound follow.

use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

use crate::mix::RATE;
use crate::Failure;

/// Ticks of the clock in a second: one tick is 100 nanoseconds.
pub const TICKS: i64 = 10_000_000;

/// The moment now, in ticks.
pub fn now() -> Result<i64, Failure> {
    let mut count = 0;
    let mut rate = 0;
    // SAFETY: both functions only write a number to the place they are given.
    unsafe {
        QueryPerformanceCounter(&raw mut count)?;
        QueryPerformanceFrequency(&raw mut rate)?;
    }
    Ok(i64::try_from(
        i128::from(count) * i128::from(TICKS) / i128::from(rate.max(1)),
    )?)
}

/// How many moments of sound there are in the ticks.
pub fn moments(ticks: i64) -> u64 {
    u64::try_from(ticks).unwrap_or(0) * RATE / TICKS.unsigned_abs()
}

/// How many ticks the moments of sound take.
pub fn ticks(moments: u64) -> i64 {
    i64::try_from(moments * TICKS.unsigned_abs() / RATE).unwrap_or(i64::MAX)
}
