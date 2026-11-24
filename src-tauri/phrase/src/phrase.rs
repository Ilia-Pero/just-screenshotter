//! A phrase for a person.

use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::Language;

/// A phrase: the key of its text and values for the holes in it, such as `{reason}`.
/// Shown to a person, it speaks the current language.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Phrase {
    key: &'static str,
    values: Vec<(&'static str, String)>,
}

impl Phrase {
    #[must_use]
    pub fn new(key: &'static str) -> Self {
        Self {
            key,
            values: Vec::new(),
        }
    }

    /// The same phrase with the hole filled.
    #[must_use]
    pub fn with(&self, hole: &'static str, value: impl Display) -> Self {
        let mut values = self.values.clone();
        values.push((hole, value.to_string()));
        Self {
            key: self.key,
            values,
        }
    }

    /// The phrase in the language.
    #[must_use]
    pub fn spoken(&self, language: &Language) -> String {
        self.values
            .iter()
            .fold(language.text(self.key), |text, (hole, value)| {
                text.replace(&format!("{{{hole}}}"), value)
            })
    }
}

impl Display for Phrase {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        out.write_str(&self.spoken(&Language::current()))
    }
}

impl Error for Phrase {}

#[cfg(test)]
mod tests {
    use super::Phrase;
    use crate::Language;

    #[test]
    fn speaks_given_language() {
        assert_eq!(
            Phrase::new("pencil").spoken(&Language::tagged("fr")),
            "Crayon"
        );
    }

    #[test]
    fn fills_the_hole() {
        assert_eq!(
            Phrase::new("error.code")
                .with("code", 500)
                .spoken(&Language::tagged("en")),
            "The server answered with error 500."
        );
    }

    #[test]
    fn fills_several_holes() {
        assert_eq!(
            Phrase::new("error.kept")
                .with("key", "F9")
                .with("kept", "F10")
                .spoken(&Language::tagged("en")),
            "Another program uses F9. The shortcut is still F10."
        );
    }
}
