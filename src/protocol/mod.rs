use std::future::Future;

mod client;
mod listener;

pub use client::ProtocolClient;
pub use listener::ProtocolListener;

/// A protocol which huntsman can run a server for
pub trait Protocol: 'static + Sized + Send + Sync {
    /// Options to configure this protocol
    type Options;

    /// The address of a connecting client
    type ClientAddress: Send;

    /// Parser for requests from a client
    type Request<'a>;

    /// Responses sent to the client
    type Response<'a>;

    /// The error when reading and parsing requests
    type ReadError: std::error::Error;

    /// The error when sending responses
    type SendError: std::error::Error;

    /// The client for this protocol
    type Client: for<'a, 'b> ProtocolClient<
        Request<'a> = Self::Request<'a>,
        Response<'b> = Self::Response<'b>,
        ReadError = Self::ReadError,
        SendError = Self::SendError,
    >;

    /// The address of a listening socket
    type ListenAddress: Send;

    /// The error when starting the server and accepting clients
    type ListenError: std::error::Error;

    /// A socket which listens for connections
    type Listener: ProtocolListener<
        Address = Self::ListenAddress,
        Error = Self::ListenError,
        Client = Self::Client,
        ClientAddress = Self::ClientAddress,
        Options = Self::Options,
    >;

    /// Create a new socket listening on `address` with `options`
    fn start(
        addresses: &[Self::ListenAddress],
        options: Self::Options,
    ) -> impl Future<Output = Result<Self, Self::ListenError>>;

    /// Get the addresses this listen socket is bound too
    fn addresses(&self) -> &[Self::ListenAddress];

    /// Gets the listening sockets
    fn listeners(&self) -> &[Self::Listener];

    /// Gets the options used to create this server
    fn options(&self) -> &Self::Options;
}
