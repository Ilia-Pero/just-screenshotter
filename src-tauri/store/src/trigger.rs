//! What starts actions of the program.

use serde::{Deserialize, Serialize};

/// A key, as Tauri spells it, and buttons of the mouse. Empty means nothing.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Trigger {
    pub key: String,
    pub mouse: String,
}

impl Trigger {
    /// The trigger with a key and without the mouse.
    #[must_use]
    pub fn key(key: &str) -> Self {
        Self {
            key: key.to_owned(),
            ..Self::default()
        }
    }
}

/// What starts each action.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Triggers {
    /// A screenshot of an area.
    pub area: Trigger,
    /// A screenshot of the whole screen.
    pub screen: Trigger,
    /// A screenshot of the active window.
    pub window: Trigger,
    /// A film of an area.
    pub film: Trigger,
}

impl Triggers {
    /// All the triggers, in the order of actions.
    #[must_use]
    pub fn all(&self) -> [&Trigger; 4] {
        [&self.area, &self.screen, &self.window, &self.film]
    }

    /// Whether some key or some buttons of the mouse start two actions at once.
    #[must_use]
    pub fn clash(&self) -> bool {
        let twice = |words: Vec<&str>| {
            words
                .iter()
                .enumerate()
                .any(|(index, word)| !word.is_empty() && words[..index].contains(word))
        };
        twice(self.all().map(|trigger| trigger.key.as_str()).to_vec())
            || twice(self.all().map(|trigger| trigger.mouse.as_str()).to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::{Trigger, Triggers};

    fn mouse(buttons: &str) -> Trigger {
        Trigger {
            mouse: buttons.to_owned(),
            ..Trigger::default()
        }
    }

    #[test]
    fn may_start_nothing_at_all() {
        assert!(!Triggers::default().clash());
    }

    #[test]
    fn may_have_different_keys() {
        assert!(!Triggers {
            area: Trigger::key("F8"),
            film: Trigger::key("F9"),
            ..Triggers::default()
        }
        .clash());
    }

    #[test]
    fn clash_when_one_key_starts_two_actions() {
        assert!(Triggers {
            area: Trigger::key("F9"),
            film: Trigger::key("F9"),
            ..Triggers::default()
        }
        .clash());
    }

    #[test]
    fn clash_when_the_same_buttons_start_two_actions() {
        assert!(Triggers {
            screen: mouse("both"),
            window: mouse("both"),
            ..Triggers::default()
        }
        .clash());
    }
}
