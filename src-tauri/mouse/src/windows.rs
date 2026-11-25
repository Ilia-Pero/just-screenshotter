//! The watch on Windows: a hook that hears the mouse before other programs do.

#![allow(unsafe_code)]

use std::cell::RefCell;
use std::io;
use std::ptr;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use windows_sys::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Threading::{
    GetCurrentThread, GetCurrentThreadId, SetThreadPriority, THREAD_PRIORITY_HIGHEST,
};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VIRTUAL_KEY, VK_CONTROL, VK_SHIFT,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, PostThreadMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
    HC_ACTION, MSG, MSLLHOOKSTRUCT, WH_MOUSE_LL, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE,
    WM_QUIT, WM_RBUTTONDOWN, WM_RBUTTONUP,
};

use crate::pair::{Button, Pair, Verdict};
use crate::{Chord, Failure};

/// A watch for chords. Dropping it ends the watch.
pub struct Watch {
    number: u32,
    thread: Option<JoinHandle<()>>,
}

/// The one who hears the mouse.
struct Ear {
    pair: Pair,
    wanted: Vec<Chord>,
    ring: Box<dyn Fn(Chord)>,
}

thread_local! {
    /// A hook is told nothing but what the mouse says, so the ear lies in its thread.
    static EAR: RefCell<Option<Ear>> = const { RefCell::new(None) };
}

impl Watch {
    /// Starts watching. The bell rings in a thread of the watch, and the mouse waits until it has
    /// rung, so it has to be quick.
    ///
    /// # Errors
    ///
    /// The system does not let the program watch the mouse.
    pub fn start(
        wanted: Vec<Chord>,
        ring: impl Fn(Chord) + Send + 'static,
    ) -> Result<Watch, Failure> {
        let (told, heard) = mpsc::channel();
        let thread = thread::spawn(move || {
            EAR.set(Some(Ear {
                pair: Pair::default(),
                wanted,
                ring: Box::new(ring),
            }));
            listen(&told);
        });
        match heard.recv()? {
            Ok(number) => Ok(Watch {
                number,
                thread: Some(thread),
            }),
            Err(failure) => Err(failure.into()),
        }
    }
}

impl Drop for Watch {
    fn drop(&mut self) {
        // SAFETY: the thread is there until it is asked to quit. It may not have been: then it
        // has gone already, and there is nobody to ask.
        unsafe { PostThreadMessageW(self.number, WM_QUIT, 0, 0) };
        if let Some(thread) = self.thread.take() {
            // A crash of the thread ends the watch as well.
            let _ = thread.join();
        }
    }
}

/// Sets the hook and serves it until the thread is asked to quit.
fn listen(told: &mpsc::Sender<io::Result<u32>>) {
    // SAFETY: the hook lives in this thread, and its procedure lives as long as the process.
    unsafe {
        // The whole system waits for the hook, so it must not wait for a busy processor.
        SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_HIGHEST);
        let hook = SetWindowsHookExW(WH_MOUSE_LL, Some(hear), GetModuleHandleW(ptr::null()), 0);
        if hook.is_null() {
            // If nobody waits, there is nobody to tell.
            let _ = told.send(Err(io::Error::last_os_error()));
            return;
        }
        let _ = told.send(Ok(GetCurrentThreadId()));
        let mut message: MSG = std::mem::zeroed();
        while GetMessageW(&raw mut message, ptr::null_mut(), 0, 0) > 0 {}
        UnhookWindowsHookEx(hook);
    }
}

/// Hears what the mouse says.
unsafe extern "system" fn hear(code: i32, word: WPARAM, long: LPARAM) -> LRESULT {
    let said = u32::try_from(word).unwrap_or(WM_MOUSEMOVE);
    if code == HC_ACTION.cast_signed() && said != WM_MOUSEMOVE {
        // SAFETY: for this code the system gives what the mouse says.
        let moment = unsafe { (*(long as *const MSLLHOOKSTRUCT)).time };
        if judge(said, moment) != Verdict::Pass {
            return 1;
        }
    }
    // SAFETY: the arguments are those the system has given.
    unsafe { CallNextHookEx(ptr::null_mut(), code, word, long) }
}

fn judge(said: u32, moment: u32) -> Verdict {
    EAR.with_borrow_mut(|ear| {
        let Some(ear) = ear else {
            return Verdict::Pass;
        };
        match said {
            WM_LBUTTONDOWN => ear.press(Button::Left, moment),
            WM_RBUTTONDOWN => ear.press(Button::Right, moment),
            WM_LBUTTONUP => ear.pair.release(Button::Left),
            WM_RBUTTONUP => ear.pair.release(Button::Right),
            _ => Verdict::Pass,
        }
    })
}

impl Ear {
    fn press(&mut self, button: Button, moment: u32) -> Verdict {
        let chord = chord();
        let verdict = self
            .pair
            .press(button, moment, self.wanted.contains(&chord));
        if verdict == Verdict::Ring {
            (self.ring)(chord);
        }
        verdict
    }
}

/// The chord by the keys that are held now.
fn chord() -> Chord {
    if held(VK_SHIFT) {
        Chord::Shift
    } else if held(VK_CONTROL) {
        Chord::Control
    } else {
        Chord::Bare
    }
}

fn held(key: VIRTUAL_KEY) -> bool {
    // SAFETY: the call has no requirements. The sign of the answer tells that the key is down.
    unsafe { GetAsyncKeyState(i32::from(key)) < 0 }
}
