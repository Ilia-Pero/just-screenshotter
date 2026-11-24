//! Settings of the program.

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::Failure;

/// The key that takes a screenshot, as Tauri spells it, the folder for screenshots, the code of
/// the language, which is empty for the language of the system, and what to record with video.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub hotkey: String,
    pub folder: PathBuf,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub ears: Listened,
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
    /// The same settings with another key.
    #[must_use]
    pub fn with_hotkey(&self, hotkey: &str) -> Settings {
        Settings {
            hotkey: hotkey.to_owned(),
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
            Ok(json) => Ok(serde_json::from_slice(&json)?),
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
        fs::write(&draft, serde_json::to_vec_pretty(settings)?)?;
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

    fn initial() -> Settings {
        Settings {
            hotkey: "PrintScreen".to_owned(),
            folder: PathBuf::from("shots"),
            language: String::new(),
            ears: Listened::default(),
        }
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
        file(&root).write(&initial().with_hotkey("F9")).unwrap();
        assert_eq!(file(&root).read().unwrap().hotkey, "F9");
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
        file(&root).write(&initial().with_hotkey("F9")).unwrap();
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
            initial().with_hotkey("F9")
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
