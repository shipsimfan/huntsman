/// A type representing a potential error
pub type Result<T> = core::result::Result<T, Error>;

/// An error during the runtime of the server
pub enum Error {
    /// An I/O error
    IO(lasync::executor::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IO(error) => Some(error),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => error.fmt(f),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::error::Error::source(self) {
            Some(error) => std::fmt::Debug::fmt(error, f),
            None => std::fmt::Display::fmt(self, f),
        }
    }
}

impl From<lasync::executor::Error> for Error {
    fn from(error: lasync::executor::Error) -> Self {
        Error::IO(error)
    }
}
