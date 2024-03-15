use crate::{request::Stream, HTTPParseError};

/// A method for the request
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

/// The length of the longest method ("DELETE")
const LONGEST_METHOD: usize = b"DELETE".len();

impl HTTPMethod {
    /// Attempts to parse an [`HTTPMethod`] from the `stream`
    pub(super) fn parse(stream: &mut Stream) -> Result<Self, HTTPParseError> {
        let mut buffer = [0; LONGEST_METHOD];
        let length = stream.collect_until(b' ', &mut buffer)?;

        Ok(match &buffer[..length] {
            b"GET" => HTTPMethod::GET,
            b"HEAD" => HTTPMethod::HEAD,
            b"POST" => HTTPMethod::POST,
            b"PUT" => HTTPMethod::PUT,
            b"DELETE" => HTTPMethod::DELETE,
            _ => return Err(HTTPParseError::InvalidMethod),
        })
    }
}
