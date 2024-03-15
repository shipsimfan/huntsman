use crate::TransportClient;
use std::{error::Error, net::SocketAddr};

/// A parser for requests sent by the client
pub trait RequestParser: 'static + Sized {
    /// The client transport to read from
    type TransportClient: TransportClient;

    /// An error while parsing
    type Error: Error;

    /// A request sent from a client
    type Request<'a>: Sized;

    /// Create a new [`RequestParser`] for a client
    fn new(client: &mut Self::TransportClient, address: SocketAddr) -> Result<Self, Self::Error>;

    /// Attempt to parse the next request from the client
    fn parse<'a>(
        &'a mut self,
        client: &mut Self::TransportClient,
    ) -> Result<Self::Request<'a>, Self::Error>;
}
