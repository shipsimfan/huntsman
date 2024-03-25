use super::Connections;
use lasync::executor::FutureQueue;
use std::{num::NonZeroUsize, sync::Arc};

/// Asynchronously accepts clients, waiting if the max connections are reached
pub(in crate::runner) async fn accept_clients<
    'a,
    Protocol: crate::Protocol,
    App: crate::App<Protocol = Protocol>,
>(
    app: Arc<App>,
    listener: Arc<Protocol>,
    max_connections: NonZeroUsize,
    future_queue: FutureQueue<'a>,
) -> ! {
    let connections = Connections::new(max_connections);

    loop {
        connections.wait_until_available().await;

        let (client_socket, address) = match listener.accept().await {
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
