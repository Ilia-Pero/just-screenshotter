//! Settings of the program.

use std::fs::{self, File};
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{Failure, Trigger, Triggers};

/// What starts actions, the folder for screenshots, the code of the language, which is empty for
/// the language of the system, what to record with video, and whether the program has settled in
/// the system: it does so once, and then a person decides.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Settings {
    pub triggers: Triggers,
    pub folder: PathBuf,
    pub language: String,
    pub ears: Listened,
    pub settled: bool,
}

/// Settings as they lie in the file. Versions before 0.2 wrote only the key.
#[derive(Deserialize)]
struct Stored {
    #[serde(default)]
    hotkey: String,
    triggers: Option<Triggers>,
    folder: PathBuf,
    #[serde(default)]
    language: String,
    #[serde(default)]
    ears: Listened,
    #[serde(default)]
    settled: bool,
}

impl Stored {
    /// The settings. What the file does not have comes from the initial ones.
    fn settings(self, initial: &Settings) -> Settings {
        let before = Triggers {
            area: if self.hotkey.is_empty() {
                initial.triggers.area.clone()
            } else {
                Trigger::key(&self.hotkey)
            },
            ..initial.triggers.clone()
        };
        Settings {
            triggers: self.triggers.unwrap_or(before),
            folder: self.folder,
            language: self.language,
            ears: self.ears,
            settled: self.settled,
        }
    }
}

/// What is recorded together with video: the sound of the computer, and the microphone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Listened {
    pub sound: bool,
    pub voice: bool,
}

/// The microphone is off until a person turns it on.
impl Default for Listened {
    fn default() -> Self {
        Self {
            sound: true,
            voice: false,
        }
    }
}

impl Settings {
    /// The same settings with other triggers.
    #[must_use]
    pub fn with_triggers(&self, triggers: Triggers) -> Settings {
        Settings {
            triggers,
            ..self.clone()
        }
    }

    /// The same settings of a program that has settled in the system.
    #[must_use]
    pub fn settled(&self) -> Settings {
        Settings {
            settled: true,
            ..self.clone()
        }
    }

    /// The same settings with another folder.
    #[must_use]
    pub fn with_folder(&self, folder: PathBuf) -> Settings {
        Settings {
            folder,
            ..self.clone()
        }
    }

    /// The same settings with something else recorded together with video.
    #[must_use]
    pub fn with_ears(&self, ears: Listened) -> Settings {
        Settings {
            ears,
            ..self.clone()
        }
    }

    /// The same settings with another language.
    #[must_use]
    pub fn with_language(&self, language: &str) -> Settings {
        Settings {
            language: language.to_owned(),
            ..self.clone()
        }
    }
}

/// The file with the settings. Initial settings are in force while there is no file.
pub struct SettingsFile {
    file: PathBuf,
    initial: Settings,
}

impl SettingsFile {
    #[must_use]
    pub fn new(file: PathBuf, initial: Settings) -> Self {
        Self { file, initial }
    }

    /// Settings from the file.
    ///
    /// # Errors
    ///
    /// There is a file, but it cannot be read or understood.
    pub fn read(&self) -> Result<Settings, Failure> {
        match fs::read(&self.file) {
            Ok(json) => {
                let stored: Stored = serde_json::from_slice(&json)?;
                Ok(stored.settings(&self.initial))
            }
            Err(failure) if failure.kind() == ErrorKind::NotFound => Ok(self.initial.clone()),
            Err(failure) => Err(failure.into()),
        }
    }

    /// Writes the settings. The old file is replaced as a whole, so a crash in the middle
    /// does not spoil it.
    ///
    /// # Errors
    ///
    /// The file cannot be written.
    pub fn write(&self, settings: &Settings) -> Result<(), Failure> {
        if let Some(folder) = self.file.parent() {
            fs::create_dir_all(folder)?;
        }
        let draft = self.file.with_extension("new");
        let mut fresh = File::create(&draft)?;
        fresh.write_all(&serde_json::to_vec_pretty(settings)?)?;
        fresh.sync_all()?;
        drop(fresh);
        fs::rename(&draft, &self.file)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use tempfile::TempDir;

    use super::{Listened, Settings, SettingsFile};
    use crate::{Trigger, Triggers};

    fn initial() -> Settings {
        Settings {
            triggers: Triggers {
                area: Trigger::key("PrintScreen"),
                film: Trigger::key("F8"),
                ..Triggers::default()
            },
            folder: PathBuf::from("shots"),
            language: String::new(),
            ears: Listened::default(),
            settled: false,
        }
    }

    /// The initial settings with another key for the area.
    fn keyed(key: &str) -> Settings {
        initial().with_triggers(Triggers {
            area: Trigger::key(key),
            ..initial().triggers
        })
    }

    fn file(root: &TempDir) -> SettingsFile {
        SettingsFile::new(root.path().join("deep").join("settings.json"), initial())
    }

    #[test]
    fn starts_with_initial_settings() {
        let root = TempDir::new().unwrap();
        assert_eq!(file(&root).read().unwrap(), initial());
    }

    #[test]
    fn returns_what_was_written() {
        let root = TempDir::new().unwrap();
        file(&root).write(&keyed("F9")).unwrap();
        assert_eq!(file(&root).read().unwrap(), keyed("F9"));
    }

    #[test]
    fn remembers_that_it_has_settled() {
        let root = TempDir::new().unwrap();
        file(&root).write(&initial().settled()).unwrap();
        assert!(file(&root).read().unwrap().settled);
    }

    #[test]
    fn keeps_an_action_without_a_key() {
        let root = TempDir::new().unwrap();
        let bare = initial().with_triggers(Triggers::default());
        file(&root).write(&bare).unwrap();
        assert_eq!(file(&root).read().unwrap(), bare);
    }

    #[test]
    fn does_not_record_the_microphone_at_first() {
        assert!(!initial().ears.voice);
    }

    #[test]
    fn remembers_what_to_record() {
        let root = TempDir::new().unwrap();
        let ears = Listened {
            sound: false,
            voice: true,
        };
        file(&root).write(&initial().with_ears(ears)).unwrap();
        assert_eq!(file(&root).read().unwrap().ears, ears);
    }

    #[test]
    fn replaces_earlier_settings() {
        let root = TempDir::new().unwrap();
        file(&root).write(&keyed("F9")).unwrap();
        file(&root)
            .write(&initial().with_folder(PathBuf::from("other")))
            .unwrap();
        assert_eq!(
            file(&root).read().unwrap(),
            initial().with_folder(PathBuf::from("other"))
        );
    }

    #[test]
    fn reads_file_written_before_languages() {
        let root = TempDir::new().unwrap();
        fs::write(
            root.path().join("settings.json"),
            r#"{"hotkey":"F9","folder":"shots"}"#,
        )
        .unwrap();
        assert_eq!(
            SettingsFile::new(root.path().join("settings.json"), initial())
                .read()
                .unwrap(),
            keyed("F9")
        );
    }

    #[test]
    fn reports_broken_file() {
        let root = TempDir::new().unwrap();
        fs::write(root.path().join("settings.json"), "{").unwrap();
        assert!(
            SettingsFile::new(root.path().join("settings.json"), initial())
                .read()
                .is_err()
        );
    }
}
