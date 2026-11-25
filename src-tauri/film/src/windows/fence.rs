//! The fence around the area that is being filmed.

use std::sync::mpsc::sync_channel;
use std::thread::{self, JoinHandle};

use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::CreateSolidBrush;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW, PeekMessageW,
    PostThreadMessageW, RegisterClassW, SetLayeredWindowAttributes, SetWindowDisplayAffinity,
    ShowWindow, LWA_ALPHA, MSG, PM_NOREMOVE, SW_SHOWNOACTIVATE, WDA_EXCLUDEFROMCAPTURE, WM_QUIT,
    WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
    WS_POPUP,
};

use crate::{Area, Failure, Order};

/// The red of the fence: blue, green, red.
const RED: COLORREF = COLORREF(0x002A_33E5);

const CLASS: PCWSTR = w!("JustScreenshotterFence");

/// Four narrow windows around the area. The mouse goes through them, and they are not filmed.
pub struct Fence {
    thread: u32,
    keeper: Option<JoinHandle<()>>,
}

impl Fence {
    /// Puts up the fence around the area.
    pub fn put(order: &Order, area: Area) -> Result<Fence, Failure> {
        let boards = boards(order, area)?;
        let (bell, ready) = sync_channel(1);
        let keeper = thread::spawn(move || {
            // SAFETY: there is a place for a message, and the second function only returns
            // a number. The first one gives the thread a queue: without it the message to quit
            // would be lost.
            let number = unsafe {
                let mut message = MSG::default();
                let _ = PeekMessageW(&raw mut message, None, WM_QUIT, WM_QUIT, PM_NOREMOVE);
                GetCurrentThreadId()
            };
            let _ = bell.send(number);
            // Filming goes on even if the fence does not stand.
            let _ = keep(&boards);
        });
        Ok(Fence {
            thread: ready.recv()?,
            keeper: Some(keeper),
        })
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        // SAFETY: the message carries nothing. If the thread is gone, the call fails.
        let _ = unsafe { PostThreadMessageW(self.thread, WM_QUIT, WPARAM(0), LPARAM(0)) };
        if let Some(keeper) = self.keeper.take() {
            let _ = keeper.join();
        }
    }
}

/// Where the windows stand on the desktop: left, top, width, height.
fn boards(order: &Order, area: Area) -> Result<[(i32, i32, i32, i32); 4], Failure> {
    #[allow(clippy::cast_possible_truncation)]
    let thick = (order.scale * 2.0).round().clamp(2.0, 16.0) as i32;
    let left = order.monitor.left + area.left;
    let top = order.monitor.top + area.top;
    let width = i32::try_from(area.width)?;
    let height = i32::try_from(area.height)?;
    Ok([
        (left - thick, top - thick, width + 2 * thick, thick),
        (left - thick, top + height, width + 2 * thick, thick),
        (left - thick, top, thick, height),
        (left + width, top, thick, height),
    ])
}

/// Keeps the windows until the thread is told to quit.
fn keep(boards: &[(i32, i32, i32, i32)]) -> Result<(), Failure> {
    // SAFETY: the class and the windows belong to this thread, and all of them are gone when the
    // process ends. The strings are constants that end with zero.
    unsafe {
        let module = GetModuleHandleW(None)?;
        let class = WNDCLASSW {
            lpfnWndProc: Some(procedure),
            hInstance: module.into(),
            hbrBackground: CreateSolidBrush(RED),
            lpszClassName: CLASS,
            ..Default::default()
        };
        if RegisterClassW(&raw const class) == 0 {
            return Err(windows::core::Error::from_thread().into());
        }
        let mut windows = Vec::new();
        for (left, top, width, height) in boards.iter().copied() {
            let window = CreateWindowExW(
                WS_EX_TOPMOST
                    | WS_EX_TOOLWINDOW
                    | WS_EX_NOACTIVATE
                    | WS_EX_TRANSPARENT
                    | WS_EX_LAYERED,
                CLASS,
                w!(""),
                WS_POPUP,
                left,
                top,
                width,
                height,
                None,
                None,
                Some(module.into()),
                None,
            )?;
            SetLayeredWindowAttributes(window, COLORREF(0), 255, LWA_ALPHA)?;
            // Before Windows 10 2004 the fence may be seen on a film of the next area.
            let _ = SetWindowDisplayAffinity(window, WDA_EXCLUDEFROMCAPTURE);
            let _ = ShowWindow(window, SW_SHOWNOACTIVATE);
            windows.push(window);
        }
        let mut message = MSG::default();
        while GetMessageW(&raw mut message, None, 0, 0).as_bool() {
            DispatchMessageW(&raw const message);
        }
        for window in windows {
            DestroyWindow(window)?;
        }
    }
    Ok(())
}

extern "system" fn procedure(window: HWND, message: u32, first: WPARAM, second: LPARAM) -> LRESULT {
    // SAFETY: the parameters come from the system as they are.
    unsafe { DefWindowProcW(window, message, first, second) }
}
