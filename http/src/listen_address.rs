use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};

#[cfg(debug_assertions)]
/// The default port for insecure HTTP/1.1 connections
const DEFAULT_HTTP_PORT: u16 = 8080;

#[cfg(not(debug_assertions))]
/// The default port for insecure HTTP/1.1 connections
const DEFAULT_HTTP_PORT: u16 = 80;

/// The addresses this server will listen on
#[derive(Clone)]
pub struct ListenAddress {
    /// The address to listen for insecure HTTP/1.1 connections on over IPv4
    pub http: Option<SocketAddr>,
}

impl ListenAddress {
    /// Creates a [`ListenAddress`] with all addresses set to [`None`]
    pub const fn empty() -> Self {
        ListenAddress { http: None }
    }
}

impl Default for ListenAddress {
    fn default() -> Self {
        ListenAddress {
            http: Some(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, DEFAULT_HTTP_PORT, 0, 0).into()),
        }
    }
}

impl<'a> std::fmt::Display for ListenAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Server listening on:")?;

        if let Some(http) = &self.http {
            write!(f, "\n - {} (HTTP/1.1)", http)?;
        }

        Ok(())
    }
}
