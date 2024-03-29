use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

/// The options to determine how HTTP will operate
#[derive(Clone, PartialEq, Eq)]
pub struct HTTPOptions {
    /// The address to listen on
    pub address: SocketAddr,

    /// The maximum size for HTTP headers in requests
    pub max_header_size: usize,

    /// The maximum size for bodies in HTTP requests
    pub max_body_size: usize,
}

/// The default port to listen on
#[cfg(debug_assertions)]
const DEFAULT_PORT: u16 = 3000;

/// The default port to listen on
#[cfg(not(debug_assertions))]
const DEFAULT_PORT: u16 = 80;

impl Default for HTTPOptions {
    fn default() -> Self {
        HTTPOptions {
            address: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, DEFAULT_PORT)), // 0.0.0.0:80
            max_header_size: 8192,                                                           // 8 Kb
            max_body_size: 1024 * 1024,                                                      // 1 Mb
        }
    }
}
