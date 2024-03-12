use crate::runner::worker;
use std::{
    net::{SocketAddr, TcpStream},
    sync::{
        atomic::AtomicUsize,
        mpsc::{sync_channel, SyncSender},
        Arc,
    },
};

/// A single worker thread which can handle on connection at a time
pub(super) struct Worker {
    /// The queue to send new connections
    sender: SyncSender<(TcpStream, SocketAddr)>,
}

impl Worker {
    /// Spawns a new [`Worker`] thread
    pub(super) fn spawn(
        id: usize,
        max_spare_workers: usize,
        spare_worker_count: Arc<AtomicUsize>,
        spare_worker_queue: SyncSender<usize>,
        dead_worker_queue: SyncSender<usize>,
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
            )
        });

        Worker { sender }
    }

    /// Sends a connection to the worker
    pub(super) fn send_connection(&mut self, connection: (TcpStream, SocketAddr)) {
        self.sender.send(connection).unwrap();
    }
}
