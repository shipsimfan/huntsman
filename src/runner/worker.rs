use crate::ProtocolClient;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc::{Receiver, SyncSender},
    Arc,
};

/// The function which worker threads run
pub(super) fn worker<Protocol: crate::Protocol, App: crate::App<Protocol = Protocol>>(
    id: usize,
    connections: Receiver<(Protocol::Client, Protocol::Address)>,
    max_spare_workers: usize,
    spare_worker_count: Arc<AtomicUsize>,
    spare_worker_queue: SyncSender<usize>,
    dead_worker_queue: SyncSender<usize>,
    app: Arc<App>,
) {
    // Handle incoming connections
    loop {
        let (connection, address) = connections.recv().unwrap();

        match handle_connection(connection, address, &app) {
            Some(mut client) => app.on_client_disconnect(&mut client),
            None => {}
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
fn handle_connection<Protocol: crate::Protocol, App: crate::App<Protocol = Protocol>>(
    mut connection: Protocol::Client,
    address: Protocol::Address,
    app: &Arc<App>,
) -> Option<App::Client> {
    let mut client = match app.on_client_connect(address) {
        Some(client) => client,
        None => return None,
    };

    let response = loop {
        let request = match connection.read() {
            Ok(result) => result,
            Err(error) => {
                if let Some(response) = app.read_error(&mut client, error) {
                    break Some(response);
                }

                break None;
            }
        };

        let response = app.handle_request(&mut client, request);

        if let Err(error) = connection.send(response) {
            app.send_error(&mut client, error);
            break None;
        }
    };

    if let Some(response) = response {
        if let Err(error) = connection.send(response) {
            app.send_error(&mut client, error);
        }
    }

    Some(client)
}
