mod transport;

pub use transport::{Transport, TransportClient};

/// A protocol which huntsman can run a server for
pub trait Protocol {
    /// The transport used by this protocol
    type Transport: Transport;
}
