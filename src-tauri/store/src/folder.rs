//! The folder where screenshots are saved.

use std::fs::{self, File};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use screenshotter_phrase::Phrase;

use crate::Failure;

/// How every PNG file begins.
const SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";

/// A folder with screenshots on the disk.
pub struct Folder {
    root: PathBuf,
    word: String,
}

impl Folder {
    /// `word` is the word every file name starts with.
    #[must_use]
    pub fn new(root: PathBuf, word: String) -> Self {
        Self { root, word }
    }

    /// The path to the folder. Creates the folder if there is none.
    ///
    /// # Errors
    ///
    /// The folder cannot be created.
    pub fn path(&self) -> Result<&Path, Failure> {
        fs::create_dir_all(&self.root)?;
        Ok(&self.root)
    }

    /// Writes the screenshot taken at the moment to a new file and returns the path to it.
    /// Never overwrites existing files.
    ///
    /// # Errors
    ///
    /// This is not PNG, or the file cannot be written.
    pub fn store(&self, png: &[u8], moment: DateTime<Local>) -> Result<PathBuf, Failure> {
        if !png.starts_with(SIGNATURE) {
            return Err(Phrase::new("error.picture").into());
        }
        let stem = format!("{} {}", self.word, moment.format("%Y-%m-%d %H-%M-%S"));
        let root = self.path()?;
        let mut name = format!("{stem}.png");
        for copy in 2.. {
            let file = root.join(&name);
            match File::create_new(&file) {
                Ok(mut fresh) => {
                    fresh.write_all(png)?;
                    fresh.sync_all()?;
                    return Ok(file);
                }
                Err(failure) if failure.kind() == ErrorKind::AlreadyExists => {
                    name = format!("{stem} ({copy}).png");
                }
                Err(failure) => return Err(failure.into()),
            }
        }
        Err("There is no free name for the file".into())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use chrono::{DateTime, Local, TimeZone};
    use tempfile::TempDir;

    use super::Folder;

    const PNG: &[u8] = b"\x89PNG\r\n\x1a\nfirst";

    fn noon() -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 11, 23, 12, 0, 0).unwrap()
    }

    fn folder(root: PathBuf) -> Folder {
        Folder::new(root, "Screenshot".to_owned())
    }

    #[test]
    fn names_file_after_the_moment() {
        let root = TempDir::new().unwrap();
        assert!(folder(root.path().to_owned())
            .store(PNG, noon())
            .unwrap()
            .ends_with("Screenshot 2026-11-23 12-00-00.png"));
    }

    #[test]
    fn gives_second_shot_of_the_same_second_another_name() {
        let root = TempDir::new().unwrap();
        let folder = folder(root.path().to_owned());
        folder.store(PNG, noon()).unwrap();
        assert!(folder
            .store(PNG, noon())
            .unwrap()
            .ends_with("Screenshot 2026-11-23 12-00-00 (2).png"));
    }

    #[test]
    fn keeps_first_shot_when_second_comes_in_the_same_second() {
        let root = TempDir::new().unwrap();
        let folder = folder(root.path().to_owned());
        let first = folder.store(PNG, noon()).unwrap();
        folder.store(b"\x89PNG\r\n\x1a\nsecond", noon()).unwrap();
        assert_eq!(fs::read(first).unwrap(), PNG);
    }

    #[test]
    fn refuses_bytes_that_are_not_png() {
        let root = TempDir::new().unwrap();
        assert!(folder(root.path().to_owned()).store(b"", noon()).is_err());
    }

    #[test]
    fn creates_missing_folder() {
        let root = TempDir::new().unwrap();
        assert!(folder(root.path().join("new")).path().unwrap().is_dir());
    }
}
