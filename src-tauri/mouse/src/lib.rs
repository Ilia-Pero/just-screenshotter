//! Both buttons of the mouse pressed together.
//!
//! The system tells the program about every movement of the mouse while there is a watch, so
//! there is none until a person chooses to use the buttons.

use std::error::Error;

#[cfg(any(test, target_os = "windows"))]
mod pair;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::Watch;

/// A failure to watch. It travels between threads, hence `Send + Sync`.
pub type Failure = Box<dyn Error + Send + Sync>;

/// Both buttons, and the key that is held with them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chord {
    Bare,
    Shift,
    Control,
}

impl Chord {
    /// All the chords.
    pub const ALL: [Chord; 3] = [Chord::Bare, Chord::Shift, Chord::Control];

    /// The name of the chord in the settings.
    #[must_use]
    pub fn code(self) -> &'static str {
        match self {
            Chord::Bare => "both",
            Chord::Shift => "shift+both",
            Chord::Control => "ctrl+both",
        }
    }

    /// The chords with the names. A name nobody knows is left out.
    pub fn named<'a>(codes: impl Iterator<Item = &'a str>) -> Vec<Chord> {
        codes
            .filter_map(|code| Chord::ALL.into_iter().find(|chord| chord.code() == code))
            .collect()
    }
}

/// Whether the system lets the program watch the mouse.
#[must_use]
pub fn available() -> bool {
    cfg!(target_os = "windows")
}

/// A watch for chords. Dropping it ends the watch.
///
/// @todo #1 Only Windows has the watch. On macOS `NSEvent` tells about presses of buttons outside
///  the program without a permission, on X11 `XInput2` does: `XI_RawButtonPress` of the root
///  window. Neither tells about movements if it is not asked to. Wayland does not let a program
///  watch the mouse.
#[cfg(not(target_os = "windows"))]
pub struct Watch;

#[cfg(not(target_os = "windows"))]
impl Watch {
    /// Starts watching.
    ///
    /// # Errors
    ///
    /// The system does not let the program watch the mouse.
    pub fn start(_wanted: Vec<Chord>, _ring: impl Fn(Chord) + 'static) -> Result<Watch, Failure> {
        Err("This system does not let a program watch the mouse".into())
    }
}

#[cfg(test)]
mod tests {
    use super::Chord;

    #[test]
    fn is_found_by_its_name() {
        assert_eq!(
            Chord::named(["shift+both", "both"].into_iter()),
            [Chord::Shift, Chord::Bare]
        );
    }

    #[test]
    fn is_not_found_by_an_unknown_name() {
        assert_eq!(Chord::named(["", "wheel"].into_iter()).len(), 0);
    }
}
