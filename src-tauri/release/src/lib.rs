//! What the latest release of the program is.

use std::error::Error;

mod latest;
mod version;

pub use latest::Latest;
pub use version::Version;

/// A failure to learn it. It travels between threads, hence `Send + Sync`.
pub type Failure = Box<dyn Error + Send + Sync>;
