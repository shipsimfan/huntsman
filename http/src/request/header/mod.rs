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

        Ok(HTTPRequestHeader { method, target })
    }
}
