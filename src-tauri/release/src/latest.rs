//! The page of the latest release.

use std::time::Duration;

use ureq::http::StatusCode;
use ureq::Agent;

use crate::version::Version;
use crate::Failure;

/// The latest release of the program on GitHub.
pub struct Latest {
    home: String,
}

impl Latest {
    /// The home is the page of the repository.
    #[must_use]
    pub fn new(home: &str) -> Latest {
        Latest {
            home: home.to_owned(),
        }
    }

    /// The page that tells what is new.
    #[must_use]
    pub fn page(&self) -> String {
        format!("{}/releases/latest", self.home)
    }

    /// The address of a file of the release.
    #[must_use]
    pub fn file(&self, name: &str) -> String {
        format!("{}/releases/latest/download/{name}", self.home)
    }

    /// The version of the release. The page leads to the one of the release itself, and its address
    /// ends with the version, so nothing has to be downloaded.
    ///
    /// # Errors
    ///
    /// GitHub does not answer, or there is no release.
    pub fn version(&self) -> Result<Version, Failure> {
        let agent: Agent = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(15)))
            .max_redirects(0)
            .max_redirects_will_error(false)
            .build()
            .into();
        let answer = agent.head(self.page()).call()?;
        if answer.status() != StatusCode::FOUND {
            return Err(format!("GitHub has answered {}", answer.status()).into());
        }
        answer
            .headers()
            .get("location")
            .and_then(|place| place.to_str().ok())
            .and_then(|place| place.rsplit('/').next())
            .ok_or("GitHub has not named the release")?
            .parse()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    use super::Latest;

    /// GitHub on this computer that gives the answer once.
    fn served(answer: &'static str) -> Latest {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let home = format!("http://{}/a/b", listener.local_addr().unwrap());
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let _ = stream.read(&mut [0; 2048]).unwrap();
            stream.write_all(answer.as_bytes()).unwrap();
        });
        Latest::new(&home)
    }

    #[test]
    fn learns_version_from_address() {
        assert_eq!(
            served(
                "HTTP/1.1 302 Found\r\nLocation: https://github.com/a/b/releases/tag/0.4.0\r\n\
                 Content-Length: 0\r\nConnection: close\r\n\r\n"
            )
            .version()
            .unwrap()
            .to_string(),
            "0.4.0"
        );
    }

    #[test]
    fn does_not_take_a_list_for_a_release() {
        assert!(served(
            "HTTP/1.1 302 Found\r\nLocation: https://github.com/a/b/releases\r\n\
             Content-Length: 0\r\nConnection: close\r\n\r\n"
        )
        .version()
        .is_err());
    }

    #[test]
    fn does_not_take_a_page_for_a_release() {
        assert!(
            served("HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                .version()
                .is_err()
        );
    }

    #[test]
    fn reports_a_missing_page() {
        assert!(
            served("HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                .version()
                .is_err()
        );
    }

    #[test]
    fn names_files_without_version() {
        assert_eq!(
            Latest::new("https://github.com/a/b").file("c.exe"),
            "https://github.com/a/b/releases/latest/download/c.exe"
        );
    }
}
