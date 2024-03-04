use std::net::{IpAddr, SocketAddr};

/// The settings for the huntsman server
pub struct Options {
    /// The address to listen on
    address: IpAddr,

    /// The port to listen on
    port: u16,

    /// The maximum number of connections a worker can have
    max_worker_connections: usize,
}

impl Options {
    /// Gets the address that the server will listen on
    pub fn address(&self) -> IpAddr {
        self.address
    }

    /// Gets the port the server will listen on
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Gets the maximum number of connections a worker can handle
    pub fn max_worker_connections(&self) -> usize {
        self.max_worker_connections
    }

    /// Sets the address the server will listen on
    pub fn set_address(&mut self, address: IpAddr) {
        self.address = address;
    }

    /// Sets the port the server will listen on
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    /// Sets the maximum connections a worker can handle
    ///
    /// # Panic
    /// This function will panic if `max_worker_connections` is greater than 1024
    pub fn set_max_worker_connections(&mut self, max_worker_connections: usize) {
        assert!(max_worker_connections <= 1024);

        self.max_worker_connections = max_worker_connections;
    }

    /// Gets the [`SocketAddr`] to listen on
    pub(crate) fn socket_address(&self) -> SocketAddr {
        (self.address, self.port).into()
    }
}
