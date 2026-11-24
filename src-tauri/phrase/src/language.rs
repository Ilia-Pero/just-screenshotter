//! A language the program speaks.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{PoisonError, RwLock};

/// A language: its code and the text of its file with translations.
#[derive(Clone, Copy, Debug)]
pub struct Language {
    pub code: &'static str,
    text: &'static str,
}

/// All languages. The first one is used when there is no better choice.
pub const LANGUAGES: &[Language] = &[
    Language {
        code: "en",
        text: include_str!("../../../locales/en.json"),
    },
    Language {
        code: "de",
        text: include_str!("../../../locales/de.json"),
    },
    Language {
        code: "el",
        text: include_str!("../../../locales/el.json"),
    },
    Language {
        code: "es",
        text: include_str!("../../../locales/es.json"),
    },
    Language {
        code: "fr",
        text: include_str!("../../../locales/fr.json"),
    },
    Language {
        code: "he",
        text: include_str!("../../../locales/he.json"),
    },
    Language {
        code: "it",
        text: include_str!("../../../locales/it.json"),
    },
    Language {
        code: "ja",
        text: include_str!("../../../locales/ja.json"),
    },
    Language {
        code: "ko",
        text: include_str!("../../../locales/ko.json"),
    },
    Language {
        code: "ru",
        text: include_str!("../../../locales/ru.json"),
    },
    Language {
        code: "sr",
        text: include_str!("../../../locales/sr.json"),
    },
    Language {
        code: "zh",
        text: include_str!("../../../locales/zh.json"),
    },
];

/// The code of the language chosen in the menu. Empty while the system decides.
static CHOSEN: RwLock<String> = RwLock::new(String::new());

impl Language {
    /// The language with the tag, such as `ru-RU` or `zh_CN.UTF-8`.
    #[must_use]
    pub fn tagged(tag: &str) -> Language {
        let code = tag
            .split(['-', '_', '.'])
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        // Old systems call Hebrew by its former code.
        let code = if code == "iw" { "he" } else { code.as_str() };
        LANGUAGES
            .iter()
            .copied()
            .find(|language| language.code == code)
            .unwrap_or(LANGUAGES[0])
    }

    /// The language the program speaks now: the chosen one, or the language of the system.
    #[must_use]
    pub fn current() -> Language {
        let chosen = CHOSEN
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        if chosen.is_empty() {
            Language::tagged(&sys_locale::get_locale().unwrap_or_default())
        } else {
            Language::tagged(&chosen)
        }
    }

    /// The code of the language chosen in the installer, which leaves it in a file next to
    /// the program. Empty if there was no choice: only the installer for Windows asks.
    #[must_use]
    pub fn installed(folder: &Path) -> String {
        let code = fs::read_to_string(folder.join("language")).unwrap_or_default();
        LANGUAGES
            .iter()
            .map(|language| language.code)
            .find(|known| *known == code.trim())
            .unwrap_or_default()
            .to_owned()
    }

    /// Makes the whole program speak the language with the code. An empty code returns
    /// the choice to the system.
    pub fn choose(code: &str) {
        code.clone_into(&mut CHOSEN.write().unwrap_or_else(PoisonError::into_inner));
    }

    /// The text under the key. Empty if the language has no such text.
    #[must_use]
    pub fn text(&self, key: &str) -> String {
        self.texts().remove(key).unwrap_or_default()
    }

    /// All texts of the language by their keys.
    #[must_use]
    pub fn texts(&self) -> HashMap<String, String> {
        serde_json::from_str(self.text).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;

    use tempfile::TempDir;

    use super::{Language, LANGUAGES};

    /// The language of a program next to which the installer has left the text.
    fn installed(text: &str) -> String {
        let folder = TempDir::new().unwrap();
        fs::write(folder.path().join("language"), text).unwrap();
        Language::installed(folder.path())
    }

    fn holes(text: &str) -> BTreeSet<&str> {
        text.split('{')
            .skip(1)
            .filter_map(|part| part.split('}').next())
            .collect()
    }

    /// Languages for which the statement about English texts is wrong.
    fn failing(agrees: impl Fn(&Language) -> bool) -> Vec<&'static str> {
        LANGUAGES
            .iter()
            .filter(|language| !agrees(language))
            .map(|language| language.code)
            .collect()
    }

    #[test]
    fn has_the_same_keys_in_every_language() {
        let english: BTreeSet<String> = LANGUAGES[0].texts().into_keys().collect();
        assert_eq!(
            failing(|language| language.texts().into_keys().collect::<BTreeSet<_>>() == english),
            Vec::<&str>::new()
        );
    }

    #[test]
    fn has_the_same_holes_in_every_language() {
        let english = LANGUAGES[0].texts();
        assert_eq!(
            failing(|language| language
                .texts()
                .iter()
                .all(|(key, text)| holes(text) == holes(&english[key]))),
            Vec::<&str>::new()
        );
    }

    #[test]
    fn has_no_empty_text_in_any_language() {
        assert_eq!(
            failing(|language| language
                .texts()
                .values()
                .all(|text| !text.trim().is_empty())),
            Vec::<&str>::new()
        );
    }

    #[test]
    fn understands_tag_with_region() {
        assert_eq!(Language::tagged("ru-RU").code, "ru");
    }

    #[test]
    fn understands_tag_with_underscore_and_encoding() {
        assert_eq!(Language::tagged("zh_CN.UTF-8").code, "zh");
    }

    #[test]
    fn understands_tag_with_script() {
        assert_eq!(Language::tagged("sr-Latn-RS").code, "sr");
    }

    #[test]
    fn understands_former_code_of_hebrew() {
        assert_eq!(Language::tagged("iw-IL").code, "he");
    }

    #[test]
    fn remembers_what_was_chosen_in_the_installer() {
        assert_eq!(installed("el\r\n"), "el");
    }

    #[test]
    fn does_not_take_an_unknown_language_from_the_installer() {
        assert_eq!(installed("xx"), "");
    }

    #[test]
    fn leaves_the_choice_to_the_system_without_an_installer() {
        assert_eq!(Language::installed(TempDir::new().unwrap().path()), "");
    }

    #[test]
    fn falls_back_to_english() {
        assert_eq!(Language::tagged("fi-FI").code, "en");
    }
}
