//! Films the screen for a while and makes sure that there is a film:
//! `cargo run -p screenshotter-film --example film -- 5 film.mp4 1920 1080 100 100 800 600`.
//! The numbers are seconds, the size of the monitor, and the area: left, top, width, height.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use screenshotter_film::{Area, Camera, Failure, Native, Order};

/// A film of a second is not smaller than this.
const SMALLEST: u64 = 2000;

fn main() -> Result<(), Failure> {
    let words: Vec<String> = env::args().skip(1).collect();
    let [seconds, file, numbers @ ..] = words.as_slice() else {
        return Err("Seconds, a file and six numbers are expected".into());
    };
    let numbers: Vec<u32> = numbers
        .iter()
        .map(|number| number.parse())
        .collect::<Result<_, _>>()?;
    let [wide, high, left, top, width, height] = numbers.as_slice() else {
        return Err("Six numbers are expected".into());
    };
    let order = Order {
        monitor: Area {
            left: 0,
            top: 0,
            width: *wide,
            height: *high,
        },
        scale: 1.0,
        area: Area {
            left: i32::try_from(*left)?,
            top: i32::try_from(*top)?,
            width: *width,
            height: *height,
        },
        sound: true,
        voice: false,
        file: PathBuf::from(file),
    };
    let time = Duration::from_secs(seconds.parse()?);
    let (bell, rung) = mpsc::channel();
    let take = Native.film(&order, bell.clone())?;
    println!("{:?}", take.ears());
    thread::spawn(move || {
        thread::sleep(time);
        bell.send(())
    });
    take.rest(&rung)?;
    take.finish()?;
    let size = fs::metadata(file)?.len();
    println!("{size} bytes");
    if size < SMALLEST {
        return Err("There is nothing in the film".into());
    }
    Ok(())
}
