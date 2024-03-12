use std::{
    error::Error,
    io::{Read, Write},
    net::TcpStream,
};

/// A client connection
pub trait TransportClient: 'static + Sized + Read + Write + Send {
    /// The error type this client can produce
    type Error: 'static + Error;
}

impl TransportClient for TcpStream {
    type Error = std::io::Error;
}
