use std::num::NonZeroUsize;

/// The settings for the huntsman server
pub struct Options {
    /// The maximum number of connections
    max_connections: NonZeroUsize,

    /// The initial number of workers to spawn
    initial_workers: NonZeroUsize,

    /// The minimum number of spare workers to keep in the pool
    min_spare_workers: usize,

    /// The maximum number of spare workers to keep in the pool
    max_spare_workers: Option<usize>,
}

impl Options {
    /// Gets the maximum number of connections
    pub fn max_connections(&self) -> NonZeroUsize {
        self.max_connections
    }

    /// Gets the initial number of workers to spawn
    pub fn initial_workers(&self) -> NonZeroUsize {
        self.initial_workers.max(
            NonZeroUsize::new(self.min_spare_workers()).unwrap_or(NonZeroUsize::new(1).unwrap()),
        )
    }

    /// Gets the minimum number of spare workers to keep around for connections
    pub fn min_spare_workers(&self) -> usize {
        self.min_spare_workers.min(self.max_connections.get() - 1)
    }

    /// Gets the maximum number of spare workers to keep around for connections
    pub fn max_spare_workers(&self) -> usize {
        self.max_spare_workers
            .unwrap_or((self.max_connections.get() + 1) / 2)
            .max(self.min_spare_workers())
    }

    /// Sets the maximum connections a worker can handle
    pub fn set_max_connections(&mut self, max_connections: NonZeroUsize) {
        self.max_connections = max_connections;
    }

    /// Sets the number of initial workers to handle connections
    pub fn set_initial_workers(&mut self, initial_workers: NonZeroUsize) {
        self.initial_workers = initial_workers;
    }

    /// Sets the maximum number of spare workers to handle connections
    pub fn set_max_spare_workers(&mut self, max_spare_workers: usize) {
        self.max_spare_workers = Some(max_spare_workers);
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            max_connections: NonZeroUsize::new(256).unwrap(),
            initial_workers: NonZeroUsize::new(32).unwrap(),
            min_spare_workers: 16,
            max_spare_workers: None,
        }
    }
}
