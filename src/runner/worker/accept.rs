use super::Connections;
use crate::ProtocolListener;
use lasync::FutureQueue;
use std::{num::NonZeroUsize, rc::Rc, sync::Arc};

/// Spawns the tasks to accept clients from the `protocol`'s listener
pub(in crate::runner) fn accept_clients<
    'a,
    Protocol: crate::Protocol,
    App: crate::App<Protocol = Protocol>,
>(
    app: Arc<App>,
    protocol: Arc<Protocol>,
    max_connections: NonZeroUsize,
    future_queue: &FutureQueue<'a>,
) {
    let connections = Connections::new(max_connections);
    for i in 0..protocol.listeners().len() {
        let child_app = app.clone();
        let child_protocol = protocol.clone();
        let child_future_queue = future_queue.clone();
        let child_connections = connections.clone();

        future_queue.push(async move {
            accept_client(
                child_app,
                child_protocol,
                i,
                child_connections,
                child_future_queue,
            )
            .await;
        });
    }
}

/// Asynchronously accepts clients, waiting if the max connections are reached
async fn accept_client<'a, Protocol: crate::Protocol, App: crate::App<Protocol = Protocol>>(
    app: Arc<App>,
    protocol: Arc<Protocol>,
    listener: usize,
    connections: Rc<Connections>,
    future_queue: FutureQueue<'a>,
) -> ! {
    let listener = &protocol.listeners()[listener];

    loop {
        connections.wait_until_available().await;

        let (client_socket, address) = match listener.accept(protocol.options()).await {
            Ok(client) => client,
            Err(error) => {
                app.accept_error(error).await;
                continue;
            }
        };

        let client = match app.on_client_connect(address).await {
            Some(client) => client,
            None => continue,
        };

        connections.new_connection();

        let child_app = app.clone();
        let child_connections = connections.clone();
        future_queue.push(async move {
            super::handle_client(child_app, child_connections, client, client_socket).await;
        });
    }
}
