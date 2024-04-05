/// An error during the runtime of the server
pub enum StartError<Protocol: crate::Protocol> {
    /// The error occurred while starting the asynchronous runtime
    Async(lasync::Error),

    /// The error occurred while the protocol was starting the listen sockets
    Protocol(Protocol::ListenError),

    /// The error occurred while a worker was being spawned
    Worker(std::io::Error),
}

impl<Protocol: crate::Protocol> std::error::Error for StartError<Protocol> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StartError::Async(error) => Some(error),
            StartError::Protocol(error) => Some(error),
            StartError::Worker(error) => Some(error),
        }
    }
}

impl<Protocol: crate::Protocol> std::fmt::Display for StartError<Protocol> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartError::Async(error) => write!(f, "unable to start the runtime - {}", error),
            StartError::Protocol(error) => write!(f, "unable to start the server - {}", error),
            StartError::Worker(error) => write!(f, "unable to spawn a worker - {}", error),
        }
    }
}

impl<Protocol: crate::Protocol> std::fmt::Debug for StartError<Protocol> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<Protocol: crate::Protocol> From<lasync::Error> for StartError<Protocol> {
    fn from(error: lasync::Error) -> Self {
        StartError::Async(error)
    }
}

impl<Protocol: crate::Protocol> From<std::io::Error> for StartError<Protocol> {
    fn from(error: std::io::Error) -> Self {
        StartError::Worker(error)
    }
}
