use super::connections::Connections;
use std::{rc::Rc, sync::Arc};

/// A function which handles a client until an error occurs or a client disconnects
pub(super) async fn handle_client<
    Protocol: crate::Protocol,
    App: crate::App<Protocol = Protocol>,
>(
    app: Arc<App>,
    connections: Rc<Connections>,
    client: App::Client,
    client_socket: Protocol::Client,
) {
    todo!("handle_client()");
}
