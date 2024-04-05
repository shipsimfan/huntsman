use crate::{HTTPParseError, HTTPRequest, HTTPResponse};
use buffer::HeaderBuffer;
use huntsman::ProtocolClient;
use lasync::net::TCPStream;
use std::{future::Future, time::Duration};

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

    /// The maximum amount of time allowed between body reads
    body_read_timeout: Duration,

    /// The maximum amount of time allowed between writes
    write_timeout: Duration,
}

impl HTTPClient {
    /// Creates a new [`HTTPClient`]
    pub(crate) fn new(
        socket: TCPStream,
        max_header_size: usize,
        max_body_size: usize,
        header_read_timeout: Duration,
        body_read_timeout: Duration,
        write_timeout: Duration,
    ) -> Self {
        let buffer = HeaderBuffer::new(max_header_size, header_read_timeout);

        HTTPClient {
            socket,
            buffer,
            max_body_size,
            body_read_timeout,
            write_timeout,
        }
    }
}

impl ProtocolClient for HTTPClient {
    type ReadError = HTTPParseError;

    type SendError = lasync::Error;

    type Request<'a> = HTTPRequest<'a>;

    type Response = HTTPResponse;

    fn read<'a>(&'a mut self) -> impl Future<Output = Result<Self::Request<'a>, Self::ReadError>> {
        let stream = Stream::new(&mut self.buffer, &mut self.socket);

        HTTPRequest::parse(stream, self.max_body_size, self.body_read_timeout)
    }

    fn send(
        &mut self,
        response: Self::Response,
    ) -> impl Future<Output = Result<(), Self::SendError>> {
        response.send(&mut self.socket, self.write_timeout)
    }
}
