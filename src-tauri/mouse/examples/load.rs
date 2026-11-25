//! Measures what the watch costs while the mouse moves:
//! `cargo run --release -p screenshotter-mouse --example load -- 1000 10`.
//! The numbers are messages of the mouse in a second, and seconds. The cursor stays where it is:
//! the movements are zero. The second line is the control, without the watch.

#[cfg(target_os = "windows")]
fn main() -> Result<(), screenshotter_mouse::Failure> {
    use screenshotter_mouse::{Chord, Watch};

    let numbers: Vec<u64> = std::env::args()
        .skip(1)
        .map(|word| word.parse())
        .collect::<Result<_, _>>()?;
    let [pace, seconds] = numbers[..] else {
        return Err("Two numbers are expected".into());
    };
    let watch = Watch::start(vec![Chord::Bare], |_| {})?;
    println!("with the watch: {:?}", windows::spent(pace, seconds));
    drop(watch);
    println!("without it: {:?}", windows::spent(pace, seconds));
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
#[allow(unsafe_code)]
mod windows {
    use std::mem::zeroed;
    use std::thread;
    use std::time::{Duration, Instant};

    use windows_sys::Win32::Foundation::FILETIME;
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, GetProcessTimes};
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{mouse_event, MOUSEEVENTF_MOVE};

    /// The processor time of this process while the mouse says something. The calls that make the
    /// mouse speak are in it too, in both measurements.
    pub fn spent(pace: u64, seconds: u64) -> (u64, Duration) {
        let before = processor();
        let start = Instant::now();
        let mut said = 0;
        while start.elapsed() < Duration::from_secs(seconds) {
            let due = u64::try_from(start.elapsed().as_millis()).unwrap_or(0) * pace / 1000;
            while said < due {
                // SAFETY: the call has no requirements.
                unsafe { mouse_event(MOUSEEVENTF_MOVE, 0, 0, 0, 0) };
                said += 1;
            }
            thread::sleep(Duration::from_millis(1));
        }
        (said, processor().saturating_sub(before))
    }

    fn processor() -> Duration {
        // SAFETY: there is a place for every answer.
        let (kernel, user) = unsafe {
            let mut times: [FILETIME; 4] = zeroed();
            let [created, ended, kernel, user] = &mut times;
            GetProcessTimes(GetCurrentProcess(), created, ended, kernel, user);
            (*kernel, *user)
        };
        let ticks =
            |time: FILETIME| (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime);
        Duration::from_nanos((ticks(kernel) + ticks(user)) * 100)
    }
}
