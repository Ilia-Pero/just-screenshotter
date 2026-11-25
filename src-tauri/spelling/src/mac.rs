//! Spelling on macOS: the web view asks the system if the program wants it to.

#![allow(unsafe_code)]

use std::path::PathBuf;

use objc2::runtime::AnyObject;
use objc2_foundation::{NSDictionary, NSNumber, NSString, NSUserDefaults};

use crate::Failure;

/// The setting the web view looks at.
const KEY: &str = "WebContinuousSpellCheckingEnabled";

/// Spelling in windows of the program.
pub struct Spelling {
    _folder: PathBuf,
}

impl Spelling {
    /// The folder is the one where the program keeps its local data.
    #[must_use]
    pub fn new(folder: PathBuf) -> Spelling {
        Spelling { _folder: folder }
    }

    /// Makes the web view check spelling. Call it before the first window. macOS finds the language
    /// by itself. This is a wish for a start: a person can turn it off in the menu of the field, and
    /// that is remembered.
    ///
    /// # Errors
    ///
    /// There are none.
    #[allow(clippy::unnecessary_wraps)]
    pub fn wake(&self) -> Result<(), Failure> {
        let key = NSString::from_str(KEY);
        let on = NSNumber::new_bool(true);
        let value: &AnyObject = &on;
        let wishes = NSDictionary::from_slices(&[&*key], &[value]);
        // SAFETY: the dictionary holds what settings may hold: a number under a string.
        unsafe { NSUserDefaults::standardUserDefaults().registerDefaults(&wishes) };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use objc2_foundation::{NSString, NSUserDefaults};

    use super::{Spelling, KEY};

    #[test]
    fn wakes() {
        Spelling::new(PathBuf::new()).wake().unwrap();
        assert!(NSUserDefaults::standardUserDefaults().boolForKey(&NSString::from_str(KEY)));
    }
}
