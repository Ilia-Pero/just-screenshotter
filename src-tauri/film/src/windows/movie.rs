//! The file with the film.

use std::fs;
use std::path::Path;
use std::slice;

use windows::core::{GUID, HSTRING};
use windows::Win32::Media::MediaFoundation::{
    eAVEncH264VProfile_High, IMFAttributes, IMFMediaBuffer, IMFMediaType, IMFSinkWriter,
    MFAudioFormat_AAC, MFAudioFormat_PCM, MFCreateAttributes, MFCreateMediaType,
    MFCreateMemoryBuffer, MFCreateSample, MFCreateSinkWriterFromURL, MFMediaType_Audio,
    MFMediaType_Video, MFNominalRange_16_235, MFTranscodeContainerType_FMPEG4, MFVideoFormat_H264,
    MFVideoFormat_NV12, MFVideoInterlace_Progressive, MFVideoPrimaries_BT709, MFVideoTransFunc_709,
    MFVideoTransferMatrix_BT709, MF_MT_AUDIO_AVG_BYTES_PER_SECOND, MF_MT_AUDIO_BITS_PER_SAMPLE,
    MF_MT_AUDIO_BLOCK_ALIGNMENT, MF_MT_AUDIO_NUM_CHANNELS, MF_MT_AUDIO_SAMPLES_PER_SECOND,
    MF_MT_AVG_BITRATE, MF_MT_FRAME_RATE, MF_MT_FRAME_SIZE, MF_MT_INTERLACE_MODE, MF_MT_MAJOR_TYPE,
    MF_MT_MPEG2_PROFILE, MF_MT_PIXEL_ASPECT_RATIO, MF_MT_SUBTYPE, MF_MT_TRANSFER_FUNCTION,
    MF_MT_VIDEO_NOMINAL_RANGE, MF_MT_VIDEO_PRIMARIES, MF_MT_YUV_MATRIX,
    MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, MF_TRANSCODE_CONTAINERTYPE,
};

use crate::mix::{CHANNELS, RATE};
use crate::nv12::length;
use crate::{Area, Failure};

use super::clock::ticks;

/// Pictures in a second. There may be fewer of them when nothing moves on the screen.
pub const PACE: u32 = 30;

/// Bytes in a second of compressed sound: 160 kilobits.
const VOLUME: u32 = 20_000;

/// The file that is being written: video in H.264 and sound in AAC. It is made of fragments, so
/// what has already been written is not lost if the computer is switched off.
pub struct Movie {
    writer: IMFSinkWriter,
    video: u32,
    audio: Option<u32>,
    size: usize,
}

// SAFETY: the sink writer is free-threaded; pictures and sound are written by different threads
// into different streams of it.
unsafe impl Send for Movie {}
// SAFETY: see above.
unsafe impl Sync for Movie {}

impl Movie {
    /// Creates the file. If it cannot be a film, an empty file does not stay.
    pub fn create(file: &Path, area: &Area, sound: bool) -> Result<Movie, Failure> {
        Self::begin(file, area, sound).inspect_err(|_| {
            if fs::metadata(file).is_ok_and(|empty| empty.len() == 0) {
                // It is not worth telling: the failure itself is told.
                let _ = fs::remove_file(file);
            }
        })
    }

    fn begin(file: &Path, area: &Area, sound: bool) -> Result<Movie, Failure> {
        // SAFETY: the calls get live objects and return owned ones.
        unsafe {
            let mut wishes = None;
            MFCreateAttributes(&raw mut wishes, 2)?;
            let wishes: IMFAttributes = wishes.ok_or("Media Foundation has given no attributes")?;
            wishes.SetUINT32(&MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, 1)?;
            wishes.SetGUID(
                &MF_TRANSCODE_CONTAINERTYPE,
                &MFTranscodeContainerType_FMPEG4,
            )?;
            let writer =
                MFCreateSinkWriterFromURL(&HSTRING::from(file.as_os_str()), None, &wishes)?;
            let video = writer.AddStream(&h264(area)?)?;
            writer.SetInputMediaType(video, &picture(area, &MFVideoFormat_NV12)?, None)?;
            let audio = if sound {
                let stream = writer.AddStream(&aac()?)?;
                writer.SetInputMediaType(stream, &pcm()?, None)?;
                Some(stream)
            } else {
                None
            };
            writer.BeginWriting()?;
            Ok(Movie {
                writer,
                video,
                audio,
                size: length(area.width as usize, area.height as usize),
            })
        }
    }

    /// A picture in NV12 drawn by the function.
    pub fn picture(
        &self,
        draw: impl FnOnce(&mut [u8]) -> Result<(), Failure>,
    ) -> Result<IMFMediaBuffer, Failure> {
        filled(self.size, draw)
    }

    /// Shows the picture from the moment on, for the given time, both in ticks.
    pub fn show(&self, picture: &IMFMediaBuffer, moment: i64, time: i64) -> Result<(), Failure> {
        self.write(self.video, picture, moment, time)
    }

    /// Plays the samples from the moment on, which is counted in moments of sound.
    pub fn play(&self, samples: &[i16], moment: u64) -> Result<(), Failure> {
        let Some(stream) = self.audio else {
            return Ok(());
        };
        let buffer = filled(samples.len() * 2, |out| {
            for (bytes, sample) in out.as_chunks_mut::<2>().0.iter_mut().zip(samples) {
                *bytes = sample.to_le_bytes();
            }
            Ok(())
        })?;
        let count = (samples.len() / CHANNELS) as u64;
        let start = ticks(moment);
        self.write(stream, &buffer, start, ticks(moment + count) - start)
    }

    /// Finishes the file.
    pub fn finish(&self) -> Result<(), Failure> {
        // SAFETY: the writer is alive.
        unsafe { self.writer.Finalize()? };
        Ok(())
    }

    fn write(
        &self,
        stream: u32,
        buffer: &IMFMediaBuffer,
        moment: i64,
        time: i64,
    ) -> Result<(), Failure> {
        // SAFETY: the calls get live objects.
        unsafe {
            let sample = MFCreateSample()?;
            sample.AddBuffer(buffer)?;
            sample.SetSampleTime(moment)?;
            sample.SetSampleDuration(time)?;
            self.writer.WriteSample(stream, &sample)?;
        }
        Ok(())
    }
}

/// A buffer of the given size filled by the function.
fn filled(
    size: usize,
    fill: impl FnOnce(&mut [u8]) -> Result<(), Failure>,
) -> Result<IMFMediaBuffer, Failure> {
    let bytes = u32::try_from(size)?;
    // SAFETY: while the buffer is locked, the pointer leads to at least `size` bytes of it, and
    // nobody else has the buffer yet.
    unsafe {
        let buffer = MFCreateMemoryBuffer(bytes)?;
        let mut start = std::ptr::null_mut();
        buffer.Lock(&raw mut start, None, None)?;
        let done = fill(slice::from_raw_parts_mut(start, size));
        buffer.Unlock()?;
        done?;
        buffer.SetCurrentLength(bytes)?;
        Ok(buffer)
    }
}

/// A pair of numbers the way Media Foundation keeps sizes and ratios.
fn pair(first: u32, second: u32) -> u64 {
    (u64::from(first) << 32) | u64::from(second)
}

/// The bits of video in a second: enough for text to stay sharp.
fn bitrate(area: &Area) -> u32 {
    let pixels = u64::from(area.width) * u64::from(area.height) * u64::from(PACE);
    u32::try_from((pixels * 8 / 100).clamp(1_000_000, 24_000_000)).unwrap_or(24_000_000)
}

fn picture(area: &Area, kind: &GUID) -> Result<IMFMediaType, Failure> {
    // SAFETY: the calls get live objects.
    unsafe {
        let media = MFCreateMediaType()?;
        media.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;
        media.SetGUID(&MF_MT_SUBTYPE, kind)?;
        media.SetUINT32(
            &MF_MT_INTERLACE_MODE,
            MFVideoInterlace_Progressive.0.unsigned_abs(),
        )?;
        media.SetUINT64(&MF_MT_FRAME_SIZE, pair(area.width, area.height))?;
        media.SetUINT64(&MF_MT_FRAME_RATE, pair(PACE, 1))?;
        media.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, pair(1, 1))?;
        media.SetUINT32(
            &MF_MT_VIDEO_PRIMARIES,
            MFVideoPrimaries_BT709.0.unsigned_abs(),
        )?;
        media.SetUINT32(
            &MF_MT_TRANSFER_FUNCTION,
            MFVideoTransFunc_709.0.unsigned_abs(),
        )?;
        media.SetUINT32(
            &MF_MT_YUV_MATRIX,
            MFVideoTransferMatrix_BT709.0.unsigned_abs(),
        )?;
        media.SetUINT32(
            &MF_MT_VIDEO_NOMINAL_RANGE,
            MFNominalRange_16_235.0.unsigned_abs(),
        )?;
        Ok(media)
    }
}

fn h264(area: &Area) -> Result<IMFMediaType, Failure> {
    let media = picture(area, &MFVideoFormat_H264)?;
    // SAFETY: the object is alive.
    unsafe {
        media.SetUINT32(&MF_MT_AVG_BITRATE, bitrate(area))?;
        media.SetUINT32(
            &MF_MT_MPEG2_PROFILE,
            eAVEncH264VProfile_High.0.unsigned_abs(),
        )?;
    }
    Ok(media)
}

fn sound(kind: &GUID) -> Result<IMFMediaType, Failure> {
    // SAFETY: the calls get live objects.
    unsafe {
        let media = MFCreateMediaType()?;
        media.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Audio)?;
        media.SetGUID(&MF_MT_SUBTYPE, kind)?;
        media.SetUINT32(&MF_MT_AUDIO_BITS_PER_SAMPLE, 16)?;
        media.SetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND, u32::try_from(RATE)?)?;
        media.SetUINT32(&MF_MT_AUDIO_NUM_CHANNELS, u32::try_from(CHANNELS)?)?;
        Ok(media)
    }
}

fn aac() -> Result<IMFMediaType, Failure> {
    let media = sound(&MFAudioFormat_AAC)?;
    // SAFETY: the object is alive.
    unsafe { media.SetUINT32(&MF_MT_AUDIO_AVG_BYTES_PER_SECOND, VOLUME)? };
    Ok(media)
}

fn pcm() -> Result<IMFMediaType, Failure> {
    let media = sound(&MFAudioFormat_PCM)?;
    let align = u32::try_from(CHANNELS * 2)?;
    // SAFETY: the object is alive.
    unsafe {
        media.SetUINT32(&MF_MT_AUDIO_BLOCK_ALIGNMENT, align)?;
        media.SetUINT32(
            &MF_MT_AUDIO_AVG_BYTES_PER_SECOND,
            u32::try_from(RATE)? * align,
        )?;
    }
    Ok(media)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use windows::Win32::Media::MediaFoundation::{MFStartup, MFSTARTUP_LITE, MF_VERSION};
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

    use super::{bitrate, Movie, PACE};
    use crate::mix::RATE;
    use crate::windows::clock::TICKS;
    use crate::Area;

    const AREA: Area = Area {
        left: 0,
        top: 0,
        width: 320,
        height: 240,
    };

    fn prepare() {
        // SAFETY: the calls change only this process and this thread.
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
            MFStartup(MF_VERSION, MFSTARTUP_LITE).unwrap();
        }
    }

    /// A second of a grey picture and, if asked for, of a tone: the bytes of the file.
    fn film(sound: bool) -> Vec<u8> {
        prepare();
        let folder = TempDir::new().unwrap();
        let file = folder.path().join("film.mp4");
        let movie = Movie::create(&file, &AREA, sound).unwrap();
        let grey = movie
            .picture(|out| {
                out.fill(128);
                Ok(())
            })
            .unwrap();
        let frame = TICKS / i64::from(PACE);
        for number in 0..i64::from(PACE) {
            movie.show(&grey, number * frame, frame).unwrap();
        }
        let tone: Vec<i16> = (0..RATE)
            .flat_map(|moment| [if moment % 100 < 50 { 9000 } else { -9000 }; 2])
            .collect();
        movie.play(&tone, 0).unwrap();
        movie.finish().unwrap();
        fs::read(file).unwrap()
    }

    fn holds(bytes: &[u8], word: &[u8]) -> bool {
        bytes.windows(word.len()).any(|part| part == word)
    }

    #[test]
    fn has_video_that_everything_plays() {
        assert!(holds(&film(false), b"avc1"));
    }

    #[test]
    fn has_sound_that_everything_plays() {
        assert!(holds(&film(true), b"mp4a"));
    }

    #[test]
    fn has_no_empty_track_when_nothing_is_heard() {
        assert!(!holds(&film(false), b"mp4a"));
    }

    #[test]
    fn is_made_of_fragments() {
        assert!(holds(&film(true), b"moof"));
    }

    #[test]
    fn leaves_no_empty_file_when_the_area_is_too_small() {
        let folder = TempDir::new().unwrap();
        let file = folder.path().join("film.mp4");
        let tiny = Area {
            width: 2,
            height: 2,
            ..AREA
        };
        prepare();
        Movie::create(&file, &tiny, false).err().unwrap();
        assert!(!file.exists());
    }

    #[test]
    fn gives_a_small_area_enough_bits() {
        assert_eq!(bitrate(&AREA), 1_000_000);
    }

    #[test]
    fn gives_a_large_area_more_bits() {
        assert_eq!(
            bitrate(&Area {
                width: 1920,
                height: 1080,
                ..AREA
            }),
            4_976_640
        );
    }
}
