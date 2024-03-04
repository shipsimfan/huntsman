use lasync::executor::FutureQueue;
use std::sync::{atomic::AtomicUsize, Arc};

/// Controller for a single worker
pub(super) struct Worker {
    /// The current number of connections the worker is handling
    current_connections: Arc<AtomicUsize>,

    /// The queue to signal new connections on
    ///
    /// TODO: Add pipe for non-local workers
    queue: FutureQueue,
}

impl Worker {
    /// Create a [`Worker`] representing the local thread
    pub(super) fn new_local(queue: FutureQueue) -> Self {
        Worker {
            current_connections: Arc::new(AtomicUsize::new(0)),
            queue,
        }
    }
}
