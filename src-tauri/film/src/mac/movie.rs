//! A file of a film.

use std::ffi::c_void;
use std::ptr::{self, NonNull};
use std::sync::mpsc;
use std::time::Duration;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_av_foundation::{
    AVAssetWriter, AVAssetWriterInput, AVAssetWriterStatus, AVFileTypeQuickTimeMovie, AVMediaType,
    AVMediaTypeAudio, AVMediaTypeVideo, AVVideoAverageBitRateKey, AVVideoCodecKey,
    AVVideoCodecTypeH264, AVVideoCompressionPropertiesKey, AVVideoExpectedSourceFrameRateKey,
    AVVideoHeightKey, AVVideoMaxKeyFrameIntervalDurationKey, AVVideoWidthKey,
};
use objc2_core_audio_types::{
    kAudioFormatLinearPCM, kAudioFormatMPEG4AAC, kLinearPCMFormatFlagIsPacked,
    kLinearPCMFormatFlagIsSignedInteger, AudioStreamBasicDescription,
};
use objc2_core_foundation::CFRetained;
use objc2_core_media::{
    kCMBlockBufferAssureMemoryNowFlag, kCMTimeInvalid, CMAudioFormatDescription,
    CMAudioFormatDescriptionCreate, CMAudioSampleBufferCreateReadyWithPacketDescriptions,
    CMBlockBuffer, CMSampleBuffer, CMSampleTimingInfo, CMTime,
};
use objc2_foundation::{NSDictionary, NSNumber, NSString, NSURL};

use crate::desk::Tape;
use crate::mix::{CHANNELS, RATE};
use crate::{Area, Failure};

use super::clock::time;

/// Pictures in the form encoders take, brightness and colour apart.
pub const NV12: u32 = u32::from_be_bytes(*b"420v");

/// Pictures in a second.
pub const PACE: u32 = 30;

/// Encoders of video cards take pictures of this size, and not all of them take larger ones.
const LIMIT: (u32, u32) = (4096, 2304);

/// Seconds between pictures from which a film can be shown.
const ROOT: u32 = 2;

/// Bits of compressed sound in a second.
const SOUND: u32 = 160_000;

/// How long to wait for the file to be finished.
const PATIENCE: Duration = Duration::from_secs(30);

/// A picture of the screen.
pub struct Picture(CFRetained<CMSampleBuffer>);

// SAFETY: a picture does not change, and the count of its owners is atomic.
unsafe impl Send for Picture {}

impl Picture {
    /// The picture, if there is one: the system also tells that nothing has changed.
    pub fn new(buffer: &CMSampleBuffer) -> Option<Self> {
        // SAFETY: the buffer is alive.
        unsafe { buffer.image_buffer() }?;
        Some(Self(buffer.into()))
    }
}

/// A file that is being written.
pub struct Movie {
    writer: Retained<AVAssetWriter>,
    video: Retained<AVAssetWriterInput>,
    sound: Option<Retained<AVAssetWriterInput>>,
    format: CFRetained<CMAudioFormatDescription>,
}

// SAFETY: the objects may be used in any thread, and the desk lets one thread in at a time.
unsafe impl Send for Movie {}

impl Movie {
    /// Creates the file.
    ///
    /// # Errors
    ///
    /// The system does not write such films.
    pub fn create(file: &NSURL, area: &Area, heard: bool) -> Result<Self, Failure> {
        // SAFETY: the objects are alive, and the constants are strings that live as long as
        // the process.
        unsafe {
            let kind = AVFileTypeQuickTimeMovie.ok_or("macOS has no films")?;
            let writer = AVAssetWriter::assetWriterWithURL_fileType_error(file, kind)
                .map_err(|error| error.localizedDescription().to_string())?;
            // The file is made of fragments of a second, so what has already been written is not
            // lost if the computer is switched off.
            writer.setMovieFragmentInterval(CMTime::new(1, 1));
            let seen = seen(area)?;
            let video = entry(&writer, AVMediaTypeVideo, &seen)?;
            let sound = heard
                .then(|| entry(&writer, AVMediaTypeAudio, &listened()))
                .transpose()?;
            if !writer.startWriting() {
                return Err(fault(&writer));
            }
            Ok(Self {
                writer,
                video,
                sound,
                format: plain()?,
            })
        }
    }

    /// The sound the way the system takes it.
    fn portion(&self, samples: &[i16], moment: i64) -> Result<CFRetained<CMSampleBuffer>, Failure> {
        let bytes = size_of_val(samples);
        let mut block = ptr::null_mut();
        // SAFETY: the system allocates the memory itself and copies the samples there.
        let block = unsafe {
            check(CMBlockBuffer::create_with_memory_block(
                None,
                ptr::null_mut(),
                bytes,
                None,
                ptr::null(),
                0,
                bytes,
                kCMBlockBufferAssureMemoryNowFlag,
                NonNull::from(&mut block),
            ))?;
            let block = CFRetained::from_raw(NonNull::new(block).ok_or("There is no memory")?);
            let source = NonNull::from(samples).cast::<c_void>();
            check(CMBlockBuffer::replace_data_bytes(source, &block, 0, bytes))?;
            block
        };
        let mut buffer = ptr::null_mut();
        // SAFETY: the block holds as many moments as it is told.
        unsafe {
            check(CMAudioSampleBufferCreateReadyWithPacketDescriptions(
                None,
                &block,
                &self.format,
                isize::try_from(samples.len() / CHANNELS)?,
                time(moment),
                ptr::null(),
                NonNull::from(&mut buffer),
            ))?;
            Ok(CFRetained::from_raw(
                NonNull::new(buffer).ok_or("There is no sound")?,
            ))
        }
    }
}

impl Tape for Movie {
    type Picture = Picture;

    fn begin(&self, moment: i64) {
        // SAFETY: the writer has started, and it is begun once.
        unsafe { self.writer.startSessionAtSourceTime(time(moment)) };
    }

    fn show(&self, picture: &Picture, moment: i64) -> Result<bool, Failure> {
        // SAFETY: the objects are alive, and there is one timing for one picture.
        unsafe {
            if !self.video.isReadyForMoreMediaData() {
                return Ok(false);
            }
            let timing = CMSampleTimingInfo {
                duration: kCMTimeInvalid,
                presentationTimeStamp: time(moment),
                decodeTimeStamp: kCMTimeInvalid,
            };
            let mut copy = ptr::null_mut();
            check(CMSampleBuffer::create_copy_with_new_timing(
                None,
                &picture.0,
                1,
                &raw const timing,
                NonNull::from(&mut copy),
            ))?;
            let copy = CFRetained::from_raw(NonNull::new(copy).ok_or("There is no picture")?);
            if self.video.appendSampleBuffer(&copy) {
                Ok(true)
            } else {
                Err(fault(&self.writer))
            }
        }
    }

    fn roomy(&self) -> bool {
        self.sound
            .as_ref()
            // SAFETY: the object is alive.
            .is_some_and(|sound| unsafe { sound.isReadyForMoreMediaData() })
    }

    fn play(&self, samples: &[i16], moment: i64) -> Result<(), Failure> {
        let Some(sound) = &self.sound else {
            return Ok(());
        };
        let portion = self.portion(samples, moment)?;
        // SAFETY: the objects are alive.
        if unsafe { sound.appendSampleBuffer(&portion) } {
            Ok(())
        } else {
            Err(fault(&self.writer))
        }
    }

    fn finish(&self, moment: i64) -> Result<(), Failure> {
        let (bell, rung) = mpsc::channel();
        let finished = RcBlock::new(move || {
            // If nobody waits any more, there is nobody to tell.
            let _ = bell.send(());
        });
        // SAFETY: the objects are alive, nothing is written any more, and the block is kept by
        // the writer until it is called.
        unsafe {
            if self.writer.status() != AVAssetWriterStatus::Writing {
                return Err(fault(&self.writer));
            }
            self.video.markAsFinished();
            if let Some(sound) = &self.sound {
                sound.markAsFinished();
            }
            self.writer.endSessionAtSourceTime(time(moment));
            self.writer.finishWritingWithCompletionHandler(&finished);
        }
        rung.recv_timeout(PATIENCE)
            .map_err(|_| "macOS does not finish the file")?;
        // SAFETY: the object is alive.
        if unsafe { self.writer.status() } == AVAssetWriterStatus::Completed {
            Ok(())
        } else {
            Err(fault(&self.writer))
        }
    }

    fn erase(&self) {
        // SAFETY: the object is alive.
        unsafe { self.writer.cancelWriting() };
    }
}

/// What the writer complains of.
fn fault(writer: &AVAssetWriter) -> Failure {
    // SAFETY: the object is alive.
    unsafe { writer.error() }
        .map_or_else(
            || "macOS does not write the film".to_owned(),
            |error| error.localizedDescription().to_string(),
        )
        .into()
}

/// An entry of the file.
///
/// # Safety
///
/// The settings are such as the system knows.
unsafe fn entry(
    writer: &AVAssetWriter,
    kind: Option<&'static AVMediaType>,
    settings: &NSDictionary<NSString, AnyObject>,
) -> Result<Retained<AVAssetWriterInput>, Failure> {
    let kind = kind.ok_or("macOS has no such media")?;
    // SAFETY: it is promised above.
    unsafe {
        let input =
            AVAssetWriterInput::assetWriterInputWithMediaType_outputSettings(kind, Some(settings));
        input.setExpectsMediaDataInRealTime(true);
        if !writer.canAddInput(&input) {
            return Err("macOS does not write such films".into());
        }
        writer.addInput(&input);
        Ok(input)
    }
}

/// The settings of video.
///
/// # Safety
///
/// The constants are strings that live as long as the process.
unsafe fn seen(area: &Area) -> Result<Retained<NSDictionary<NSString, AnyObject>>, Failure> {
    const MISSING: &str = "macOS has no video";
    let (width, height) = area.shrunk(LIMIT);
    let number = NSNumber::new_u32;
    // SAFETY: it is promised above.
    unsafe {
        let compression = NSDictionary::from_retained_objects(
            &[
                AVVideoAverageBitRateKey.ok_or(MISSING)?,
                AVVideoExpectedSourceFrameRateKey.ok_or(MISSING)?,
                AVVideoMaxKeyFrameIntervalDurationKey.ok_or(MISSING)?,
            ],
            &[number(area.bits(PACE)), number(PACE), number(ROOT)],
        );
        let codec: &NSString = AVVideoCodecTypeH264.ok_or(MISSING)?;
        let (width, height) = (number(width), number(height));
        Ok(NSDictionary::from_slices(
            &[
                AVVideoCodecKey.ok_or(MISSING)?,
                AVVideoWidthKey.ok_or(MISSING)?,
                AVVideoHeightKey.ok_or(MISSING)?,
                AVVideoCompressionPropertiesKey.ok_or(MISSING)?,
            ],
            &[
                codec.as_ref(),
                width.as_ref(),
                height.as_ref(),
                compression.as_ref(),
            ],
        ))
    }
}

/// The size of pictures in the film of the area.
pub fn size(area: &Area) -> (u32, u32) {
    area.shrunk(LIMIT)
}

/// The settings of sound.
fn listened() -> Retained<NSDictionary<NSString, AnyObject>> {
    let keys = [
        "AVFormatIDKey",
        "AVSampleRateKey",
        "AVNumberOfChannelsKey",
        "AVEncoderBitRateKey",
    ]
    .map(NSString::from_str);
    let [format, rate, channels, bits] = [
        kAudioFormatMPEG4AAC,
        u32::try_from(RATE).unwrap_or_default(),
        u32::try_from(CHANNELS).unwrap_or_default(),
        SOUND,
    ]
    .map(NSNumber::new_u32);
    NSDictionary::from_slices(
        &keys.each_ref().map(|key| &**key),
        &[
            format.as_ref(),
            rate.as_ref(),
            channels.as_ref(),
            bits.as_ref(),
        ],
    )
}

/// The description of the sound that comes to the file.
fn plain() -> Result<CFRetained<CMAudioFormatDescription>, Failure> {
    let channels = u32::try_from(CHANNELS)?;
    let moment = channels * 2;
    let mut sound = AudioStreamBasicDescription {
        mSampleRate: f64::from(u32::try_from(RATE)?),
        mFormatID: kAudioFormatLinearPCM,
        mFormatFlags: kLinearPCMFormatFlagIsSignedInteger | kLinearPCMFormatFlagIsPacked,
        mBytesPerPacket: moment,
        mFramesPerPacket: 1,
        mBytesPerFrame: moment,
        mChannelsPerFrame: channels,
        mBitsPerChannel: 16,
        mReserved: 0,
    };
    let mut format = ptr::null();
    // SAFETY: the description outlives the call, and there is nothing else to describe.
    unsafe {
        check(CMAudioFormatDescriptionCreate(
            None,
            NonNull::from(&mut sound),
            0,
            ptr::null(),
            0,
            ptr::null(),
            None,
            NonNull::from(&mut format),
        ))?;
        let format = NonNull::new(format.cast_mut()).ok_or("macOS does not know the sound")?;
        Ok(CFRetained::from_raw(format))
    }
}

/// What the system answers, as a failure if it is one.
fn check(status: i32) -> Result<(), Failure> {
    if status == 0 {
        Ok(())
    } else {
        Err(format!("macOS does not write the film ({status})").into())
    }
}
