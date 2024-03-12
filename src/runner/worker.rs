use crate::{Result, Transport};
use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{Receiver, SyncSender},
        Arc,
    },
};

/// The function which worker threads run
pub(super) fn worker<Protocol: crate::Protocol>(
    id: usize,
    connections: Receiver<(<Protocol::Transport as Transport>::Client, SocketAddr)>,
    max_spare_workers: usize,
    spare_worker_count: Arc<AtomicUsize>,
    spare_worker_queue: SyncSender<usize>,
    dead_worker_queue: SyncSender<usize>,
) {
    // Handle incoming connections
    loop {
        let (connection, address) = connections.recv().unwrap();

        match handle_connection::<Protocol>(connection, address) {
            Ok(()) => {}
            Err(error) => eprintln!("Error while handling client connection: {}", error),
        }

        // Check if this thread should die or if it should make itself available for more
        // connections
        let spare_count = spare_worker_count.fetch_add(1, Ordering::AcqRel);
        if spare_count >= max_spare_workers {
            spare_worker_count.fetch_sub(1, Ordering::Release);
            dead_worker_queue.send(id).unwrap();
            return;
        }

        spare_worker_queue.send(id).unwrap();
    }
}

/// Handle an incoming connection from a client
fn handle_connection<Protocol: crate::Protocol>(
    connection: <Protocol::Transport as Transport>::Client,
    address: SocketAddr,
) -> Result<(), Protocol> {
    println!("Client connected from {}", address);
    Ok(())
}
