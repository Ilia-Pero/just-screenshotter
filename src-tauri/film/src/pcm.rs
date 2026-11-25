//! Sound as systems give it, turned into the sound of a film.

/// How numbers of sound lie in memory.
#[derive(Clone, Copy, Debug)]
pub struct Shape {
    /// How many channels there are.
    pub channels: usize,
    /// True if the numbers are fractions, false if they are 16-bit integers.
    pub float: bool,
    /// True if every channel lies apart, false if numbers of channels take turns.
    pub apart: bool,
}

impl Shape {
    /// The sound in two channels. One channel is heard in both, the third and the next ones are
    /// not heard.
    pub fn stereo(&self, bytes: &[u8]) -> Vec<i16> {
        let numbers = self.numbers(bytes);
        let channels = self.channels.max(1);
        let moments = numbers.len() / channels;
        let right = usize::from(channels > 1);
        (0..moments)
            .flat_map(|moment| {
                [0, right].map(|channel| {
                    numbers[if self.apart {
                        channel * moments + moment
                    } else {
                        moment * channels + channel
                    }]
                })
            })
            .collect()
    }

    fn numbers(&self, bytes: &[u8]) -> Vec<i16> {
        if self.float {
            bytes
                .as_chunks()
                .0
                .iter()
                .map(|four| level(f32::from_ne_bytes(*four)))
                .collect()
        } else {
            bytes
                .as_chunks()
                .0
                .iter()
                .map(|two| i16::from_ne_bytes(*two))
                .collect()
        }
    }
}

/// A fraction from minus one to one as an integer.
#[allow(clippy::cast_possible_truncation)]
fn level(fraction: f32) -> i16 {
    (fraction * 32_767.0).round() as i16
}

#[cfg(test)]
mod tests {
    use super::Shape;

    const APART: Shape = Shape {
        channels: 2,
        float: true,
        apart: true,
    };

    /// One channel of integers.
    const SINGLE: Shape = Shape {
        channels: 1,
        float: false,
        apart: false,
    };

    fn fractions(numbers: &[f32]) -> Vec<u8> {
        numbers
            .iter()
            .flat_map(|number| number.to_ne_bytes())
            .collect()
    }

    fn integers(numbers: &[i16]) -> Vec<u8> {
        numbers
            .iter()
            .flat_map(|number| number.to_ne_bytes())
            .collect()
    }

    #[test]
    fn puts_channels_that_lie_apart_together() {
        assert_eq!(
            APART.stereo(&fractions(&[0.0, 1.0, -1.0, 0.0])),
            [0, -32_767, 32_767, 0]
        );
    }

    #[test]
    fn keeps_channels_that_take_turns() {
        assert_eq!(
            Shape {
                channels: 2,
                ..SINGLE
            }
            .stereo(&integers(&[1, 2, 3, 4])),
            [1, 2, 3, 4]
        );
    }

    #[test]
    fn is_heard_in_both_ears_if_there_is_one_channel() {
        assert_eq!(SINGLE.stereo(&integers(&[5, 6])), [5, 5, 6, 6]);
    }

    #[test]
    fn takes_the_first_two_of_many_channels() {
        assert_eq!(
            Shape {
                channels: 3,
                ..SINGLE
            }
            .stereo(&integers(&[1, 2, 3, 4, 5, 6])),
            [1, 2, 4, 5]
        );
    }

    #[test]
    fn does_not_overflow_when_it_is_too_loud() {
        assert_eq!(APART.stereo(&fractions(&[2.0, -2.0])), [i16::MAX, i16::MIN]);
    }

    #[test]
    fn is_empty_without_numbers() {
        assert_eq!(APART.stereo(&[]).len(), 0);
    }
}
