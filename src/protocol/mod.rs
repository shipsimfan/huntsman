mod client;

pub use client::ProtocolClient;

/// A protocol which huntsman can run a server for
pub trait Protocol: 'static + Sized {
    /// Options to configure this protocol
    type Options;

    /// The address of a connecting client
    type Address: Send;

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

    /// Create a new listen socket with `options`
    fn start(options: Self::Options) -> Result<Self, Self::ListenError>;

    /// Get the addresses this socket is bound too
    fn get_addresses(&mut self) -> &[Self::Address];

    /// Accept a new client on this socket
    fn accept(&mut self) -> Result<(Self::Client, Self::Address), Self::ListenError>;
}
