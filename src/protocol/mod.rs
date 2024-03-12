mod request;
mod response;
mod transport;

pub use request::RequestParser;
pub use response::Response;
pub use transport::{Transport, TransportClient};

/// A protocol which huntsman can run a server for
pub trait Protocol {
    /// The transport used by this protocol
    type Transport: Transport;

    /// Parser for requests from a client
    type RequestParser: RequestParser<TransportClient = <Self::Transport as Transport>::Client>;

    /// Responses sent to the client
    type Response: Response;
}
