use buffer::HeaderBuffer;
use huntsman::ProtocolClient;
use std::net::TcpStream;

mod buffer;
mod stream;

pub(crate) use stream::Stream;

use crate::{HTTPParseError, HTTPRequest, HTTPResponse};

/// A client connected to the server
pub struct HTTPClient {
    /// The socket representing the underlying connection
    socket: TcpStream,

    /// The buffer for more efficient header reading and parsing
    buffer: HeaderBuffer,

    /// The maximum size for request bodies
    max_body_size: usize,
}

impl HTTPClient {
    /// Creates a new [`HTTPClient`]
    pub(crate) fn new(socket: TcpStream, max_header_size: usize, max_body_size: usize) -> Self {
        let buffer = HeaderBuffer::new(max_header_size);

        HTTPClient {
            socket,
            buffer,
            max_body_size,
        }
    }
}

impl ProtocolClient for HTTPClient {
    type ReadError = HTTPParseError;

    type SendError = std::io::Error;

    type Request<'a> = HTTPRequest<'a>;

    type Response = HTTPResponse;

    fn read<'a>(&'a mut self) -> Result<Self::Request<'a>, Self::ReadError> {
        let stream = Stream::new(&mut self.buffer, &mut self.socket);

        HTTPRequest::parse(stream, self.max_body_size)
    }

    fn send(&mut self, response: Self::Response) -> Result<(), Self::SendError> {
        response.send(&mut self.socket)
    }
}
