//! Sounds of several sources put together.

use std::collections::VecDeque;

/// Moments of sound in a second.
pub const RATE: u64 = 48_000;

/// Numbers in one moment: the left and the right channel.
pub const CHANNELS: usize = 2;

/// How far a portion of sound may be from its place and still be taken for the next one:
/// 20 milliseconds. Clocks of devices differ, and it must not be heard.
const SLACK: u64 = RATE / 50;

/// The sound of one source that is not mixed yet.
#[derive(Default)]
struct Track {
    samples: VecDeque<i16>,
}

/// Sounds of several sources on a common scale of time. The scale is counted in moments from
/// the beginning of the film. Where a source is silent, there is silence.
pub struct Mix {
    tracks: Vec<Track>,
    /// How many moments are mixed already.
    ready: u64,
}

impl Mix {
    /// The mix of the given number of sources.
    pub fn new(sources: usize) -> Self {
        Self {
            tracks: (0..sources).map(|_| Track::default()).collect(),
            ready: 0,
        }
    }

    /// Puts down what the source has said starting with the moment.
    pub fn put(&mut self, source: usize, start: u64, samples: &[i16]) {
        let Some(track) = self.tracks.get_mut(source) else {
            return;
        };
        let end = self.ready + (track.samples.len() / CHANNELS) as u64;
        let mut late = 0;
        if start > end + SLACK {
            let gap = usize::try_from(start - end).unwrap_or(0) * CHANNELS;
            track.samples.extend(std::iter::repeat_n(0, gap));
        } else if start + SLACK < end {
            late = usize::try_from(end - start).unwrap_or(0) * CHANNELS;
        }
        track.samples.extend(samples.iter().skip(late));
    }

    /// Takes the mixed sound up to the moment.
    pub fn take(&mut self, until: u64) -> Vec<i16> {
        let moments = usize::try_from(until.saturating_sub(self.ready)).unwrap_or(0);
        let mut mixed = vec![0_i16; moments * CHANNELS];
        for track in &mut self.tracks {
            let heard = track.samples.len().min(mixed.len());
            for (sum, sample) in mixed.iter_mut().zip(track.samples.drain(..heard)) {
                *sum = sum.saturating_add(sample);
            }
        }
        self.ready += moments as u64;
        mixed
    }
}

#[cfg(test)]
mod tests {
    use super::{Mix, SLACK};

    /// The same number in both channels, for the given number of moments.
    fn tone(level: i16, moments: usize) -> Vec<i16> {
        vec![level; moments * 2]
    }

    #[test]
    fn is_silent_without_sources() {
        assert_eq!(Mix::new(0).take(3), tone(0, 3));
    }

    #[test]
    fn gives_back_what_was_put() {
        let mut mix = Mix::new(1);
        mix.put(0, 0, &tone(7, 3));
        assert_eq!(mix.take(3), tone(7, 3));
    }

    #[test]
    fn adds_sources_together() {
        let mut mix = Mix::new(2);
        mix.put(0, 0, &tone(7, 3));
        mix.put(1, 0, &tone(5, 3));
        assert_eq!(mix.take(3), tone(12, 3));
    }

    #[test]
    fn does_not_overflow_when_both_sources_are_loud() {
        let mut mix = Mix::new(2);
        mix.put(0, 0, &tone(30_000, 1));
        mix.put(1, 0, &tone(30_000, 1));
        assert_eq!(mix.take(1), tone(i16::MAX, 1));
    }

    #[test]
    fn fills_the_end_with_silence() {
        let mut mix = Mix::new(1);
        mix.put(0, 0, &tone(7, 1));
        assert_eq!(mix.take(2), [tone(7, 1), tone(0, 1)].concat());
    }

    #[test]
    fn fills_a_pause_with_silence() {
        let mut mix = Mix::new(1);
        mix.put(0, 0, &tone(7, 1));
        mix.put(0, 1 + SLACK + 1, &tone(7, 1));
        assert_eq!(mix.take(SLACK + 3)[2..4], [0, 0]);
    }

    #[test]
    fn puts_sound_after_a_pause_in_its_place() {
        let mut mix = Mix::new(1);
        mix.put(0, 0, &tone(7, 1));
        mix.put(0, 1 + SLACK + 1, &tone(9, 1));
        assert!(mix.take(SLACK + 3).ends_with(&tone(9, 1)));
    }

    #[test]
    fn joins_portions_that_are_almost_in_their_place() {
        let mut mix = Mix::new(1);
        mix.put(0, 0, &tone(7, 1));
        mix.put(0, 5, &tone(9, 1));
        assert_eq!(mix.take(2), [tone(7, 1), tone(9, 1)].concat());
    }

    #[test]
    fn drops_what_came_too_late() {
        let mut mix = Mix::new(1);
        mix.take(SLACK + 10);
        mix.put(0, 0, &tone(7, 10));
        assert_eq!(mix.take(SLACK + 11), tone(0, 1));
    }

    #[test]
    fn continues_where_it_stopped() {
        let mut mix = Mix::new(1);
        mix.put(0, 0, &[tone(7, 1), tone(9, 1)].concat());
        mix.take(1);
        assert_eq!(mix.take(2), tone(9, 1));
    }

    #[test]
    fn gives_nothing_for_a_moment_that_has_passed() {
        let mut mix = Mix::new(1);
        mix.take(5);
        assert_eq!(mix.take(3), tone(0, 0));
    }
}
