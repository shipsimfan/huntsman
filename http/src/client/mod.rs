use crate::{HTTPParseError, HTTPRequest, HTTPResponse};
use buffer::HeaderBuffer;
use huntsman::ProtocolClient;
use lasync::futures::net::TCPStream;
use std::future::Future;

mod buffer;
mod stream;

pub(crate) use stream::Stream;

/// A client connected to the server
pub struct HTTPClient {
    /// The socket representing the underlying connection
    socket: TCPStream,

    /// The buffer for more efficient header reading and parsing
    buffer: HeaderBuffer,

    /// The maximum size for request bodies
    max_body_size: usize,
}

impl HTTPClient {
    /// Creates a new [`HTTPClient`]
    pub(crate) fn new(socket: TCPStream, max_header_size: usize, max_body_size: usize) -> Self {
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

    type SendError = lasync::executor::Error;

    type Request<'a> = HTTPRequest<'a>;

    type Response = HTTPResponse;

    fn read<'a>(&'a mut self) -> impl Future<Output = Result<Self::Request<'a>, Self::ReadError>> {
        async { todo!() }
    }

    fn send(
        &mut self,
        response: Self::Response,
    ) -> impl Future<Output = Result<(), Self::SendError>> {
        async { todo!() }
    }
}
