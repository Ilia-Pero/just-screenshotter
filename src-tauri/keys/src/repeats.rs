//! Repetitions of a key that is held.

/// Tells a press of a key from its repetition: to repeat a key that is held, the server makes up
/// its release and its press at the same moment.
#[derive(Default)]
pub struct Repeats {
    /// The key that was released last, and when.
    released: Option<(u8, u32)>,
}

impl Repeats {
    /// The key is released at the moment.
    pub fn release(&mut self, code: u8, moment: u32) {
        self.released = Some((code, moment));
    }

    /// Whether a press of the key at the moment repeats the one before it.
    pub fn again(&self, code: u8, moment: u32) -> bool {
        self.released == Some((code, moment))
    }
}

#[cfg(test)]
mod tests {
    use super::Repeats;

    /// The key 10 was released at the moment 500.
    fn released() -> Repeats {
        let mut repeats = Repeats::default();
        repeats.release(10, 500);
        repeats
    }

    #[test]
    fn begin_with_a_press() {
        assert!(!Repeats::default().again(10, 500));
    }

    #[test]
    fn are_released_and_pressed_at_the_same_moment() {
        assert!(released().again(10, 500));
    }

    #[test]
    fn are_over_when_the_key_is_pressed_later() {
        assert!(!released().again(10, 620));
    }

    #[test]
    fn are_of_one_key() {
        assert!(!released().again(11, 500));
    }
}
