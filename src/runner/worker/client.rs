use super::Connections;
use crate::ProtocolClient;
use std::{rc::Rc, sync::Arc};

/// A function which handles a client until an error occurs or a client disconnects
pub(super) async fn handle_client<
    Protocol: crate::Protocol,
    App: crate::App<Protocol = Protocol>,
>(
    app: Arc<App>,
    connections: Rc<Connections>,
    mut client: App::Client,
    mut client_socket: Protocol::Client,
) {
    let mut response = None;

    loop {
        let request = match client_socket.read().await {
            Ok(request) => match request {
                Some(request) => request,
                None => break,
            },
            Err(error) => {
                response = app.read_error(&mut client, error).await;
                break;
            }
        };

        let response = app.handle_request(&mut client, request).await;

        if let Err(error) = client_socket.send(response).await {
            app.send_error(&mut client, error).await;
            break;
        }
    }

    let send_result = match response.take() {
        Some(response) => client_socket.send(response).await,
        None => Ok(()),
    };
    drop(response);

    if let Err(error) = send_result {
        app.send_error(&mut client, error).await;
    }

    connections.end_connection();
    app.on_client_disconnect(&mut client).await;
}
