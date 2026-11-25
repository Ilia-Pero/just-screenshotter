//! Filming that goes on.

use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use dispatch2::{DispatchQueue, DispatchQueueAttr};
use objc2::rc::Retained;
use objc2::runtime::{NSObjectProtocol, ProtocolObject};
use objc2_foundation::{NSActivityOptions, NSProcessInfo, NSString};

use crate::desk::Desk;
use crate::mix::RATE;
use crate::{Ears, Failure, Order, Take};

use super::booth::Booth;
use super::clock::now;
use super::ear::Ear;
use super::eye::Eye;
use super::lens::Lens;
use super::movie::Movie;
use super::place::url;

/// How late a picture or sound may come and still get into the film: 0.3 of a second.
#[allow(clippy::cast_possible_wrap)]
const LAG: i64 = RATE as i64 * 3 / 10;

/// How often to write down what has come.
const SIP: Duration = Duration::from_millis(100);

/// How long to wait for the first picture.
const PATIENCE: Duration = Duration::from_secs(10);

/// What looks at the screen until it is dropped.
pub trait Sight {}

impl Sight for Eye {}
impl Sight for Lens {}

/// Opens what looks at the screen.
pub type Opened = fn(&Order, Arc<Booth>, &DispatchQueue) -> Result<Box<dyn Sight>, Failure>;

/// Looks through the eye. It also hears the computer.
pub fn eye(
    order: &Order,
    booth: Arc<Booth>,
    queue: &DispatchQueue,
) -> Result<Box<dyn Sight>, Failure> {
    Ok(Box::new(Eye::open(order, booth, queue)?))
}

/// Looks through the lens.
pub fn lens(
    order: &Order,
    booth: Arc<Booth>,
    queue: &DispatchQueue,
) -> Result<Box<dyn Sight>, Failure> {
    Ok(Box::new(Lens::open(order, booth, queue)?))
}

/// Filming that goes on.
struct Shoot {
    sight: Box<dyn Sight>,
    ear: Option<Ear>,
    booth: Arc<Booth>,
    ears: Ears,
    /// While it is here, the screen does not go out.
    _awake: Retained<ProtocolObject<dyn NSObjectProtocol>>,
}

/// Starts filming.
///
/// # Errors
///
/// The system does not show the screen, or the microphone cannot be used.
pub fn film(order: &Order, bell: Sender<()>, opened: Opened) -> Result<Box<dyn Take>, Failure> {
    let area = order.area.within(order.monitor.width, order.monitor.height);
    let order = Order {
        area,
        ..order.clone()
    };
    let file = url(&order.file)?;
    let sources = usize::from(order.sound) + usize::from(order.voice);
    let movie = Movie::create(&file, &area, sources > 0)?;
    let booth = Arc::new(Booth::new(Desk::new(movie, sources), bell));
    let shoot = open(&order, &booth, opened);
    if shoot.is_err() {
        // What has gone wrong is known already.
        let _ = booth.finish(now());
    }
    shoot
}

/// Opens what sees and hears, and waits for the first picture.
fn open(order: &Order, booth: &Arc<Booth>, opened: Opened) -> Result<Box<dyn Take>, Failure> {
    let queue = DispatchQueue::new("just-screenshotter.film", DispatchQueueAttr::SERIAL);
    let ear = order
        .voice
        .then(|| Ear::open(Arc::clone(booth), usize::from(order.sound), &queue))
        .transpose()?;
    let sight = opened(order, Arc::clone(booth), &queue)?;
    // The sound comes later than the first picture does, and by now it comes.
    booth.start(now());
    let shoot = Shoot {
        sight,
        ear,
        booth: Arc::clone(booth),
        ears: Ears {
            sound: order.sound,
            voice: order.voice,
        },
        _awake: NSProcessInfo::processInfo().beginActivityWithOptions_reason(
            NSActivityOptions::UserInitiated | NSActivityOptions::IdleDisplaySleepDisabled,
            &NSString::from_str("The screen is being filmed"),
        ),
    };
    let asked = Instant::now();
    while !booth.began() {
        if asked.elapsed() > PATIENCE {
            return Err("macOS does not show the screen".into());
        }
        thread::sleep(SIP);
    }
    Ok(Box::new(shoot))
}

impl Take for Shoot {
    fn ears(&self) -> Ears {
        self.ears
    }

    fn rest(&self, bell: &Receiver<()>) -> Result<(), Failure> {
        while bell.recv_timeout(SIP) == Err(RecvTimeoutError::Timeout) {
            self.booth.turn(now() - LAG);
        }
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Failure> {
        let end = now();
        let Shoot {
            sight, ear, booth, ..
        } = *self;
        drop((sight, ear));
        booth.finish(end)
    }
}
