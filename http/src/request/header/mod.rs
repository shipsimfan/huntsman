use crate::HTTPParseError;

mod buffer;
mod method;
mod stream;

pub use method::HTTPMethod;

pub(super) use buffer::HTTPHeaderBuffer;
pub(super) use stream::Stream;

/// The header of an HTTP request
#[derive(Debug)]
pub struct HTTPRequestHeader {
    /// The request method
    method: HTTPMethod,
}

impl HTTPRequestHeader {
    /// Attempts to parse an [`HTTPRequestHeader`] from `stream`
    pub(super) fn parse(stream: &mut Stream) -> Result<Self, HTTPParseError> {
        let method = HTTPMethod::parse(stream)?;

        Ok(HTTPRequestHeader { method })
    }
}
