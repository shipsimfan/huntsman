use std::num::NonZeroUsize;

/// The settings for the huntsman server
#[derive(Debug, PartialEq, Eq)]
pub struct Options<Protocol: crate::Protocol> {
    /// The number of workers to handle connections
    workers: Option<NonZeroUsize>,

    /// The maximum number of connections a single worker can handle
    connections_per_worker: NonZeroUsize,

    /// The address to listen for connections on
    addresses: Vec<Protocol::ListenAddress>,
}

impl<Protocol: crate::Protocol> Options<Protocol> {
    /// Gets the number of workers to handle connections
    pub fn workers(&self) -> NonZeroUsize {
        self.workers.unwrap_or_else(|| {
            std::thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap())
        })
    }

    /// Gets the maximum number of connections a single worker can handle
    pub fn connections_per_worker(&self) -> NonZeroUsize {
        self.connections_per_worker
    }

    /// Gets the address to listen for connections on
    pub fn addresses(&self) -> &[Protocol::ListenAddress] {
        &self.addresses
    }

    /// Sets the number of workers to handle connections
    pub fn set_workers(&mut self, workers: NonZeroUsize) {
        self.workers = Some(workers);
    }

    /// Sets the maximum connections a single worker can handle
    pub fn set_connections_per_worker(&mut self, connections_per_worker: NonZeroUsize) {
        self.connections_per_worker = connections_per_worker;
    }

    /// Sets the address to listen for connections on
    pub fn add_address(&mut self, address: Protocol::ListenAddress) {
        self.addresses.push(address);
    }

    /// Gets the address to listen for connections on mutably
    pub fn addresses_mut(&mut self) -> &mut Vec<Protocol::ListenAddress> {
        &mut self.addresses
    }
}

impl<Protocol: crate::Protocol> Default for Options<Protocol> {
    fn default() -> Self {
        Options {
            workers: None,
            connections_per_worker: NonZeroUsize::new(64).unwrap(),
            addresses: Vec::new(),
        }
    }
}

impl<
        ListenAddress: crate::ProtocolListener + Clone,
        Protocol: crate::Protocol<ListenAddress = ListenAddress>,
    > Clone for Options<Protocol>
{
    fn clone(&self) -> Self {
        Options {
            workers: self.workers.clone(),
            connections_per_worker: self.connections_per_worker.clone(),
            addresses: self.addresses.clone(),
        }
    }
}
