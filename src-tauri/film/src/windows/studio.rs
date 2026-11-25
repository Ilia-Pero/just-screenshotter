//! Everything that takes part in filming.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender, SyncSender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use screenshotter_phrase::Phrase;
use windows::core::w;
use windows::Win32::Media::MediaFoundation::{
    IMFMediaBuffer, MFStartup, MFSTARTUP_LITE, MF_VERSION,
};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
use windows::Win32::System::LibraryLoader::{LoadLibraryExW, LOAD_LIBRARY_SEARCH_SYSTEM32};
use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
};
use windows::Win32::UI::HiDpi::{
    SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
};

use crate::mix::Mix;
use crate::{Camera, Ears, Failure, Order, Take};

use super::clock::{moments, now, TICKS};
use super::ear::{Ear, Source};
use super::eye::Eye;
use super::fence::Fence;
use super::movie::{Movie, PACE};

/// The time of one picture, in ticks.
const FRAME: i64 = TICKS / PACE as i64;

/// How often a picture is repeated when nothing moves on the screen: twice a second.
const STILL: i64 = TICKS / 2;

/// How long to wait for the first picture.
const PATIENCE: i64 = TICKS * 5;

/// How late sound may come and still be heard: a tenth of a second.
const LAG: i64 = TICKS / 10;

/// How often to take the sound.
const SIP: Duration = Duration::from_millis(20);

/// How many times the sound is taken between checks of the device: about two seconds.
const SIPS: u32 = 100;

/// The camera of Windows.
///
/// @todo #1 Pictures are turned into NV12 by the processor. Where there is a video card, let it
///  do so: `VideoProcessorBlt` into a texture, and the texture through `MFCreateDXGISurfaceBuffer`
///  to the encoder. Measure the processor time before and after on a machine with a video card.
pub struct Studio;

/// Filming that goes on in two threads: one sees, the other hears.
struct Shoot {
    ears: Ears,
    stop: Arc<AtomicBool>,
    wake: SyncSender<()>,
    movie: Arc<Movie>,
    threads: Vec<JoinHandle<Result<(), Failure>>>,
    _fence: Option<Fence>,
}

impl Camera for Studio {
    fn film(&self, order: &Order, bell: Sender<()>) -> Result<Box<dyn Take>, Failure> {
        prepare()?;
        let (eye, wake) = Eye::open(order)?;
        let area = eye.area();
        let first = order.sound.then(|| Ear::open(Source::Sound).ok()).flatten();
        let second = order
            .voice
            .then(|| Ear::open(Source::Voice))
            .transpose()
            .map_err(|failure| Phrase::new("error.voice").with("reason", failure))?;
        let ears = Ears {
            sound: first.is_some(),
            voice: second.is_some(),
        };
        let ears_open: Vec<Ear> = first.into_iter().chain(second).collect();
        wait(&eye)?;
        let movie = Movie::create(&order.file, &area, !ears_open.is_empty())
            .inspect_err(|_| order.discard())?;
        let movie = Arc::new(movie);
        let stop = Arc::new(AtomicBool::new(false));
        let origin = now()?;
        let mut threads = vec![started(&bell, {
            let movie = Arc::clone(&movie);
            let stop = Arc::clone(&stop);
            move || see(&Watch { eye, movie, origin }, &stop)
        })];
        if !ears_open.is_empty() {
            threads.push(started(&bell, {
                let movie = Arc::clone(&movie);
                let stop = Arc::clone(&stop);
                move || hear(ears_open, &movie, origin, &stop)
            }));
        }
        Ok(Box::new(Shoot {
            ears,
            stop,
            wake,
            movie,
            threads,
            _fence: Fence::put(order, area).ok(),
        }))
    }
}

impl Take for Shoot {
    fn ears(&self) -> Ears {
        self.ears
    }

    /// The file is finished even after a failure: what has been filmed is not lost.
    fn finish(self: Box<Self>) -> Result<(), Failure> {
        self.stop.store(true, Ordering::Relaxed);
        // If the eye is awake, there is no need to wake it.
        let _ = self.wake.try_send(());
        let mut outcome = Ok(());
        for thread in self.threads {
            let ended = thread
                .join()
                .unwrap_or_else(|_| Err("A thread of filming has crashed".into()));
            outcome = outcome.and(ended);
        }
        let finished = self.movie.finish();
        outcome.and(finished)
    }
}

/// Gets the process ready for filming.
fn prepare() -> Result<(), Failure> {
    // SAFETY: the calls change only this process and this thread.
    unsafe {
        // Windows N comes without Media Foundation. Its libraries are loaded on the first call,
        // and without them the call would crash the process.
        for library in [w!("mfplat.dll"), w!("mfreadwrite.dll"), w!("d3d11.dll")] {
            LoadLibraryExW(library, None, LOAD_LIBRARY_SEARCH_SYSTEM32)
                .map_err(|_| Phrase::new("error.media"))?;
        }
        // It may be set already, and then there is nothing to do.
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        MFStartup(MF_VERSION, MFSTARTUP_LITE)?;
        // The screen must not go out while it is being filmed.
        SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED);
    }
    Ok(())
}

/// A thread that rings the bell when it fails.
fn started(
    bell: &Sender<()>,
    work: impl FnOnce() -> Result<(), Failure> + Send + 'static,
) -> JoinHandle<Result<(), Failure>> {
    let bell = bell.clone();
    thread::spawn(move || {
        // SAFETY: the call changes only this thread.
        let worked = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }
            .ok()
            .map_err(Failure::from)
            .and_then(|()| work());
        if worked.is_err() {
            // If nobody listens to the bell, filming is being finished already.
            let _ = bell.send(());
        }
        worked
    })
}

/// Waits until the eye has seen the first picture.
fn wait(eye: &Eye) -> Result<(), Failure> {
    let start = now()?;
    while now()? - start < PATIENCE {
        eye.wait(Duration::from_millis(100));
        if eye.look()? {
            return Ok(());
        }
    }
    Err(Phrase::new("error.locked").into())
}

/// What the seeing thread works with.
struct Watch {
    eye: Eye,
    movie: Arc<Movie>,
    origin: i64,
}

// SAFETY: the objects of Direct3D and of the capture are free-threaded, and only one thread has
// the eye.
unsafe impl Send for Watch {}

/// Films the screen: a picture for every change, but not more often than the pace allows, and
/// a repeated one when nothing moves.
fn see(watch: &Watch, stop: &AtomicBool) -> Result<(), Failure> {
    let draw = || watch.movie.picture(|out| watch.eye.draw(out));
    let show = |picture: &IMFMediaBuffer, moment: i64| {
        watch.movie.show(picture, moment - watch.origin, FRAME)
    };
    let mut picture = draw()?;
    let mut last = watch.origin;
    let mut fresh = false;
    show(&picture, last)?;
    while !stop.load(Ordering::Relaxed) {
        let due = last + if fresh { FRAME } else { STILL };
        let rest = due - now()?;
        if rest > 0 {
            watch
                .eye
                .wait(Duration::from_nanos(rest.unsigned_abs() * 100));
        }
        fresh |= watch.eye.look()?;
        let moment = now()?;
        if moment - last >= if fresh { FRAME } else { STILL } {
            if fresh {
                picture = draw()?;
                fresh = false;
            }
            show(&picture, moment)?;
            last = moment;
        }
    }
    let moment = now()?.max(last + FRAME);
    show(&picture, moment)
}

/// Records sound: takes it from all the ears, mixes it and writes it down.
fn hear(mut ears: Vec<Ear>, movie: &Movie, origin: i64, stop: &AtomicBool) -> Result<(), Failure> {
    let mut mix = Mix::new(ears.len());
    let mut written = 0;
    for sip in 1.. {
        let last = stop.load(Ordering::Relaxed);
        for (source, ear) in ears.iter_mut().enumerate() {
            ear.hear(|moment, samples| {
                let early = usize::try_from(moments(origin - moment)).unwrap_or(usize::MAX);
                let skipped = early.saturating_mul(2).min(samples.len());
                mix.put(source, moments(moment - origin), &samples[skipped..]);
            });
            if sip % SIPS == 0 {
                ear.turn();
            }
        }
        let until = moments(now()? - origin - if last { 0 } else { LAG });
        let mixed = mix.take(until);
        if !mixed.is_empty() {
            movie.play(&mixed, written)?;
            written = until;
        }
        if last {
            break;
        }
        thread::sleep(SIP);
    }
    Ok(())
}
