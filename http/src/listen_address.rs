use std::net::{SocketAddr, ToSocketAddrs};

/// The addresses this server will listen on
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HTTPListenAddress {
    /// The address to listen for insecure HTTP/1.1 connections
    HTTP(SocketAddr),
}

impl HTTPListenAddress {
    /// Creates a new [`ListenAddress`] for insecure HTTP/1.1 connections
    pub fn http<S: ToSocketAddrs>(addr: S) -> std::io::Result<Self> {
        Ok(HTTPListenAddress::HTTP(
            addr.to_socket_addrs()?.next().ok_or(std::io::Error::new(
                std::io::ErrorKind::AddrNotAvailable,
                "could not get address",
            ))?,
        ))
    }
}

impl<'a> std::fmt::Display for HTTPListenAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HTTPListenAddress::HTTP(address) => write!(f, "{} (HTTP/1.1)", address),
        }
    }
}
