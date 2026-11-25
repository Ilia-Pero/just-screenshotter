//! Where what is seen and what is heard come together.

use crate::mix::{Mix, CHANNELS, RATE};
use crate::Failure;

/// How often a picture is repeated when nothing moves on the screen: twice a second.
#[allow(clippy::cast_possible_wrap)]
const STILL: i64 = RATE as i64 / 2;

/// What a film is written on. Moments are counted by the clock of the computer, `RATE` of them
/// in a second.
pub trait Tape {
    type Picture;

    /// Begins the film at the moment.
    fn begin(&self, moment: i64);

    /// Writes down a picture that is seen since the moment. False if there is no room for it now.
    ///
    /// # Errors
    ///
    /// The film cannot be written any more.
    fn show(&self, picture: &Self::Picture, moment: i64) -> Result<bool, Failure>;

    /// Whether there is room for sound now.
    fn roomy(&self) -> bool;

    /// Writes down sound that is heard since the moment.
    ///
    /// # Errors
    ///
    /// The film cannot be written any more.
    fn play(&self, samples: &[i16], moment: i64) -> Result<(), Failure>;

    /// Finishes the film at the moment.
    ///
    /// # Errors
    ///
    /// The film cannot be finished.
    fn finish(&self, moment: i64) -> Result<(), Failure>;

    /// Leaves nothing of the film.
    fn erase(&self);
}

/// A desk with a tape. The film begins with the first picture.
pub struct Desk<T: Tape> {
    tape: T,
    sources: usize,
    mix: Mix,
    /// When the film began.
    origin: Option<i64>,
    /// The last picture and its moment.
    shown: Option<(T::Picture, i64)>,
    /// How many moments of sound are written down.
    written: u64,
}

impl<T: Tape> Desk<T> {
    /// A desk for the given number of sources of sound.
    pub fn new(tape: T, sources: usize) -> Self {
        Self {
            tape,
            sources,
            mix: Mix::new(sources),
            origin: None,
            shown: None,
            written: 0,
        }
    }

    /// Whether the film has begun.
    pub fn began(&self) -> bool {
        self.origin.is_some()
    }

    /// Takes a picture. A picture older than the last one is left out.
    ///
    /// # Errors
    ///
    /// The film cannot be written any more.
    pub fn see(&mut self, picture: T::Picture, moment: i64) -> Result<(), Failure> {
        if self.origin.is_none() {
            self.tape.begin(moment);
            self.origin = Some(moment);
        }
        let fresh = self.shown.as_ref().is_none_or(|(_, last)| moment > *last);
        if fresh && self.tape.show(&picture, moment)? {
            self.shown = Some((picture, moment));
        }
        Ok(())
    }

    /// Takes what a source has said since the moment. What was said before the film is left out.
    pub fn hear(&mut self, source: usize, moment: i64, samples: &[i16]) {
        let Some(origin) = self.origin else { return };
        let early = usize::try_from(origin - moment)
            .unwrap_or(0)
            .saturating_mul(CHANNELS)
            .min(samples.len());
        let start = u64::try_from(moment - origin).unwrap_or(0);
        self.mix.put(source, start, &samples[early..]);
    }

    /// Writes down the sound up to the moment, and the picture once more if nothing has moved for
    /// long.
    ///
    /// # Errors
    ///
    /// The film cannot be written any more.
    pub fn turn(&mut self, until: i64) -> Result<(), Failure> {
        if let Some((picture, last)) = &mut self.shown {
            if until - *last >= STILL && self.tape.show(picture, until)? {
                *last = until;
            }
        }
        if let Some(origin) = self
            .origin
            .filter(|_| self.sources > 0 && self.tape.roomy())
        {
            let mixed = self.mix.take(u64::try_from(until - origin).unwrap_or(0));
            if !mixed.is_empty() {
                let start = origin + i64::try_from(self.written)?;
                self.tape.play(&mixed, start)?;
                self.written += (mixed.len() / CHANNELS) as u64;
            }
        }
        Ok(())
    }

    /// Finishes the film at the moment. If it has not begun, nothing is left of it.
    ///
    /// # Errors
    ///
    /// The film cannot be finished.
    pub fn finish(&mut self, moment: i64) -> Result<(), Failure> {
        if !self.began() {
            self.tape.erase();
            return Ok(());
        }
        let turned = self.turn(moment);
        let shown = match &self.shown {
            Some((picture, last)) if moment > *last => self.tape.show(picture, moment).map(drop),
            _ => Ok(()),
        };
        let finished = self.tape.finish(moment);
        turned.and(shown).and(finished)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use super::{Desk, Tape, STILL};
    use crate::Failure;

    /// What is written on the tape, line by line.
    #[derive(Clone, Default)]
    struct Lines {
        lines: Rc<RefCell<Vec<String>>>,
        full: Rc<Cell<bool>>,
    }

    impl Lines {
        fn write(&self, line: String) {
            self.lines.borrow_mut().push(line);
        }

        /// What is written after the first pair of lines.
        fn rest(&self) -> Vec<String> {
            self.lines.borrow()[2..].to_vec()
        }
    }

    impl Tape for Lines {
        type Picture = &'static str;

        fn begin(&self, moment: i64) {
            self.write(format!("begin {moment}"));
        }

        fn show(&self, picture: &&'static str, moment: i64) -> Result<bool, Failure> {
            if !self.full.get() {
                self.write(format!("{picture} {moment}"));
            }
            Ok(!self.full.get())
        }

        fn roomy(&self) -> bool {
            !self.full.get()
        }

        fn play(&self, samples: &[i16], moment: i64) -> Result<(), Failure> {
            self.write(format!("{samples:?} {moment}"));
            Ok(())
        }

        fn finish(&self, moment: i64) -> Result<(), Failure> {
            self.write(format!("end {moment}"));
            Ok(())
        }

        fn erase(&self) {
            self.write("nothing".to_owned());
        }
    }

    /// The tape, and the desk for the sources that has seen the sky at the moment 100.
    fn begun(sources: usize) -> (Lines, Desk<Lines>) {
        let lines = Lines::default();
        let mut desk = Desk::new(lines.clone(), sources);
        desk.see("sky", 100).unwrap();
        (lines, desk)
    }

    #[test]
    fn begins_with_the_first_picture() {
        assert_eq!(*begun(1).0.lines.borrow(), ["begin 100", "sky 100"]);
    }

    #[test]
    fn has_not_begun_without_a_picture() {
        assert!(!Desk::new(Lines::default(), 1).began());
    }

    #[test]
    fn begins_once() {
        let (lines, mut desk) = begun(1);
        desk.see("sea", 200).unwrap();
        assert_eq!(lines.rest(), ["sea 200"]);
    }

    #[test]
    fn leaves_out_a_picture_that_is_late() {
        let (lines, mut desk) = begun(1);
        desk.see("sea", 100).unwrap();
        assert_eq!(lines.rest().len(), 0);
    }

    #[test]
    fn leaves_out_sound_before_the_first_picture() {
        let lines = Lines::default();
        let mut desk = Desk::new(lines.clone(), 1);
        desk.hear(0, 100, &[7, 7]);
        desk.see("sky", 100).unwrap();
        desk.turn(101).unwrap();
        assert_eq!(lines.rest(), ["[0, 0] 100"]);
    }

    #[test]
    fn leaves_out_the_beginning_of_sound_that_began_earlier() {
        let (lines, mut desk) = begun(1);
        desk.hear(0, 99, &[1, 2, 3, 4]);
        desk.turn(101).unwrap();
        assert_eq!(lines.rest(), ["[3, 4] 100"]);
    }

    #[test]
    fn writes_sound_portion_by_portion() {
        let (lines, mut desk) = begun(1);
        desk.hear(0, 100, &[1, 2, 3, 4]);
        desk.turn(101).unwrap();
        desk.turn(102).unwrap();
        assert_eq!(lines.rest(), ["[1, 2] 100", "[3, 4] 101"]);
    }

    #[test]
    fn keeps_sound_until_there_is_room() {
        let (lines, mut desk) = begun(1);
        desk.hear(0, 100, &[1, 2, 3, 4]);
        lines.full.set(true);
        desk.turn(101).unwrap();
        lines.full.set(false);
        desk.turn(102).unwrap();
        assert_eq!(lines.rest(), ["[1, 2, 3, 4] 100"]);
    }

    #[test]
    fn writes_no_sound_without_sources() {
        let (lines, mut desk) = begun(0);
        desk.turn(101).unwrap();
        assert_eq!(lines.rest().len(), 0);
    }

    #[test]
    fn repeats_the_picture_when_nothing_moves() {
        let (lines, mut desk) = begun(0);
        desk.turn(100 + STILL).unwrap();
        assert_eq!(lines.rest(), ["sky 24100"]);
    }

    #[test]
    fn does_not_repeat_the_picture_too_often() {
        let (lines, mut desk) = begun(0);
        desk.turn(100 + STILL).unwrap();
        desk.turn(101 + STILL).unwrap();
        assert_eq!(lines.rest().len(), 1);
    }

    #[test]
    fn leaves_nothing_if_it_has_not_begun() {
        let lines = Lines::default();
        Desk::new(lines.clone(), 1).finish(5).unwrap();
        assert_eq!(*lines.lines.borrow(), ["nothing"]);
    }

    #[test]
    fn ends_with_the_last_picture() {
        let (lines, mut desk) = begun(0);
        desk.finish(109).unwrap();
        assert_eq!(lines.rest(), ["sky 109", "end 109"]);
    }

    #[test]
    fn ends_with_the_last_sound() {
        let (lines, mut desk) = begun(1);
        desk.hear(0, 100, &[1, 2]);
        desk.finish(101).unwrap();
        assert_eq!(lines.rest(), ["[1, 2] 100", "sky 101", "end 101"]);
    }
}
