//! Presses both buttons of the mouse and makes sure that the watch rings and that other programs
//! do not hear the second button: `cargo run -p screenshotter-mouse --example chord`.
//! It clicks where the cursor is, so run it only on a machine nobody works at.

#[cfg(target_os = "windows")]
fn main() -> Result<(), screenshotter_mouse::Failure> {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use screenshotter_mouse::{Chord, Watch};
    use windows::{heard, press, Act};

    let pause = || thread::sleep(Duration::from_millis(60));
    let (bell, rung) = mpsc::channel();
    let watch = Watch::start(vec![Chord::Bare], move |chord| {
        // If nobody listens any more, the check is over.
        let _ = bell.send(chord);
    })?;
    press(Act::LeftDown);
    pause();
    press(Act::LeftUp);
    pause();
    if rung.try_recv().is_ok() {
        return Err("The watch has rung after one button".into());
    }
    press(Act::LeftDown);
    pause();
    press(Act::RightDown);
    pause();
    let second = heard();
    press(Act::RightUp);
    press(Act::LeftUp);
    pause();
    let chord = rung.try_recv().map_err(|_| "The watch has not rung")?;
    if second {
        return Err("Other programs hear the second button".into());
    }
    drop(watch);
    press(Act::RightDown);
    pause();
    let alone = heard();
    press(Act::RightUp);
    if !alone {
        return Err("Other programs do not hear the button after the watch is over".into());
    }
    println!("{chord:?}");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
#[allow(unsafe_code)]
mod windows {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        mouse_event, GetAsyncKeyState, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
        MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, VK_RBUTTON,
    };

    #[derive(Clone, Copy)]
    pub enum Act {
        LeftDown,
        LeftUp,
        RightDown,
        RightUp,
    }

    pub fn press(act: Act) {
        let said = match act {
            Act::LeftDown => MOUSEEVENTF_LEFTDOWN,
            Act::LeftUp => MOUSEEVENTF_LEFTUP,
            Act::RightDown => MOUSEEVENTF_RIGHTDOWN,
            Act::RightUp => MOUSEEVENTF_RIGHTUP,
        };
        // SAFETY: the call has no requirements.
        unsafe { mouse_event(said, 0, 0, 0, 0) };
    }

    /// Whether the system believes that the right button is down.
    pub fn heard() -> bool {
        // SAFETY: the call has no requirements. The sign of the answer tells that it is down.
        unsafe { GetAsyncKeyState(i32::from(VK_RBUTTON)) < 0 }
    }
}
