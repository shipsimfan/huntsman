use lasync::executor::FutureQueue;
use std::num::NonZeroUsize;
use worker::Worker;

mod worker;

/// Controller of workers
pub(super) struct Workers {
    /// The workers themselves
    workers: Box<[Worker]>,

    /// The maximum number of connections each worker can have
    max_worker_connections: NonZeroUsize,
}

impl Workers {
    /// Creates a new [`Workers`] manager and spawns the workers to be managed
    pub(super) fn new(
        workers: NonZeroUsize,
        max_worker_connections: NonZeroUsize,
        queue: FutureQueue,
    ) -> Self {
        // TODO: Support multiple workers
        assert_eq!(workers.get(), 1);

        let mut workers = Vec::with_capacity(workers.get());
        workers.push(Worker::new_local(queue));

        Workers {
            workers: workers.into_boxed_slice(),
            max_worker_connections,
        }
    }
}
