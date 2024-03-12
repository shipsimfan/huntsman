use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::NonZeroUsize,
};

/// The settings for the huntsman server
pub struct Options {
    /// The address to listen on
    address: IpAddr,

    /// The port to listen on
    port: u16,

    /// The maximum number of connections
    max_connections: NonZeroUsize,

    /// The initial number of threads to spawn
    initial_threads: NonZeroUsize,

    /// The minimum number of spare threads to keep in the pool
    min_spare_threads: usize,

    /// The maximum number of spare threads to keep in the pool
    max_spare_threads: Option<usize>,
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

    /// Gets the maximum number of connections
    pub fn max_connections(&self) -> NonZeroUsize {
        self.max_connections
    }

    /// Gets the initial number of threads to spawn
    pub fn initial_threads(&self) -> NonZeroUsize {
        self.initial_threads.max(
            NonZeroUsize::new(self.min_spare_threads()).unwrap_or(NonZeroUsize::new(1).unwrap()),
        )
    }

    /// Gets the minimum number of spare threads to keep around for connections
    pub fn min_spare_threads(&self) -> usize {
        self.min_spare_threads.min(self.max_connections.get() - 1)
    }

    /// Gets the maximum number of spare threads to keep around for connections
    pub fn max_spare_threads(&self) -> usize {
        self.max_spare_threads
            .unwrap_or((self.max_connections.get() + 1) / 2)
            .max(self.min_spare_threads())
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
    pub fn set_max_connections(&mut self, max_connections: NonZeroUsize) {
        self.max_connections = max_connections;
    }

    /// Sets the number of initial threads to handle connections
    pub fn set_initial_threads(&mut self, initial_threads: NonZeroUsize) {
        self.initial_threads = initial_threads;
    }

    /// Sets the minimum number of spare threads to handle connections
    pub fn set_min_spare_threads(&mut self, min_spare_threads: usize) {
        self.min_spare_threads = min_spare_threads;
    }

    /// Sets the maximum number of spare threads to handle connections
    pub fn set_max_spare_threads(&mut self, max_spare_threads: usize) {
        self.max_spare_threads = Some(max_spare_threads);
    }

    /// Gets the [`SocketAddr`] to listen on
    pub(crate) fn socket_address(&self) -> SocketAddr {
        (self.address, self.port).into()
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 3000,
            max_connections: NonZeroUsize::new(256).unwrap(),
            initial_threads: NonZeroUsize::new(32).unwrap(),
            min_spare_threads: 16,
            max_spare_threads: None,
        }
    }
}
