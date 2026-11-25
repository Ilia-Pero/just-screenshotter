//! What the program keeps on the disk: screenshots and settings.

use std::error::Error;

mod folder;
mod settings;

pub use folder::Folder;
pub use settings::{Listened, Settings, SettingsFile};

/// A failure of the disk. It travels between threads, hence `Send + Sync`.
pub type Failure = Box<dyn Error + Send + Sync>;
