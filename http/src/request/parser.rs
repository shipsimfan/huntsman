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
    type Client = TcpStream;

    type Error = HTTPParseError;

    type Request<'a> = HTTPRequest<'a>;

    fn new(_: &mut Self::Client, _: std::net::SocketAddr) -> Result<Self, Self::Error> {
        let buffer = HTTPHeaderBuffer::new(BUFFER_SIZE);

        Ok(HTTPRequestParser { buffer })
    }

    fn parse<'a>(&'a mut self, client: &mut TcpStream) -> Result<HTTPRequest<'a>, HTTPParseError> {
        let stream = Stream::new(&mut self.buffer, client);

        HTTPRequest::parse(stream)
    }
}
