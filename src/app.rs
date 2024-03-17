use crate::Protocol;
use std::sync::Arc;

/// A huntsman application
pub trait App: 'static + Send + Sync {
    /// The protocol this app runs on
    type Protocol: Protocol;

    /// A state for any client that connects
    type Client;

    /// Handle a request from a client
    fn handle_request<'a>(
        self: &Arc<Self>,
        client: &mut Self::Client,
        request: <Self::Protocol as Protocol>::Request<'a>,
    ) -> <Self::Protocol as Protocol>::Response;

    /// Called when the server starts
    #[allow(unused_variables)]
    fn on_server_start(self: &Arc<Self>, addresses: &[<Self::Protocol as Protocol>::Address]) {}

    /// Called when a client connects to the server
    ///
    /// Returns [`None`] if the client should be rejected
    #[allow(unused_variables)]
    fn on_client_connect(
        self: &Arc<Self>,
        source: <Self::Protocol as Protocol>::Address,
    ) -> Option<Self::Client>;

    /// Called when a client disconnects
    #[allow(unused_variables)]
    fn on_client_disconnect(self: &Arc<Self>, client: &mut Self::Client) {}

    /// An error occurred while accepting a client
    #[allow(unused_variables)]
    fn accept_error(self: &Arc<Self>, error: <Self::Protocol as Protocol>::ListenError) {}

    /// An error occurred while parsing a request from a client
    #[allow(unused_variables)]
    fn read_error(
        self: &Arc<Self>,
        client: &mut Self::Client,
        error: <Self::Protocol as Protocol>::ReadError,
    ) -> Option<<Self::Protocol as Protocol>::Response> {
        None
    }

    /// An error occurred while sending the response
    #[allow(unused_variables)]
    fn send_error(
        self: &Arc<Self>,
        client: &mut Self::Client,
        error: <Self::Protocol as Protocol>::SendError,
    ) {
    }
}
