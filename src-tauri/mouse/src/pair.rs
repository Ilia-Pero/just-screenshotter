//! What two buttons of the mouse are doing.

/// How far apart two presses may be and still be taken for one, in milliseconds.
const TOGETHER: u32 = 150;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Button {
    Left,
    Right,
}

/// What to do with what the mouse says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Other programs hear it.
    Pass,
    /// Other programs do not hear it.
    Keep,
    /// Other programs do not hear it: both buttons are pressed.
    Ring,
}

/// Two buttons. Other programs hear the first one as usual, because nobody knows yet that the
/// second one will follow, and they do not hear the second one.
#[derive(Default)]
pub struct Pair {
    /// The button that was pressed first, and when.
    held: Option<(Button, u32)>,
    /// The button other programs do not hear.
    kept: Option<Button>,
}

impl Pair {
    /// A button is pressed at the moment. Both buttons matter only if they are wanted.
    pub fn press(&mut self, button: Button, moment: u32, wanted: bool) -> Verdict {
        match self.held {
            Some((first, since))
                if wanted && first != button && moment.wrapping_sub(since) <= TOGETHER =>
            {
                self.kept = Some(button);
                Verdict::Ring
            }
            _ => {
                self.held = Some((button, moment));
                Verdict::Pass
            }
        }
    }

    /// A button is released.
    pub fn release(&mut self, button: Button) -> Verdict {
        if self.kept == Some(button) {
            self.kept = None;
            return Verdict::Keep;
        }
        if self.held.is_some_and(|(held, _)| held == button) {
            self.held = None;
        }
        Verdict::Pass
    }
}

#[cfg(test)]
mod tests {
    use super::{Button, Pair, Verdict, TOGETHER};

    /// The pair whose left button was pressed at the moment 100.
    fn held() -> Pair {
        let mut pair = Pair::default();
        pair.press(Button::Left, 100, true);
        pair
    }

    /// The pair whose both buttons are pressed.
    fn rung() -> Pair {
        let mut pair = held();
        pair.press(Button::Right, 120, true);
        pair
    }

    #[test]
    fn lets_one_button_pass() {
        assert_eq!(Pair::default().press(Button::Left, 0, true), Verdict::Pass);
    }

    #[test]
    fn rings_when_the_second_button_follows() {
        assert_eq!(held().press(Button::Right, 120, true), Verdict::Ring);
    }

    #[test]
    fn rings_when_the_right_button_is_the_first() {
        let mut pair = Pair::default();
        pair.press(Button::Right, 100, true);
        assert_eq!(pair.press(Button::Left, 100, true), Verdict::Ring);
    }

    #[test]
    fn does_not_ring_when_the_second_button_is_late() {
        assert_eq!(
            held().press(Button::Right, 101 + TOGETHER, true),
            Verdict::Pass
        );
    }

    #[test]
    fn does_not_ring_when_it_is_not_wanted() {
        assert_eq!(held().press(Button::Right, 120, false), Verdict::Pass);
    }

    #[test]
    fn does_not_ring_after_a_click() {
        let mut pair = held();
        pair.release(Button::Left);
        assert_eq!(pair.press(Button::Right, 120, true), Verdict::Pass);
    }

    #[test]
    fn rings_when_the_clock_begins_anew() {
        let mut pair = Pair::default();
        pair.press(Button::Left, u32::MAX, true);
        assert_eq!(pair.press(Button::Right, 10, true), Verdict::Ring);
    }

    #[test]
    fn keeps_the_release_of_the_second_button() {
        assert_eq!(rung().release(Button::Right), Verdict::Keep);
    }

    #[test]
    fn lets_the_release_of_the_first_button_pass() {
        assert_eq!(rung().release(Button::Left), Verdict::Pass);
    }

    #[test]
    fn rings_again() {
        let mut pair = rung();
        pair.release(Button::Right);
        pair.release(Button::Left);
        pair.press(Button::Left, 300, true);
        assert_eq!(pair.press(Button::Right, 310, true), Verdict::Ring);
    }
}
