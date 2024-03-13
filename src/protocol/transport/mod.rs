use std::{
    error::Error,
    net::{SocketAddr, TcpListener, TcpStream},
};

mod client;

pub use client::TransportClient;

/// A transport which can carry a huntsman protocol
pub trait Transport: 'static + Sized {
    /// The error type which this transport can produce
    type Error: 'static + Error;

    /// The type that client sockets take
    type Client: TransportClient<Error = Self::Error>;

    /// Create a new listen socket and bind it to `addr`
    fn bind(addr: SocketAddr) -> Result<Self, Self::Error>;

    /// Get the socket address this socket is bound too
    fn get_socket_address(&mut self) -> Result<SocketAddr, Self::Error>;

    /// Accept a new client on this socket
    fn accept(&mut self) -> Result<(Self::Client, SocketAddr), Self::Error>;
}

impl Transport for TcpListener {
    type Error = std::io::Error;

    type Client = TcpStream;

    fn bind(addr: SocketAddr) -> Result<Self, Self::Error> {
        TcpListener::bind(addr)
    }

    fn get_socket_address(&mut self) -> Result<SocketAddr, Self::Error> {
        self.local_addr()
    }

    fn accept(&mut self) -> Result<(Self::Client, SocketAddr), Self::Error> {
        TcpListener::accept(self)
    }
}
