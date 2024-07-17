use crate::ProtocolClient;
use std::future::Future;

/// A socket which listens for client connections
pub trait ProtocolListener {
    /// The address of a listening socket
    type Address: Send;

    /// The client this listener produces
    type Client: ProtocolClient;

    /// The address of a connecting client
    type ClientAddress: Send;

    /// The error when starting the server and accepting clients
    type Error: std::error::Error;

    /// Options to configure this protocol
    type Options;

    /// Accept a new client on this socket
    fn accept(
        &self,
        options: &Self::Options,
    ) -> impl Future<Output = Result<(Self::Client, Self::ClientAddress), Self::Error>>;
}
