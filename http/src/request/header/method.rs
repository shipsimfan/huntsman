use super::Stream;
use crate::HTTPParseError;

/// A method for the request
#[derive(Debug)]
pub enum HTTPMethod {
    /// The [`HTTPMethod::GET`] method requests transfer of a current selected representation for
    /// the target resource
    GET,

    /// The [`HTTPMethod::HEAD`] method is identical to [`HTTPMethod::GET`] except that the server
    /// MUST NOT send content in the response
    HEAD,

    /// The [`HTTPMethod::POST`] method requests that the target resource process the
    /// representation enclosed in the request according to the resource's own specific semantics
    POST,

    /// The [`HTTPMethod::PUT`] method requests that the state of the target resource be created or
    /// replaced with the state defined by the representation enclosed in the request message
    /// content
    PUT,

    /// The [`HTTPMethod::DELETE`] method requests that the origin server remove the association
    /// between the target resource and its current functionality
    DELETE,
}

impl HTTPMethod {
    /// Attempts to parse an [`HTTPMethod`] from `stream`
    pub(super) fn parse(stream: &mut Stream) -> Result<Self, HTTPParseError> {
        let method = stream.collect_until(b' ')?;

        Ok(match &method[..method.len() - 1] {
            b"GET" => HTTPMethod::GET,
            b"HEAD" => HTTPMethod::HEAD,
            b"POST" => HTTPMethod::POST,
            b"PUT" => HTTPMethod::PUT,
            b"DELETE" => HTTPMethod::DELETE,
            _ => return Err(HTTPParseError::InvalidMethod),
        })
    }
}
