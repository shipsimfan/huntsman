use crate::Transport;
use std::net::SocketAddr;

/// A type representing a potential error
pub type Result<T, Protocol> = core::result::Result<T, Error<Protocol>>;

/// An error during the runtime of the server
pub enum Error<Protocol: crate::Protocol> {
    /// Binding the listen socket failed
    ListenBindFailed((<Protocol::Transport as Transport>::Error, SocketAddr)),
}

impl<Protocol: crate::Protocol> std::error::Error for Error<Protocol> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ListenBindFailed((error, _)) => Some(error),
        }
    }
}

impl<Protocol: crate::Protocol> std::fmt::Display for Error<Protocol> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ListenBindFailed((error, address)) => {
                write!(f, "Unable to begin listening on {} - {}", address, error)
            }
        }
    }
}

impl<Protocol: crate::Protocol> std::fmt::Debug for Error<Protocol> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
