use super::{Buffer, Stream};
use crate::{HTTPParseError, HTTPRequest};
use huntsman::RequestParser;
use std::net::TcpStream;

/// A parser for HTTP requests
pub struct HTTPRequestParser {
    /// The buffer to improve read efficiency
    buffer: Buffer,
}

const BUFFER_SIZE: usize = 8192;

impl RequestParser for HTTPRequestParser {
    type TransportClient = TcpStream;

    type Error = HTTPParseError;

    type Request<'a> = HTTPRequest;

    fn new(_: &mut Self::TransportClient, _: std::net::SocketAddr) -> Result<Self, Self::Error> {
        let buffer = Buffer::new(BUFFER_SIZE);

        Ok(HTTPRequestParser { buffer })
    }

    fn parse<'a>(&'a mut self, client: &mut TcpStream) -> Result<HTTPRequest, HTTPParseError> {
        let stream = Stream::new(&mut self.buffer, client);

        todo!("HTTPRequestParser::parse()");
    }
}
