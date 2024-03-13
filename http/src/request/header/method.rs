use crate::HTTPParseError;
use std::str::FromStr;

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

impl FromStr for HTTPMethod {
    type Err = HTTPParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "GET" => HTTPMethod::GET,
            "HEAD" => HTTPMethod::HEAD,
            "POST" => HTTPMethod::POST,
            "PUT" => HTTPMethod::PUT,
            "DELETE" => HTTPMethod::DELETE,
            _ => return Err(HTTPParseError::UnknownMethod(s.to_owned())),
        })
    }
}
