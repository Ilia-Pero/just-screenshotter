//! Settings of the web view.

use serde_json::{json, Map, Value};

use crate::Failure;

/// The part of the settings about spelling.
const PART: &str = "spellcheck";

/// The languages that are checked besides the one of the system. The web view sets the usual list
/// anew at every start, and this one it leaves alone.
const EXTRA: &str = "forced_dictionaries";

/// The settings of the web view: an object of JSON.
pub struct Profile {
    all: Map<String, Value>,
}

impl Profile {
    /// Reads the text of the settings.
    ///
    /// # Errors
    ///
    /// This is not an object of JSON.
    pub fn read(text: &str) -> Result<Profile, Failure> {
        Ok(Profile {
            all: serde_json::from_str(text)?,
        })
    }

    /// Whether exactly these languages are checked besides the one of the system.
    #[must_use]
    pub fn checks(&self, languages: &[&str]) -> bool {
        self.all
            .get(PART)
            .and_then(|part| part.get(EXTRA))
            .is_some_and(|list| *list == json!(languages))
    }

    /// The text of the settings where these languages are checked.
    #[must_use]
    pub fn with(mut self, languages: &[&str]) -> String {
        let part = self.all.entry(PART).or_insert_with(|| json!({}));
        if !part.is_object() {
            *part = json!({});
        }
        part[EXTRA] = json!(languages);
        Value::Object(self.all).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Profile;

    const WHOLE: &str = r#"{"intl":{"a":1},"spellcheck":{"dictionaries":["ru"]}}"#;

    fn taught(text: &str) -> Profile {
        Profile::read(&Profile::read(text).unwrap().with(&["ru", "en-US"])).unwrap()
    }

    #[test]
    fn checks_nothing_at_first() {
        assert!(!Profile::read(WHOLE).unwrap().checks(&["ru", "en-US"]));
    }

    #[test]
    fn checks_languages_it_is_given() {
        assert!(taught(WHOLE).checks(&["ru", "en-US"]));
    }

    #[test]
    fn does_not_check_other_languages() {
        assert!(!taught(WHOLE).checks(&["ru"]));
    }

    #[test]
    fn begins_with_nothing() {
        assert!(taught("{}").checks(&["ru", "en-US"]));
    }

    #[test]
    fn keeps_other_settings() {
        assert!(Profile::read(WHOLE)
            .unwrap()
            .with(&["ru"])
            .contains(r#""intl":{"a":1}"#));
    }

    #[test]
    fn keeps_language_of_system() {
        assert!(Profile::read(WHOLE)
            .unwrap()
            .with(&["en-US"])
            .contains(r#""dictionaries":["ru"]"#));
    }

    #[test]
    fn repairs_a_part_of_another_kind() {
        assert!(taught(r#"{"spellcheck":7}"#).checks(&["ru", "en-US"]));
    }

    #[test]
    fn does_not_read_a_broken_text() {
        assert!(Profile::read(r#"{"spellcheck":"#).is_err());
    }
}
