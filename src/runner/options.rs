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

    /// The maximum number of connections a worker can have
    max_worker_connections: NonZeroUsize,

    /// The maximum number of workers
    workers: NonZeroUsize,
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
    pub fn max_worker_connections(&self) -> NonZeroUsize {
        self.max_worker_connections
    }

    /// Gets the number of workers
    pub fn workers(&self) -> NonZeroUsize {
        self.workers
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
    pub fn set_max_worker_connections(&mut self, max_worker_connections: NonZeroUsize) {
        assert!(max_worker_connections.get() <= 1024);

        self.max_worker_connections = max_worker_connections;
    }

    /// Sets the number of workers
    pub fn set_workers(&mut self, workers: NonZeroUsize) {
        assert_eq!(
            workers.get(),
            1,
            "More than one worker is currently not supported!"
        );

        self.workers = workers;
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
            port: 80,
            max_worker_connections: NonZeroUsize::new(1024).unwrap(),
            workers: NonZeroUsize::new(1).unwrap(),
        }
    }
}
