//! The screen of Windows: the area is copied through GDI, as the Print Screen key does.

#![allow(unsafe_code)]

use std::mem::{size_of, zeroed};
use std::ptr::null_mut;

use screenshotter_phrase::Phrase;
use windows_sys::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
    ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, CAPTUREBLT, DIB_RGB_COLORS,
    SRCCOPY,
};

use crate::{Failure, Patch, Pixels, Screen};

/// The screen of Windows.
pub struct Gdi;

impl Gdi {
    /// Pixels of the area in the order of Windows: blue, green, red, spare. Rows go from the top.
    fn bgra(patch: &Patch) -> Result<Vec<u8>, Failure> {
        let width = i32::try_from(patch.width)?;
        let height = i32::try_from(patch.height)?;
        let header = u32::try_from(size_of::<BITMAPINFOHEADER>())?;
        let mut pixels = vec![0_u8; patch.width as usize * patch.height as usize * 4];
        // SAFETY: all handles are obtained here and released before leaving; GDI functions reject
        // null handles, and then rows is zero. The buffer holds exactly width * height pixels
        // of 4 bytes each, as BITMAPINFO says.
        let (copied, rows) = unsafe {
            let screen = GetDC(null_mut());
            if screen.is_null() {
                return Err(refused());
            }
            let memory = CreateCompatibleDC(screen);
            let bitmap = CreateCompatibleBitmap(screen, width, height);
            let before = SelectObject(memory, bitmap);
            let copied = BitBlt(
                memory,
                0,
                0,
                width,
                height,
                screen,
                patch.left,
                patch.top,
                SRCCOPY | CAPTUREBLT,
            );
            SelectObject(memory, before);
            let mut info: BITMAPINFO = zeroed();
            info.bmiHeader.biSize = header;
            info.bmiHeader.biWidth = width;
            info.bmiHeader.biHeight = -height;
            info.bmiHeader.biPlanes = 1;
            info.bmiHeader.biBitCount = 32;
            info.bmiHeader.biCompression = BI_RGB;
            let rows = GetDIBits(
                screen,
                bitmap,
                0,
                patch.height,
                pixels.as_mut_ptr().cast(),
                &raw mut info,
                DIB_RGB_COLORS,
            );
            DeleteObject(bitmap);
            DeleteDC(memory);
            ReleaseDC(null_mut(), screen);
            (copied, rows)
        };
        if copied == 0 || rows == 0 {
            return Err(refused());
        }
        Ok(pixels)
    }
}

impl Screen for Gdi {
    fn pngs(&self, patches: &[Patch]) -> Result<Vec<Vec<u8>>, Failure> {
        patches
            .iter()
            .map(|patch| {
                Pixels {
                    width: patch.width,
                    height: patch.height,
                    rgb: Self::bgra(patch)?
                        .as_chunks::<4>()
                        .0
                        .iter()
                        .flat_map(|&[blue, green, red, _]| [red, green, blue])
                        .collect(),
                }
                .png()
            })
            .collect()
    }
}

/// Windows keeps the screen to itself while the session is locked or the remote desktop
/// is minimised.
fn refused() -> Failure {
    Phrase::new("error.locked").into()
}
