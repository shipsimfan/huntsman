use std::{error::Error, net::TcpStream};

/// A client connection
pub trait TransportClient: 'static + Sized + Send {
    /// The error type this client can produce
    type Error: 'static + Error;
}

impl TransportClient for TcpStream {
    type Error = std::io::Error;
}
