use crate::{HTTPParseError, HTTPRequest, HTTPResponse};
use buffer::HeaderBuffer;
use huntsman::ProtocolClient;
use lasync::{time::Timeout, Error};
use std::{future::Future, time::Duration};

mod address;
mod buffer;
mod socket;
mod stream;

pub use address::{HTTPClientAddress, HTTPProtocol};

pub(crate) use socket::HTTPSocket;
pub(crate) use stream::Stream;

/// A client connected to the server
pub struct HTTPClient {
    /// The socket representing the underlying connection
    socket: HTTPSocket,

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
        socket: HTTPSocket,
        max_header_size: usize,
        max_body_size: usize,
        header_read_timeout: Duration,
        body_read_timeout: Duration,
        write_timeout: Duration,
    ) -> crate::Result<Self> {
        let buffer = HeaderBuffer::new(max_header_size, header_read_timeout);

        Ok(HTTPClient {
            socket,
            buffer,
            max_body_size,
            body_read_timeout,
            write_timeout,
        })
    }
}

impl ProtocolClient for HTTPClient {
    type ReadError = HTTPParseError;

    type SendError = Error;

    type Request<'a> = HTTPRequest<'a>;

    type Response<'a> = HTTPResponse<'a>;

    fn read<'a>(
        &'a mut self,
    ) -> impl Future<Output = Result<Option<Self::Request<'a>>, Self::ReadError>> {
        let stream = Stream::new(&mut self.buffer, &mut self.socket);

        HTTPRequest::parse(stream, self.max_body_size, self.body_read_timeout)
    }

    async fn send<'a>(&mut self, response: Self::Response<'a>) -> Result<(), Self::SendError> {
        let (header, body) = response.generate_header();

        let write_timeout = self.write_timeout;
        Timeout::new(
            async move {
                self.socket.write(&header).await?;

                if let Some(body) = body {
                    self.socket.write(body.body()).await
                } else {
                    Ok(())
                }
            },
            write_timeout,
        )?
        .await
        .unwrap_or(Err(Error::ETIMEDOUT))
    }
}
