use crate::runner::worker;
use std::sync::{
    atomic::AtomicUsize,
    mpsc::{sync_channel, SyncSender},
    Arc,
};

/// A single worker thread which can handle on connection at a time
pub(super) struct Worker<Protocol: crate::Protocol> {
    /// The queue to send new connections
    sender: SyncSender<(Protocol::Client, Protocol::Address)>,
}

impl<Protocol: crate::Protocol> Worker<Protocol> {
    /// Spawns a new [`Worker`] thread
    pub(super) fn spawn<App: crate::App<Protocol = Protocol>>(
        id: usize,
        max_spare_workers: usize,
        spare_worker_count: Arc<AtomicUsize>,
        spare_worker_queue: SyncSender<usize>,
        dead_worker_queue: SyncSender<usize>,
        app: Arc<App>,
    ) -> Self {
        let (sender, connections) = sync_channel(1);

        std::thread::spawn(move || {
            worker(
                id,
                connections,
                max_spare_workers,
                spare_worker_count,
                spare_worker_queue,
                dead_worker_queue,
                app,
            )
        });

        Worker { sender }
    }

    /// Sends a connection to the worker
    pub(super) fn send_connection(&mut self, connection: (Protocol::Client, Protocol::Address)) {
        self.sender.send(connection).unwrap();
    }
}
