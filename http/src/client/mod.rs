use crate::{
    HTTPChunkedResponseBody, HTTPParseError, HTTPRequest, HTTPResponse, HTTPResponseBodyContent,
};
use buffer::HeaderBuffer;
use huntsman::ProtocolClient;
use lasync::{time::Timeout, Error};
use std::{future::Future, marker::PhantomData, time::Duration};

mod address;
mod buffer;
mod socket;
mod stream;

pub use address::{HTTPClientAddress, HTTPProtocol};

pub(crate) use socket::HTTPSocket;
pub(crate) use stream::Stream;

/// A client connected to the server
pub struct HTTPClient<B: HTTPChunkedResponseBody> {
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

    /// The type used for chunked response bodies
    _chunked_response_body: PhantomData<B>,
}

impl<B: HTTPChunkedResponseBody> HTTPClient<B> {
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
            _chunked_response_body: PhantomData,
        })
    }
}

const HEX_STR_LEN: usize = std::mem::size_of::<usize>() * 2;

fn gen_hex_str(value: usize) -> ([u8; HEX_STR_LEN], usize) {
    let mut str = [0; HEX_STR_LEN];
    let mut start = None;
    for i in 0..HEX_STR_LEN {
        let shift = (HEX_STR_LEN - i - 1) as u32 * 4;
        let val = (value.wrapping_shr(shift) & 0xF) as u8;

        if val > 9 {
            str[i] = val + b'A' - 10;
        } else {
            str[i] = val + b'0';
        }

        if start.is_none() && val != 0 {
            start = Some(i);
        }
    }

    (str, start.unwrap_or(HEX_STR_LEN - 1))
}

async fn send_chunk(socket: &mut HTTPSocket, chunk: &[u8]) -> Result<(), Error> {
    let (len, len_idx) = gen_hex_str(chunk.len());
    socket.write(&len[len_idx..]).await?;
    socket.write(b"\r\n").await?;
    socket.write(chunk).await?;
    socket.write(b"\r\n").await
}

async fn send_chunked<B: HTTPChunkedResponseBody>(
    socket: &mut HTTPSocket,
    mut body: B,
    write_timeout: Duration,
) -> Result<(), Error> {
    loop {
        let socket = &mut *socket;
        let body = &mut body;
        if !Timeout::new(
            async move {
                match body.next().await? {
                    Some(chunk) => {
                        if chunk.len() == 0 {
                            Ok(false)
                        } else {
                            send_chunk(socket, chunk).await?;
                            Ok(true)
                        }
                    }
                    None => Ok(false),
                }
            },
            write_timeout,
        )?
        .await
        .unwrap_or(Err(Error::ETIMEDOUT))?
        {
            break;
        }
    }

    Timeout::new(send_chunk(socket, &[]), write_timeout)?
        .await
        .unwrap_or(Err(Error::ETIMEDOUT))
}

impl<B: HTTPChunkedResponseBody> ProtocolClient for HTTPClient<B> {
    type ReadError = HTTPParseError;

    type SendError = Error;

    type Request<'a> = HTTPRequest<'a>;

    type Response<'a> = HTTPResponse<'a, B>;

    fn read<'a>(
        &'a mut self,
    ) -> impl Future<Output = Result<Option<Self::Request<'a>>, Self::ReadError>> {
        let stream = Stream::new(&mut self.buffer, &mut self.socket);

        HTTPRequest::parse(stream, self.max_body_size, self.body_read_timeout)
    }

    async fn send<'a>(&mut self, response: Self::Response<'a>) -> Result<(), Self::SendError> {
        let (header, body) = response.generate_header();

        let write_timeout = self.write_timeout;
        let socket = &mut self.socket;
        Timeout::new(async move { socket.write(&header).await }, write_timeout)?
            .await
            .unwrap_or(Err(Error::ETIMEDOUT))?;

        match body {
            Some(HTTPResponseBodyContent::Slice(body)) => {
                Timeout::new(async move { self.socket.write(&body).await }, write_timeout)?
                    .await
                    .unwrap_or(Err(Error::ETIMEDOUT))
            }
            Some(HTTPResponseBodyContent::Chunked(body)) => {
                send_chunked(&mut self.socket, body, write_timeout).await
            }

            None => Ok(()),
        }
    }
}

unsafe impl<B: HTTPChunkedResponseBody> Send for HTTPClient<B> {}
unsafe impl<B: HTTPChunkedResponseBody> Sync for HTTPClient<B> {}
