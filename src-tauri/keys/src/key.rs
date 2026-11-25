//! A key the way X11 knows it.

/// Keys that are held, as bits of X11.
const SHIFT: u16 = 1;
const CONTROL: u16 = 1 << 2;
const ALT: u16 = 1 << 3;
const SUPER: u16 = 1 << 6;

/// Keys with names, and what X11 calls them.
const NAMED: [(&str, u32); 25] = [
    ("PrintScreen", 0xff61),
    ("Pause", 0xff13),
    ("ScrollLock", 0xff14),
    ("Insert", 0xff63),
    ("Delete", 0xffff),
    ("Home", 0xff50),
    ("End", 0xff57),
    ("PageUp", 0xff55),
    ("PageDown", 0xff56),
    ("ArrowLeft", 0xff51),
    ("ArrowUp", 0xff52),
    ("ArrowRight", 0xff53),
    ("ArrowDown", 0xff54),
    ("Space", 0x20),
    ("Backquote", 0x60),
    ("Minus", 0x2d),
    ("Equal", 0x3d),
    ("Comma", 0x2c),
    ("Period", 0x2e),
    ("Slash", 0x2f),
    ("Semicolon", 0x3b),
    ("Quote", 0x27),
    ("Backslash", 0x5c),
    ("BracketLeft", 0x5b),
    ("BracketRight", 0x5d),
];

/// The first function key.
const F1: u32 = 0xffbe;

/// The zero of the numeric pad.
const PAD: u32 = 0xffb0;

/// A key and the ones that are held with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key {
    /// The keys that are held, as bits.
    pub held: u16,
    /// The name of the key in X11.
    pub symbol: u32,
}

impl Key {
    /// The key by its spelling in the settings, such as `Ctrl+Shift+PrintScreen`. Nothing if
    /// there is no such key.
    #[must_use]
    pub fn spelled(words: &str) -> Option<Key> {
        let mut held = 0;
        let mut symbol = None;
        for word in words.split('+') {
            match word {
                "Shift" => held |= SHIFT,
                "Ctrl" => held |= CONTROL,
                "Alt" => held |= ALT,
                "Cmd" => held |= SUPER,
                name => symbol = Some(named(name)?),
            }
        }
        symbol.map(|symbol| Key { held, symbol })
    }
}

fn named(name: &str) -> Option<u32> {
    let mut letters = name.chars();
    match (letters.next(), letters.next()) {
        (Some(letter), None) if letter.is_ascii_alphanumeric() => {
            Some(u32::from(letter.to_ascii_lowercase()))
        }
        (Some('F'), Some(_)) if name.len() <= 3 => {
            let number: u32 = name[1..].parse().ok()?;
            (1..=24).contains(&number).then(|| F1 + number - 1)
        }
        _ => name
            .strip_prefix("Numpad")
            .and_then(|digit| digit.parse::<u32>().ok())
            .filter(|digit| *digit < 10)
            .map(|digit| PAD + digit)
            .or_else(|| {
                NAMED
                    .iter()
                    .find(|(known, _)| *known == name)
                    .map(|(_, symbol)| *symbol)
            }),
    }
}

#[cfg(test)]
mod tests {
    use super::Key;

    #[test]
    fn has_a_name() {
        assert_eq!(
            Key::spelled("PrintScreen"),
            Some(Key {
                held: 0,
                symbol: 0xff61
            })
        );
    }

    #[test]
    fn is_held_with_others() {
        assert_eq!(Key::spelled("Ctrl+Shift+F9").map(|key| key.held), Some(5));
    }

    #[test]
    fn is_held_with_the_key_of_the_system() {
        assert_eq!(Key::spelled("Alt+Cmd+S").map(|key| key.held), Some(72));
    }

    #[test]
    fn is_a_function_key() {
        assert_eq!(Key::spelled("F12").map(|key| key.symbol), Some(0xffc9));
    }

    #[test]
    fn is_a_letter() {
        assert_eq!(Key::spelled("Ctrl+F").map(|key| key.symbol), Some(0x66));
    }

    #[test]
    fn is_a_digit_of_the_pad() {
        assert_eq!(
            Key::spelled("Alt+Numpad7").map(|key| key.symbol),
            Some(0xffb7)
        );
    }

    #[test]
    fn is_not_there_without_a_name() {
        assert_eq!(Key::spelled("Ctrl+Shift"), None);
    }

    #[test]
    fn is_not_there_with_an_unknown_name() {
        assert_eq!(Key::spelled("Ctrl+F99"), None);
    }
}
