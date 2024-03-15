use super::Stream;
use crate::HTTPParseError;

mod method;

pub use method::HTTPMethod;

/// The header of an HTTP request
pub struct HTTPRequestHeader {
    /// The request method
    method: HTTPMethod,
}

impl HTTPRequestHeader {
    /// Attempts to parse an [`HTTPRequestHeader`] from the `stream`
    pub(super) fn parse(stream: &mut Stream) -> Result<Self, HTTPParseError> {
        let method = HTTPMethod::parse(stream)?;

        Ok(HTTPRequestHeader { method })
    }
}
