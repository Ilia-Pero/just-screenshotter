//! The server that accepts screenshots.

use std::time::Duration;

use screenshotter_phrase::Phrase;
use serde::{Deserialize, Serialize};

use crate::address::Address;
use crate::Failure;

/// An answer of the server is never longer.
const LIMIT: u64 = 16 * 1024;

/// A server that accepts screenshots: its address and the key it accepts them with.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Server {
    pub address: String,
    pub token: String,
}

/// An answer of the server.
#[derive(Deserialize)]
struct Receipt {
    url: String,
}

impl Server {
    /// True if there is where to upload.
    #[must_use]
    pub fn ready(&self) -> bool {
        Address::new(&self.address).safe() && !self.token.is_empty()
    }

    /// Uploads the screenshot and returns the link to it.
    ///
    /// # Errors
    ///
    /// There is no address, or the server does not answer or does not accept the screenshot.
    pub fn link(&self, png: &[u8]) -> Result<String, Failure> {
        if !self.ready() {
            return Err(Phrase::new("error.noserver").into());
        }
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(60)))
            // The key must not follow the server to another address.
            .max_redirects(0)
            .build()
            .into();
        let text = agent
            .post(&self.address)
            .header("Authorization", format!("Bearer {}", self.token))
            .content_type("image/png")
            .send(png)
            .and_then(|mut answer| {
                answer
                    .body_mut()
                    .with_config()
                    .limit(LIMIT)
                    .read_to_string()
            })
            .map_err(refusal)?;
        let url = serde_json::from_str::<Receipt>(&text)
            .map_err(|_| strange())?
            .url;
        if Address::new(&url).safe() {
            Ok(url)
        } else {
            Err(strange().into())
        }
    }
}

fn strange() -> Phrase {
    Phrase::new("error.link")
}

/// The refusal in words a person understands.
fn refusal(failure: ureq::Error) -> Failure {
    match failure {
        ureq::Error::StatusCode(401 | 403) => Phrase::new("error.refused"),
        ureq::Error::StatusCode(404 | 405) => Phrase::new("error.nowhere"),
        ureq::Error::StatusCode(413) => Phrase::new("error.large"),
        ureq::Error::StatusCode(code) => Phrase::new("error.code").with("code", code),
        other => Phrase::new("error.silent").with("reason", other),
    }
    .into()
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::thread;

    use screenshotter_phrase::Phrase;

    use super::Server;

    const CREATED: &str = "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\
        Content-Length: 35\r\nConnection: close\r\n\r\n{\"url\":\"https://shots.example/a.p\"}";

    /// A server on this computer that gives the answer once, and what it has heard.
    fn served(answer: &'static str) -> (Server, mpsc::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let server = Server {
            address: format!("http://{}/", listener.local_addr().unwrap()),
            token: "secret-key".to_owned(),
        };
        let (say, hear) = mpsc::channel();
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut heard = Vec::new();
            let mut part = [0; 1024];
            while !heard.ends_with(b"PNG-body") {
                let count = stream.read(&mut part).unwrap();
                heard.extend(&part[..count]);
            }
            say.send(String::from_utf8_lossy(&heard).into_owned())
                .unwrap();
            stream.write_all(answer.as_bytes()).unwrap();
        });
        (server, hear)
    }

    #[test]
    fn returns_link_from_server() {
        assert_eq!(
            served(CREATED).0.link(b"PNG-body").unwrap(),
            "https://shots.example/a.p"
        );
    }

    #[test]
    fn sends_key_in_header() {
        let (server, hear) = served(CREATED);
        let _ = server.link(b"PNG-body");
        assert!(hear
            .recv()
            .unwrap()
            .to_lowercase()
            .contains("authorization: bearer secret-key"));
    }

    #[test]
    fn explains_wrong_key() {
        assert_eq!(
            served("HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                .0
                .link(b"PNG-body")
                .unwrap_err()
                .to_string(),
            Phrase::new("error.refused").to_string()
        );
    }

    #[test]
    fn does_not_follow_server_elsewhere() {
        assert!(served(
            "HTTP/1.1 307 Temporary Redirect\r\nLocation: https://evil.example/\r\n\
             Content-Length: 0\r\nConnection: close\r\n\r\n"
        )
        .0
        .link(b"PNG-body")
        .is_err());
    }

    #[test]
    fn refuses_answer_that_is_not_link() {
        assert!(
            served("HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\n<ht>")
                .0
                .link(b"PNG-body")
                .is_err()
        );
    }

    #[test]
    fn is_not_ready_without_key() {
        assert!(!Server {
            address: "https://shots.example/".to_owned(),
            token: String::new()
        }
        .ready());
    }
}
