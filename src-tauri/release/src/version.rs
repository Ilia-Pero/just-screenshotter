//! A version of the program.

use std::fmt::{Display, Formatter, Result as Written};
use std::str::FromStr;

use crate::Failure;

/// A version of the program: three numbers, such as 0.4.0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    parts: [u32; 3],
}

impl FromStr for Version {
    type Err = Failure;

    fn from_str(text: &str) -> Result<Version, Failure> {
        let parts = text
            .split('.')
            .map(str::parse)
            .collect::<Result<Vec<u32>, _>>()
            .ok()
            .and_then(|parts| parts.try_into().ok());
        match parts {
            Some(parts) => Ok(Version { parts }),
            None => Err(format!("“{text}” is not a version").into()),
        }
    }
}

impl Display for Version {
    fn fmt(&self, form: &mut Formatter<'_>) -> Written {
        let [major, minor, patch] = self.parts;
        write!(form, "{major}.{minor}.{patch}")
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    fn version(text: &str) -> Version {
        text.parse().unwrap()
    }

    #[test]
    fn compares_numbers_and_not_letters() {
        assert!(version("0.10.0") > version("0.9.9"));
    }

    #[test]
    fn looks_at_the_first_number_first() {
        assert!(version("1.0.0") > version("0.9.9"));
    }

    #[test]
    fn is_not_newer_than_itself() {
        assert!(version("0.4.0") <= version("0.4.0"));
    }

    #[test]
    fn is_written_as_it_was_read() {
        assert_eq!(version("0.4.12").to_string(), "0.4.12");
    }

    #[test]
    fn is_three_numbers() {
        assert!("0.4".parse::<Version>().is_err());
    }

    #[test]
    fn is_not_words() {
        assert!("latest".parse::<Version>().is_err());
    }
}
