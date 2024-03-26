use crate::Stream;
use lasync::futures::io::Read;
use std::ops::Deref;

mod error;
mod header;

pub use error::HTTPParseError;
pub use header::{HTTPMethod, HTTPRequestHeader};

/// An HTTP request received from a client
#[derive(Debug)]
pub struct HTTPRequest<'a> {
    /// The header for the http request
    header: HTTPRequestHeader<'a>,

    /// The body of the request
    body: Box<[u8]>,
}

/// Parse the `content_length` into a [`usize`]
fn parse_content_length(content_length: &[u8]) -> Result<usize, HTTPParseError> {
    let mut value = 0;

    for byte in content_length {
        if !byte.is_ascii_digit() {
            return Err(HTTPParseError::InvalidContentLength);
        }

        value *= 10;
        value += (*byte - b'0') as usize;
    }

    Ok(value)
}

impl<'a> HTTPRequest<'a> {
    /// Attempts to parse an [`HTTPRequest`] from `stream`
    pub(crate) async fn parse(
        mut stream: Stream<'a, '_>,
        max_body_size: usize,
    ) -> Result<Self, HTTPParseError> {
        let header = HTTPRequestHeader::parse(&mut stream).await?;

        let mut body = Vec::new().into_boxed_slice();
        if let Some(content_length) = header.field(b"Content-Length") {
            let content_length = parse_content_length(content_length.value())?;

            if content_length > max_body_size {
                return Err(HTTPParseError::BodyTooLarge);
            }

            let (stream, mut buffer, current_length) = stream.body(content_length);

            if current_length < buffer.len() {
                stream.read_exact(&mut buffer[current_length..]).await?;
            }

            body = buffer;
        }

        Ok(HTTPRequest { header, body })
    }

    /// Gets the body of this request
    pub fn body(&self) -> &[u8] {
        &self.body
    }
}

impl<'a> Deref for HTTPRequest<'a> {
    type Target = HTTPRequestHeader<'a>;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
