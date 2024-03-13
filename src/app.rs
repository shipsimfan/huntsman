use crate::{Protocol, RequestParser, Transport};
use std::{net::SocketAddr, sync::Arc};

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
        request: <<Self::Protocol as Protocol>::RequestParser as RequestParser>::Request<'a>,
    ) -> <Self::Protocol as Protocol>::Response;

    /// Called when the server starts
    #[allow(unused_variables)]
    fn on_server_start(self: &Arc<Self>, address: SocketAddr) {}

    /// Called when a client connects to the server
    ///
    /// Returns [`None`] if the client should be rejected
    #[allow(unused_variables)]
    fn on_client_connect(self: &Arc<Self>, source: SocketAddr) -> Option<Self::Client>;

    /// Called when a client disconnects
    #[allow(unused_variables)]
    fn on_client_disconnect(self: &Arc<Self>, client: &mut Self::Client) {}

    /// An error occurred while accepting a client
    #[allow(unused_variables)]
    fn accept_error(
        self: &Arc<Self>,
        error: <<Self::Protocol as Protocol>::Transport as Transport>::Error,
    ) {
    }

    /// An error occurred while parsing a request from a client
    #[allow(unused_variables)]
    fn parse_error(
        self: &Arc<Self>,
        client: &mut Self::Client,
        error: <<Self::Protocol as Protocol>::RequestParser as RequestParser>::Error,
    ) -> Option<<Self::Protocol as Protocol>::Response> {
        None
    }

    /// An error occurred while sending the response
    #[allow(unused_variables)]
    fn send_error(
        self: &Arc<Self>,
        client: &mut Self::Client,
        error: <<Self::Protocol as Protocol>::Transport as Transport>::Error,
    ) {
    }
}
