mod request;
mod response;
mod transport;

pub use request::Request;
pub use response::Response;
pub use transport::{Transport, TransportClient};

/// A protocol which huntsman can run a server for
pub trait Protocol {
    /// The transport used by this protocol
    type Transport: Transport;

    /// Request received from the client
    type Request: Request;

    /// Responses sent to the client
    type Response: Response;
}
