use super::connections::Connections;
use std::{rc::Rc, sync::Arc};

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
