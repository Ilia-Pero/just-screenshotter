//! What the program says to a person, in a dozen languages.
//!
//! Translations live in `locales`, one file per language; the editor uses the same files.

mod language;
mod phrase;

pub use language::{Language, LANGUAGES};
pub use phrase::Phrase;
