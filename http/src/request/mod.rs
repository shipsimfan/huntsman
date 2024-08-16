use crate::Stream;
use lasync::time::Timeout;
use std::{ops::Deref, time::Duration};

mod display;
mod error;
mod header;

pub use display::HTTPRequestDisplay;
pub use error::HTTPParseError;
pub use header::{HTTPMethod, HTTPRequestField, HTTPRequestHeader, HTTPTarget};

/// An HTTP request received from a client
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HTTPRequest<'a> {
    /// The header for the http request
    header: HTTPRequestHeader<'a>,

    /// The body of the request
    body: Option<Box<[u8]>>,
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
        body_read_timeout: Duration,
    ) -> Result<Option<Self>, HTTPParseError> {
        let header = match HTTPRequestHeader::parse(&mut stream).await {
            Ok(header) => header,
            Err(error) => {
                return if stream.len() == 0 {
                    Ok(None)
                } else {
                    Err(error)
                };
            }
        };

        let body = if let Some(content_length) = header.field(b"Content-Length") {
            let content_length = parse_content_length(content_length.value())?;

            if content_length > max_body_size {
                return Err(HTTPParseError::BodyTooLarge);
            }

            let (stream, mut buffer, current_length) = stream.body(content_length);

            if current_length < buffer.len() {
                Timeout::new(
                    stream.read_exact(&mut buffer[current_length..]),
                    body_read_timeout,
                )?
                .await
                .map(|result| result.map_err(Into::into))
                .unwrap_or(Err(HTTPParseError::BodyReadTimeout))?;
            }

            Some(buffer)
        } else {
            None
        };

        Ok(Some(HTTPRequest { header, body }))
    }

    /// Gets the body of this request
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_ref().map(|slice| slice.as_ref())
    }
}

impl<'a> Deref for HTTPRequest<'a> {
    type Target = HTTPRequestHeader<'a>;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}
