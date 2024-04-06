use crate::Protocol;
use std::{future::Future, sync::Arc};

/// A huntsman application
pub trait App: 'static + Send + Sync {
    /// The protocol this app runs on
    type Protocol: Protocol;

    /// A state for any client that connects
    type Client;

    /// Handle a request from a client
    fn handle_request<'a, 'b>(
        self: &'a Arc<Self>,
        client: &'a mut Self::Client,
        request: <Self::Protocol as Protocol>::Request<'b>,
    ) -> impl Future<Output = <Self::Protocol as Protocol>::Response<'a>>;

    /// Called when the server starts
    #[allow(unused_variables)]
    fn on_server_start(
        self: &Arc<Self>,
        address: <Self::Protocol as Protocol>::ListenAddress,
    ) -> impl Future<Output = ()> {
        async {}
    }

    /// Called when a client connects to the server
    ///
    /// Returns [`None`] if the client should be rejected
    #[allow(unused_variables)]
    fn on_client_connect<'a>(
        self: &'a Arc<Self>,
        source: <Self::Protocol as Protocol>::ClientAddress,
    ) -> impl Future<Output = Option<Self::Client>>;

    /// Called when a client disconnects
    #[allow(unused_variables)]
    fn on_client_disconnect(
        self: &Arc<Self>,
        client: &mut Self::Client,
    ) -> impl Future<Output = ()> {
        async {}
    }

    /// An error occurred while accepting a client
    #[allow(unused_variables)]
    fn accept_error(
        self: &Arc<Self>,
        error: <Self::Protocol as Protocol>::ListenError,
    ) -> impl Future<Output = ()> {
        async {}
    }

    /// An error occurred while parsing a request from a client
    #[allow(unused_variables)]
    fn read_error<'a>(
        self: &'a Arc<Self>,
        client: &'a mut Self::Client,
        error: <Self::Protocol as Protocol>::ReadError,
    ) -> impl Future<Output = Option<<Self::Protocol as Protocol>::Response<'a>>> {
        async { None }
    }

    /// An error occurred while sending the response
    #[allow(unused_variables)]
    fn send_error(
        self: &Arc<Self>,
        client: &mut Self::Client,
        error: <Self::Protocol as Protocol>::SendError,
    ) -> impl Future<Output = ()> {
        async {}
    }
}
