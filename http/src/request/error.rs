/// An error while parsing an HTTP Request
pub enum HTTPParseError {
    /// An invalid method was provided
    InvalidMethod,

    /// The headers is too long to fit into the buffer
    HeadersTooLong,

    /// The client sent an incomplete header before disconnecting
    IncompleteHeader,

    /// An I/O error occurred while parsing a request
    IO(std::io::Error),
}

impl std::error::Error for HTTPParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HTTPParseError::IO(error) => Some(error),

            HTTPParseError::InvalidMethod
            | HTTPParseError::HeadersTooLong
            | HTTPParseError::IncompleteHeader => None,
        }
    }
}

impl std::fmt::Display for HTTPParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HTTPParseError::InvalidMethod => write!(f, "invalid method"),
            HTTPParseError::HeadersTooLong => write!(f, "headers too long"),
            HTTPParseError::IncompleteHeader => write!(f, "header is incomplete"),

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

impl From<std::io::Error> for HTTPParseError {
    fn from(error: std::io::Error) -> Self {
        HTTPParseError::IO(error)
    }
}
