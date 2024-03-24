mod client;

use std::future::Future;

pub use client::ProtocolClient;

/// A protocol which huntsman can run a server for
pub trait Protocol: 'static + Sized + Send + Sync {
    /// Options to configure this protocol
    type Options;

    /// The address of a connecting client
    type ClientAddress: Send;

    /// The address of a listening socket
    type ListenAddress: Send + Default;

    /// Parser for requests from a client
    type Request<'a>;

    /// Responses sent to the client
    type Response;

    /// The error when starting the server and accepting clients
    type ListenError: std::error::Error;

    /// The error when reading and parsing requests
    type ReadError: std::error::Error;

    /// The error when sending responses
    type SendError: std::error::Error;

    /// The client for this protocol
    type Client: for<'a> ProtocolClient<
        Request<'a> = Self::Request<'a>,
        Response = Self::Response,
        ReadError = Self::ReadError,
        SendError = Self::SendError,
    >;

    /// Create a new socket listening on `address` with `options`
    fn start(
        address: Self::ListenAddress,
        options: &Self::Options,
    ) -> impl Future<Output = Result<Self, Self::ListenError>>;

    /// Get the addresses this listen socket is bound too
    fn address(&mut self) -> impl Future<Output = Self::ListenAddress>;

    /// Accept a new client on this socket
    fn accept(
        &mut self,
    ) -> impl Future<Output = Result<(Self::Client, Self::ClientAddress), Self::ListenError>>;
}
