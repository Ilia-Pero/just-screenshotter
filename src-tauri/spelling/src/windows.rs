//! Spelling on Windows: the web view asks the spell checker of Windows.

#![allow(unsafe_code)]

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::thread;

use windows::core::{BOOL, HSTRING};
use windows::Win32::Globalization::{ISpellCheckerFactory, SpellCheckerFactory};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
};

use crate::profile::Profile;
use crate::Failure;

/// Languages as Windows calls them, and as the web view does.
const LANGUAGES: [(&str, &str); 2] = [("ru-RU", "ru"), ("en-US", "en-US")];

/// Spelling in windows of the program.
pub struct Spelling {
    folder: PathBuf,
}

impl Spelling {
    /// The folder is the one where the program keeps its local data.
    #[must_use]
    pub fn new(folder: PathBuf) -> Spelling {
        Spelling { folder }
    }

    /// Makes the web view check Russian and English besides the language of the system. Call it
    /// before the first window. Only languages Windows knows are added: for the others the web view
    /// would look for a dictionary on the internet.
    ///
    /// # Errors
    ///
    /// The settings of the web view are not read or not written.
    pub fn wake(&self) -> Result<(), Failure> {
        let path = self.folder.join("EBWebView/Default/Preferences");
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(failure) if failure.kind() == ErrorKind::NotFound => "{}".to_owned(),
            Err(failure) => return Err(failure.into()),
        };
        let profile = Profile::read(&text)?;
        if profile.checks(&LANGUAGES.map(|(_, code)| code)) {
            return Ok(());
        }
        let known = known()?;
        if known.is_empty() || profile.checks(&known) {
            return Ok(());
        }
        let draft = path.with_extension("new");
        if let Some(folder) = path.parent() {
            fs::create_dir_all(folder)?;
        }
        fs::write(&draft, profile.with(&known))?;
        Ok(fs::rename(draft, path)?)
    }
}

/// Languages that Windows can check. In a thread of its own, which takes its COM with it.
fn known() -> Result<Vec<&'static str>, Failure> {
    thread::spawn(|| {
        // SAFETY: the thread is new, and COM is let go after the last object is.
        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.ok()?;
        let asked = asked();
        // SAFETY: this pairs with the call above.
        unsafe { CoUninitialize() };
        asked
    })
    .join()
    .map_err(|_| "Asking Windows about languages has crashed")?
    .map_err(Failure::from)
}

fn asked() -> windows::core::Result<Vec<&'static str>> {
    // SAFETY: COM is ready in this thread, and the object is let go at the end.
    let factory: ISpellCheckerFactory =
        unsafe { CoCreateInstance(&SpellCheckerFactory, None, CLSCTX_INPROC_SERVER) }?;
    Ok(LANGUAGES
        .into_iter()
        .filter(|(tag, _)| {
            // SAFETY: the tag lives until the call returns.
            unsafe { factory.IsSupported(&HSTRING::from(*tag)) }.is_ok_and(BOOL::as_bool)
        })
        .map(|(_, code)| code)
        .collect())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{known, Spelling};

    #[test]
    fn begins_settings_of_web_view() {
        let folder = tempfile::tempdir().unwrap();
        Spelling::new(folder.path().to_owned()).wake().unwrap();
        assert_eq!(
            folder.path().join("EBWebView/Default/Preferences").exists(),
            !known().unwrap().is_empty()
        );
    }

    #[test]
    fn leaves_ready_settings_alone() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("EBWebView/Default/Preferences");
        let text = r#"{"spellcheck": {"forced_dictionaries": ["ru", "en-US"]}}"#;
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, text).unwrap();
        Spelling::new(folder.path().to_owned()).wake().unwrap();
        assert_eq!(fs::read_to_string(path).unwrap(), text);
    }

    #[test]
    fn does_not_touch_broken_settings() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("EBWebView/Default/Preferences");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "{").unwrap();
        assert!(Spelling::new(folder.path().to_owned()).wake().is_err());
    }
}
