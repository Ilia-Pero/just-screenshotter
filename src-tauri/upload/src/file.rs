//! The file with the server address and the key.

use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};

use screenshotter_phrase::Phrase;

use crate::address::Address;
use crate::{Failure, Server};

/// The file with the server address and the key. There is no server while there is no file.
pub struct ServerFile {
    file: PathBuf,
}

impl ServerFile {
    #[must_use]
    pub fn new(file: PathBuf) -> Self {
        Self { file }
    }

    /// The server from the file.
    ///
    /// # Errors
    ///
    /// There is a file, but it cannot be read or understood.
    pub fn read(&self) -> Result<Server, Failure> {
        match fs::read(&self.file) {
            Ok(json) => Ok(serde_json::from_slice(&json)?),
            Err(failure) if failure.kind() == ErrorKind::NotFound => Ok(Server::default()),
            Err(failure) => Err(failure.into()),
        }
    }

    /// Writes the address and the key. On macOS and Linux nobody but the owner can read the file;
    /// on Windows the folder of the program is closed to other people anyway.
    ///
    /// # Errors
    ///
    /// The address is not safe, or the file cannot be written.
    pub fn write(&self, server: &Server) -> Result<(), Failure> {
        if !server.address.is_empty() && !Address::new(&server.address).safe() {
            return Err(Phrase::new("error.address").into());
        }
        if let Some(folder) = self.file.parent() {
            fs::create_dir_all(folder)?;
        }
        let draft = self.file.with_extension("new");
        let mut secret = closed(&draft)?;
        secret.write_all(&serde_json::to_vec_pretty(server)?)?;
        secret.sync_all()?;
        drop(secret);
        fs::rename(&draft, &self.file)?;
        Ok(())
    }
}

/// An empty file that nobody but the owner can read, even if it was there and could be read.
#[cfg(unix)]
fn closed(file: &Path) -> io::Result<File> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    let opened = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(file)?;
    opened.set_permissions(fs::Permissions::from_mode(0o600))?;
    Ok(opened)
}

/// An empty file.
#[cfg(not(unix))]
fn closed(file: &Path) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::ServerFile;
    use crate::Server;

    fn server(address: &str) -> Server {
        Server {
            address: address.to_owned(),
            token: "key".to_owned(),
        }
    }

    #[test]
    fn knows_no_server_at_first() {
        let root = TempDir::new().unwrap();
        assert_eq!(
            ServerFile::new(root.path().join("server.json"))
                .read()
                .unwrap(),
            Server::default()
        );
    }

    #[test]
    fn keeps_address_and_key() {
        let root = TempDir::new().unwrap();
        let file = ServerFile::new(root.path().join("server.json"));
        file.write(&server("https://shots.example/")).unwrap();
        assert_eq!(file.read().unwrap(), server("https://shots.example/"));
    }

    #[cfg(unix)]
    #[test]
    fn closes_the_key_to_other_people() {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let root = TempDir::new().unwrap();
        let path = root.path().join("server.json");
        fs::write(path.with_extension("new"), "an old draft").unwrap();
        ServerFile::new(path.clone())
            .write(&server("https://shots.example/"))
            .unwrap();
        assert_eq!(
            fs::metadata(path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }

    #[test]
    fn refuses_to_keep_open_address() {
        let root = TempDir::new().unwrap();
        assert!(ServerFile::new(root.path().join("server.json"))
            .write(&server("http://shots.example/"))
            .is_err());
    }
}
