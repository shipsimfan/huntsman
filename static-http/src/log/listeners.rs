use huntsman_http::ListenAddress;

/// Displays listener sockets for logging
pub struct ListenerDisplay<'a>(pub &'a ListenAddress);

impl<'a> std::fmt::Display for ListenerDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Server listening on:")?;

        if let Some(http) = &self.0.http {
            write!(f, "\n - {} (HTTP/1.1)", http)?;
        }

        Ok(())
    }
}
