/// An error while parsing an HTTP Request
pub enum HTTPParseError {
    /// An invalid method was provided
    InvalidMethod,

    /// The headers is too long to fit into the buffer
    HeadersTooLong,

    /// The client sent an incomplete header before disconnecting
    IncompleteHeader,

    /// The requested target contains invalid characters
    InvalidTarget,

    /// The HTTP header contains an invalid version
    InvalidVersion,

    /// The request contains an invalid field
    InvalidField,

    /// The "Content-Length" field of the request doesn't contain a number
    InvalidContentLength,

    /// The request body is too large
    BodyTooLarge,

    /// An I/O error occurred while parsing a request
    IO(lasync::executor::Error),
}

impl std::error::Error for HTTPParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HTTPParseError::IO(error) => Some(error),

            HTTPParseError::InvalidMethod
            | HTTPParseError::HeadersTooLong
            | HTTPParseError::IncompleteHeader
            | HTTPParseError::InvalidTarget
            | HTTPParseError::InvalidVersion
            | HTTPParseError::InvalidField
            | HTTPParseError::InvalidContentLength
            | HTTPParseError::BodyTooLarge => None,
        }
    }
}

impl std::fmt::Display for HTTPParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HTTPParseError::InvalidMethod => write!(f, "invalid method"),
            HTTPParseError::HeadersTooLong => write!(f, "headers too long"),
            HTTPParseError::IncompleteHeader => write!(f, "header is incomplete"),
            HTTPParseError::InvalidTarget => write!(f, "invalid target"),
            HTTPParseError::InvalidVersion => write!(f, "invalid version"),
            HTTPParseError::InvalidField => write!(f, "invalid field"),
            HTTPParseError::InvalidContentLength => write!(f, "invalid content length"),
            HTTPParseError::BodyTooLarge => write!(f, "request body too large"),

            HTTPParseError::IO(error) => write!(
                f,
                "an I/O error occurred while parsing a request - {}",
                error
            ),
        }
    }
}

impl std::fmt::Debug for HTTPParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl From<lasync::executor::Error> for HTTPParseError {
    fn from(error: lasync::executor::Error) -> Self {
        HTTPParseError::IO(error)
    }
}
