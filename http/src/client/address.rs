use std::{fmt::Display, net::SocketAddr};

/// The protocol being used by the client
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HTTPProtocol {
    /// Unsecure HTTP/1.1
    HTTP,

    /// Secured HTTP/1.1
    HTTPS,

    /// HTTP/2.0
    HTTP2,

    /// HTTP/3.0
    HTTP3,
}

/// The address a client connected with
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HTTPClientAddress {
    /// The protocol the client is using
    protocol: HTTPProtocol,

    /// The address the client connected with
    socket_address: SocketAddr,
}

impl HTTPClientAddress {
    /// Creates a new [`HTTPClientAddress`]
    pub(crate) fn new(protocol: HTTPProtocol, socket_address: SocketAddr) -> Self {
        HTTPClientAddress {
            protocol,
            socket_address,
        }
    }

    /// Gets the protocol the client connected with
    pub fn protocol(&self) -> HTTPProtocol {
        self.protocol
    }

    /// Gets the address the client connected with
    pub fn socket_address(&self) -> SocketAddr {
        self.socket_address
    }
}

impl Display for HTTPClientAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.socket_address, self.protocol)
    }
}

impl Display for HTTPProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            HTTPProtocol::HTTP => "HTTP/1.1",
            HTTPProtocol::HTTPS => "HTTPS/1.1",
            HTTPProtocol::HTTP2 => "HTTP/2.0",
            HTTPProtocol::HTTP3 => "HTTP/3.0",
        })
    }
}
