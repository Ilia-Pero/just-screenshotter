//! The eye that watches a monitor.

use std::slice;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::time::Duration;

use windows::core::{factory, Interface};
use windows::Foundation::TypedEventHandler;
use windows::Graphics::Capture::{
    Direct3D11CaptureFramePool, GraphicsCaptureItem, GraphicsCaptureSession,
};
use windows::Graphics::DirectX::Direct3D11::IDirect3DDevice;
use windows::Graphics::DirectX::DirectXPixelFormat;
use windows::Graphics::SizeInt32;
use windows::Win32::Foundation::{HMODULE, POINT};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP};
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D, D3D11_BOX,
    D3D11_CPU_ACCESS_READ, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_MAPPED_SUBRESOURCE,
    D3D11_MAP_READ, D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows::Win32::Graphics::Gdi::{MonitorFromPoint, MONITOR_DEFAULTTONEAREST};
use windows::Win32::System::WinRT::Direct3D11::{
    CreateDirect3D11DeviceFromDXGIDevice, IDirect3DDxgiInterfaceAccess,
};
use windows::Win32::System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop;

use crate::nv12::convert;
use crate::{Area, Failure, Order};

/// The eye: it sees the area of the monitor whenever something changes there.
pub struct Eye {
    context: ID3D11DeviceContext,
    pool: Direct3D11CaptureFramePool,
    session: GraphicsCaptureSession,
    size: SizeInt32,
    area: Area,
    /// The last picture of the area, where the processor can read it.
    page: ID3D11Texture2D,
    seen: Receiver<()>,
}

impl Eye {
    /// Opens the eye. Whoever has the second result can wake it up.
    pub fn open(order: &Order) -> Result<(Eye, SyncSender<()>), Failure> {
        let (device, context) = device()?;
        // SAFETY: the calls get live objects and return owned ones.
        let (item, screen) = unsafe {
            let monitor = MonitorFromPoint(
                POINT {
                    x: order.monitor.left + i32::try_from(order.monitor.width / 2)?,
                    y: order.monitor.top + i32::try_from(order.monitor.height / 2)?,
                },
                MONITOR_DEFAULTTONEAREST,
            );
            let item: GraphicsCaptureItem =
                factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?
                    .CreateForMonitor(monitor)?;
            let screen: IDirect3DDevice =
                CreateDirect3D11DeviceFromDXGIDevice(&device.cast::<IDXGIDevice>()?)?.cast()?;
            (item, screen)
        };
        let size = item.Size()?;
        let area = order
            .area
            .within(u32::try_from(size.Width)?, u32::try_from(size.Height)?);
        if area.blank() {
            return Err("There is nothing to film outside the screen".into());
        }
        let pool = Direct3D11CaptureFramePool::CreateFreeThreaded(
            &screen,
            DirectXPixelFormat::B8G8R8A8UIntNormalized,
            2,
            size,
        )?;
        let session = pool.CreateCaptureSession(&item)?;
        // Windows 10 always draws a border around the monitor, and there is nothing to do about it.
        let _ = session.SetIsBorderRequired(false);
        let (bell, seen) = sync_channel(1);
        let ring = bell.clone();
        pool.FrameArrived(&TypedEventHandler::new(move |_, _| {
            // If the bell has already rung, there is no need to ring again.
            let _ = ring.try_send(());
            Ok(())
        }))?;
        session.StartCapture()?;
        Ok((
            Eye {
                page: page(&device, &area)?,
                context,
                pool,
                session,
                size,
                area,
                seen,
            },
            bell,
        ))
    }

    /// The area it sees, with even sides.
    pub fn area(&self) -> Area {
        self.area
    }

    /// Waits until something changes on the screen, but not longer than given.
    pub fn wait(&self, patience: Duration) {
        // It does not matter which of the two has happened: what matters is what is seen next.
        let _ = self.seen.recv_timeout(patience);
    }

    /// Looks at the screen. True if it has seen something new.
    pub fn look(&self) -> Result<bool, Failure> {
        let mut fresh = false;
        // There is an error in place of a picture when there are no more of them.
        while let Ok(frame) = self.pool.TryGetNextFrame() {
            if frame.ContentSize()? != self.size {
                return Err("The size of the screen has changed".into());
            }
            let left = self.area.left.unsigned_abs();
            let top = self.area.top.unsigned_abs();
            let part = D3D11_BOX {
                left,
                top,
                front: 0,
                right: left + self.area.width,
                bottom: top + self.area.height,
                back: 1,
            };
            // SAFETY: both textures are alive, and the part lies within both of them.
            unsafe {
                let picture: ID3D11Texture2D = frame
                    .Surface()?
                    .cast::<IDirect3DDxgiInterfaceAccess>()?
                    .GetInterface()?;
                self.context.CopySubresourceRegion(
                    &self.page,
                    0,
                    0,
                    0,
                    0,
                    &picture,
                    0,
                    Some(&raw const part),
                );
            }
            frame.Close()?;
            fresh = true;
        }
        Ok(fresh)
    }

    /// Writes what was seen last as NV12.
    pub fn draw(&self, out: &mut [u8]) -> Result<(), Failure> {
        let width = self.area.width as usize;
        let height = self.area.height as usize;
        let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
        // SAFETY: the page is readable, and while it is mapped, every row but the last takes
        // `RowPitch` bytes, and the last one has at least `width` pixels of 4 bytes each.
        unsafe {
            self.context
                .Map(&self.page, 0, D3D11_MAP_READ, 0, Some(&raw mut mapped))?;
            let pitch = mapped.RowPitch as usize;
            let screen =
                slice::from_raw_parts(mapped.pData.cast(), pitch * (height - 1) + width * 4);
            convert(screen, pitch, width, out);
            self.context.Unmap(&self.page, 0);
        }
        Ok(())
    }
}

impl Drop for Eye {
    fn drop(&mut self) {
        // The process is about to end anyway.
        let _ = self.session.Close();
        let _ = self.pool.Close();
    }
}

/// The video card, or what stands in for it where there is none.
fn device() -> Result<(ID3D11Device, ID3D11DeviceContext), Failure> {
    for kind in [D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP] {
        let mut device = None;
        let mut context = None;
        // SAFETY: the function only writes both objects to the places it is given.
        let made = unsafe {
            D3D11CreateDevice(
                None,
                kind,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                None,
                D3D11_SDK_VERSION,
                Some(&raw mut device),
                None,
                Some(&raw mut context),
            )
        };
        if let (Ok(()), Some(device), Some(context)) = (made, device, context) {
            return Ok((device, context));
        }
    }
    Err("Windows has no Direct3D 11".into())
}

/// A picture of the area which the processor can read.
fn page(device: &ID3D11Device, area: &Area) -> Result<ID3D11Texture2D, Failure> {
    let kind = D3D11_TEXTURE2D_DESC {
        Width: area.width,
        Height: area.height,
        MipLevels: 1,
        ArraySize: 1,
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: D3D11_USAGE_STAGING,
        BindFlags: 0,
        CPUAccessFlags: D3D11_CPU_ACCESS_READ.0.unsigned_abs(),
        MiscFlags: 0,
    };
    let mut page = None;
    // SAFETY: the function only writes the texture to the place it is given.
    unsafe { device.CreateTexture2D(&raw const kind, None, Some(&raw mut page))? };
    page.ok_or_else(|| "Direct3D has given no texture".into())
}
