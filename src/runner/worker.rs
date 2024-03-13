use crate::{Protocol, RequestParser, Response, Transport};
use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{Receiver, SyncSender},
        Arc,
    },
};

/// The function which worker threads run
pub(super) fn worker<App: crate::App>(
    id: usize,
    connections: Receiver<(
        <<App::Protocol as Protocol>::Transport as Transport>::Client,
        SocketAddr,
    )>,
    max_spare_workers: usize,
    spare_worker_count: Arc<AtomicUsize>,
    spare_worker_queue: SyncSender<usize>,
    dead_worker_queue: SyncSender<usize>,
    app: Arc<App>,
) {
    // Handle incoming connections
    loop {
        let (connection, address) = connections.recv().unwrap();

        match handle_connection::<App>(connection, address, &app) {
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

/// Try `expr` and if it fails, inform the app and handle a response if given one
macro_rules! r#try {
    ($client: expr, $connection: expr, $app: expr, $expr: expr) => {
        match $expr {
            Ok(result) => result,
            Err(error) => {
                if let Some(response) = $app.parse_error(&mut $client, error) {
                    if let Err(error) = response.send(&mut $connection) {
                        $app.send_error(&mut $client, error);
                    }
                }

                return Some($client);
            }
        }
    };
}

/// Handle an incoming connection from a client
fn handle_connection<App: crate::App>(
    mut connection: <<App::Protocol as Protocol>::Transport as Transport>::Client,
    address: SocketAddr,
    app: &Arc<App>,
) -> Option<App::Client> {
    let mut client = match app.on_client_connect(address) {
        Some(client) => client,
        None => return None,
    };

    let mut parser = r#try!(
        client,
        connection,
        app,
        <<App::Protocol as Protocol>::RequestParser as RequestParser>::new(
            &mut connection,
            address,
        )
    );

    loop {
        let request = r#try!(client, connection, app, parser.parse(&mut connection));

        let response = app.handle_request(&mut client, request);

        if let Err(error) = response.send(&mut connection) {
            app.send_error(&mut client, error);
            return Some(client);
        }
    }
}
