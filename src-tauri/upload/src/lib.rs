//! Uploading a screenshot to a server of one's own.

use std::error::Error;

mod address;
mod file;
mod server;

pub use file::ServerFile;
pub use server::Server;

/// A failure to upload. It travels between threads, hence `Send + Sync`.
pub type Failure = Box<dyn Error + Send + Sync>;
