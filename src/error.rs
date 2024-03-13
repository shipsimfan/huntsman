use crate::{Protocol, Transport};
use std::net::SocketAddr;

/// A type representing a potential error
pub type Result<T, App> = core::result::Result<T, Error<App>>;

/// An error during the runtime of the server
pub enum Error<App: crate::App> {
    /// Binding the listen socket failed
    ListenBindFailed(
        (
            <<App::Protocol as Protocol>::Transport as Transport>::Error,
            SocketAddr,
        ),
    ),
}

impl<App: crate::App> std::error::Error for Error<App> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ListenBindFailed((error, _)) => Some(error),
        }
    }
}

impl<App: crate::App> std::fmt::Display for Error<App> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ListenBindFailed((error, address)) => {
                write!(f, "Unable to begin listening on {} - {}", address, error)
            }
        }
    }
}

impl<App: crate::App> std::fmt::Debug for Error<App> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
