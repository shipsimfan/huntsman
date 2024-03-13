use crate::{runner::worker, Protocol, Transport};
use std::{
    net::SocketAddr,
    sync::{
        atomic::AtomicUsize,
        mpsc::{sync_channel, SyncSender},
        Arc,
    },
};

/// A single worker thread which can handle on connection at a time
pub(super) struct Worker<App: crate::App> {
    /// The queue to send new connections
    sender: SyncSender<(
        <<App::Protocol as Protocol>::Transport as Transport>::Client,
        SocketAddr,
    )>,
}

impl<App: crate::App> Worker<App> {
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
            worker::<App>(
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
    pub(super) fn send_connection(
        &mut self,
        connection: (
            <<App::Protocol as Protocol>::Transport as Transport>::Client,
            SocketAddr,
        ),
    ) {
        self.sender.send(connection).unwrap();
    }
}
