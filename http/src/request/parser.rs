use super::{HTTPHeaderBuffer, Stream};
use crate::{HTTPParseError, HTTPRequest};
use huntsman::RequestParser;
use std::net::TcpStream;

/// A parser for HTTP requests
pub struct HTTPRequestParser {
    buffer: HTTPHeaderBuffer,
}

const BUFFER_SIZE: usize = 8192;

impl RequestParser for HTTPRequestParser {
    type TransportClient = TcpStream;

    type Error = HTTPParseError;

    type Request<'a> = HTTPRequest;

    fn new(_: &mut Self::TransportClient, _: std::net::SocketAddr) -> Result<Self, Self::Error> {
        let buffer = HTTPHeaderBuffer::new(BUFFER_SIZE);

        Ok(HTTPRequestParser { buffer })
    }

    fn parse<'a>(&'a mut self, client: &mut TcpStream) -> Result<HTTPRequest, HTTPParseError> {
        let mut stream = Stream::new(&mut self.buffer, client);

        HTTPRequest::parse(&mut stream)
    }
}
