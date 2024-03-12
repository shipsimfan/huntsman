use std::net::SocketAddr;

/// A type representing a potential error
pub type Result<T> = core::result::Result<T, Error>;

/// An error during the runtime of the server
pub enum Error {
    /// Binding the listen socket failed
    ListenBindFailed((std::io::Error, SocketAddr)),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ListenBindFailed((error, _)) => Some(error),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ListenBindFailed((error, address)) => {
                write!(f, "Unable to begin listening on {} - {}", address, error)
            }
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
