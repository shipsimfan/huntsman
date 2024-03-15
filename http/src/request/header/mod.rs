use crate::HTTPParseError;

mod buffer;
mod method;
mod stream;
mod target;

pub use method::HTTPMethod;
pub use target::HTTPTarget;

pub(super) use buffer::HTTPHeaderBuffer;
pub(super) use stream::Stream;

/// The header of an HTTP request
#[derive(Debug)]
pub struct HTTPRequestHeader<'a> {
    /// The request method
    method: HTTPMethod,

    /// The target for the request
    target: HTTPTarget<'a>,
}

impl<'a> HTTPRequestHeader<'a> {
    /// Attempts to parse an [`HTTPRequestHeader`] from `stream`
    pub(super) fn parse(stream: &mut Stream<'a, '_>) -> Result<Self, HTTPParseError> {
        let method = HTTPMethod::parse(stream)?;

        let target = HTTPTarget::parse(stream)?;

        let version = stream.collect_until_newline()?;
        if &version[..version.len() - 2] != b"HTTP/1.1" {
            return Err(HTTPParseError::InvalidVersion);
        }

        loop {
            if stream.peek()? == b'\r' {
                let end = stream.collect_until_newline()?;
                if end != b"\r\n" {
                    return Err(HTTPParseError::InvalidField);
                }

                break;
            }

            // TODO: Parse fields
            stream.collect_until_newline()?;
        }

        Ok(HTTPRequestHeader { method, target })
    }
}
