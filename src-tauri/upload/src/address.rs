//! Where the key may be sent.

/// An address typed by a person or received from a server.
pub struct Address<'a> {
    text: &'a str,
}

impl<'a> Address<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    /// True if the key may be sent there: the connection is encrypted or stays on this computer.
    pub fn safe(&self) -> bool {
        if self
            .text
            .chars()
            .any(|sign| sign.is_whitespace() || sign.is_control())
        {
            return false;
        }
        if let Some(rest) = self.text.strip_prefix("https://") {
            let host = host(rest);
            return !host.is_empty() && !host.contains('@');
        }
        self.text.strip_prefix("http://").is_some_and(|rest| {
            let host = host(rest);
            let name = host.split(':').next().unwrap_or_default();
            !host.contains('@') && (name == "localhost" || name == "127.0.0.1")
        })
    }
}

/// The host with the port, if there is one.
fn host(rest: &str) -> &str {
    rest.split(['/', '?', '#']).next().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::Address;

    #[test]
    fn trusts_encrypted_address() {
        assert!(Address::new("https://shots.example/s").safe());
    }

    #[test]
    fn trusts_this_computer() {
        assert!(Address::new("http://127.0.0.1:5397/").safe());
    }

    #[test]
    fn does_not_trust_open_address() {
        assert!(!Address::new("http://shots.example/s").safe());
    }

    #[test]
    fn does_not_trust_lookalike_of_this_computer() {
        assert!(!Address::new("http://localhost.evil.example/").safe());
    }

    #[test]
    fn does_not_trust_address_with_name_before_host() {
        assert!(!Address::new("http://localhost:1@evil.example/").safe());
    }

    #[test]
    fn does_not_trust_address_without_host() {
        assert!(!Address::new("https://").safe());
    }

    #[test]
    fn does_not_trust_address_with_line_break() {
        assert!(!Address::new("https://shots.example/\r\nX: y").safe());
    }
}
