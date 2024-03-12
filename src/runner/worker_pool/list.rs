use std::sync::{atomic::AtomicUsize, mpsc::SyncSender, Arc};

use super::Worker;

/// The list of current workers
pub(super) struct WorkerList {
    /// The list of workers
    workers: Box<[Node]>,

    /// The index of the first free node
    first_free: Option<usize>,
}

/// A node in the list of workers
enum Node {
    /// An unused node, containing the index of the next free node
    Free(Option<usize>),

    /// A used node containing a worker
    Used(Worker),
}

impl WorkerList {
    /// Creates a new empty [`WorkerList`]
    pub(super) fn new(max_workers: usize) -> Self {
        let mut workers = Vec::with_capacity(max_workers);
        for i in 1..max_workers {
            workers.push(Node::Free(Some(i)));
        }
        workers.push(Node::Free(None));

        WorkerList {
            workers: workers.into_boxed_slice(),
            first_free: Some(0),
        }
    }

    /// Gets a [`Worker`] based on its `id`
    pub(super) fn get(&mut self, id: usize) -> &mut Worker {
        match &mut self.workers[id] {
            Node::Used(worker) => worker,
            Node::Free(_) => panic!("Attempting to get the worker of a free node"),
        }
    }

    /// Spawns a new [`Worker`] and returns its id
    pub(super) fn spawn(
        &mut self,
        max_spare_workers: usize,
        spare_worker_count: Arc<AtomicUsize>,
        spare_worker_queue: SyncSender<usize>,
        dead_worker_queue: SyncSender<usize>,
    ) -> Option<usize> {
        let first_free = self.first_free?;

        self.first_free = match self.workers[first_free] {
            Node::Free(next_free) => next_free,
            _ => unreachable!(),
        };

        self.workers[first_free] = Node::Used(Worker::spawn(
            first_free,
            max_spare_workers,
            spare_worker_count,
            spare_worker_queue,
            dead_worker_queue,
        ));

        Some(first_free)
    }

    /// Frees a worker from the list
    pub(super) fn free(&mut self, id: usize) {
        match self.workers[id] {
            Node::Free(_) => panic!("Attempting to free a free node"),
            _ => {}
        }

        self.workers[id] = Node::Free(self.first_free.take());
        self.first_free = Some(id);
    }
}
