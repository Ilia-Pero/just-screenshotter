//! The ear that listens to the computer or to the microphone.

use std::ptr::null_mut;
use std::slice;

use windows::Win32::Media::Audio::{
    eCapture, eConsole, eRender, IAudioCaptureClient, IAudioClient, IMMDevice, IMMDeviceEnumerator,
    MMDeviceEnumerator, AUDCLNT_BUFFERFLAGS_SILENT, AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR,
    AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM, AUDCLNT_STREAMFLAGS_LOOPBACK,
    AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY, WAVEFORMATEX, WAVE_FORMAT_PCM,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};

use crate::mix::{CHANNELS, RATE};
use crate::Failure;

use super::clock::now;

/// How much sound the system keeps until it is taken: a fifth of a second.
const STOCK: i64 = 2_000_000;

/// What to listen to.
#[derive(Clone, Copy)]
pub enum Source {
    /// What is heard from the computer.
    Sound,
    /// The microphone.
    Voice,
}

/// A device that is being listened to.
struct Device {
    name: String,
    client: IAudioClient,
    capture: IAudioCaptureClient,
}

/// The ear: it listens to the device the system uses at the moment.
pub struct Ear {
    source: Source,
    device: Option<Device>,
}

// SAFETY: audio clients are free-threaded: in the multithreaded apartment, where all threads of
// the filming process live, any of them may call the objects.
unsafe impl Send for Ear {}

impl Ear {
    /// Starts listening.
    pub fn open(source: Source) -> Result<Ear, Failure> {
        Ok(Ear {
            source,
            device: Some(Device::open(source)?),
        })
    }

    /// Hands over what has been heard since the last time: the moment, in ticks, and the samples.
    /// If the device is gone, there is nothing to hear until another one comes.
    pub fn hear(&mut self, mut tell: impl FnMut(i64, &[i16])) {
        if let Some(device) = &self.device {
            if device.hear(&mut tell).is_err() {
                self.device = None;
            }
        }
    }

    /// Turns to another device if the system has turned to it, for example because headphones
    /// were plugged in.
    pub fn turn(&mut self) {
        let Ok(name) = endpoint(self.source).and_then(|device| name(&device)) else {
            return;
        };
        if self
            .device
            .as_ref()
            .is_none_or(|device| device.name != name)
        {
            self.device = Device::open(self.source).ok();
        }
    }
}

impl Device {
    fn open(source: Source) -> Result<Device, Failure> {
        let device = endpoint(source)?;
        let loopback = match source {
            Source::Sound => AUDCLNT_STREAMFLAGS_LOOPBACK,
            Source::Voice => 0,
        };
        let align = u16::try_from(CHANNELS * 2)?;
        let rate = u32::try_from(RATE)?;
        let format = WAVEFORMATEX {
            wFormatTag: u16::try_from(WAVE_FORMAT_PCM)?,
            nChannels: u16::try_from(CHANNELS)?,
            nSamplesPerSec: rate,
            nAvgBytesPerSec: rate * u32::from(align),
            nBlockAlign: align,
            wBitsPerSample: 16,
            cbSize: 0,
        };
        // SAFETY: the calls get live objects and return owned ones; the format outlives the call.
        unsafe {
            let client: IAudioClient = device.Activate(CLSCTX_ALL, None)?;
            client.Initialize(
                AUDCLNT_SHAREMODE_SHARED,
                loopback
                    | AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM
                    | AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY,
                STOCK,
                0,
                &raw const format,
                None,
            )?;
            let capture: IAudioCaptureClient = client.GetService()?;
            client.Start()?;
            Ok(Device {
                name: name(&device)?,
                client,
                capture,
            })
        }
    }

    fn hear(&self, tell: &mut impl FnMut(i64, &[i16])) -> Result<(), Failure> {
        // SAFETY: between GetBuffer and ReleaseBuffer the data holds `count` moments, each of
        // which has two samples of two bytes, in the format the client was given.
        unsafe {
            while self.capture.GetNextPacketSize()? > 0 {
                let mut data = null_mut();
                let mut count = 0;
                let mut flags = 0;
                let mut moment = 0;
                self.capture.GetBuffer(
                    &raw mut data,
                    &raw mut count,
                    &raw mut flags,
                    None,
                    Some(&raw mut moment),
                )?;
                let samples = count as usize * CHANNELS;
                let vague = flags & AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR.0.unsigned_abs() != 0;
                let start = if vague {
                    now()?
                } else {
                    i64::try_from(moment)?
                };
                if flags & AUDCLNT_BUFFERFLAGS_SILENT.0.unsigned_abs() != 0 || data.is_null() {
                    tell(start, &vec![0; samples]);
                } else {
                    tell(start, slice::from_raw_parts(data.cast(), samples));
                }
                self.capture.ReleaseBuffer(count)?;
            }
        }
        Ok(())
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        // SAFETY: the client is alive. If it does not stop, it is released anyway.
        let _ = unsafe { self.client.Stop() };
    }
}

/// The device the system uses at the moment.
fn endpoint(source: Source) -> Result<IMMDevice, Failure> {
    let flow = match source {
        Source::Sound => eRender,
        Source::Voice => eCapture,
    };
    // SAFETY: the calls return owned objects.
    unsafe {
        let devices: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        Ok(devices.GetDefaultAudioEndpoint(flow, eConsole)?)
    }
}

fn name(device: &IMMDevice) -> Result<String, Failure> {
    // SAFETY: the system gives a string that ends with zero. The process is too short-lived for
    // the few bytes of its name to be worth returning.
    Ok(unsafe { device.GetId()?.to_string()? })
}
