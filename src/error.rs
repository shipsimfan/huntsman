/// An error during the runtime of the server
pub enum StartError<Protocol: crate::Protocol> {
    /// The error occurred while starting the asynchronous runtime
    Async(lasync::executor::Error),

    /// The error occurred while the protocol was starting the listen sockets
    Protocol(Protocol::ListenError),
}

impl<Protocol: crate::Protocol> std::error::Error for StartError<Protocol> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StartError::Async(error) => Some(error),
            StartError::Protocol(error) => Some(error),
        }
    }
}

impl<Protocol: crate::Protocol> std::fmt::Display for StartError<Protocol> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::error::Error;

        write!(f, "unable to start server - {}", self.source().unwrap())
    }
}

impl<Protocol: crate::Protocol> std::fmt::Debug for StartError<Protocol> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<Protocol: crate::Protocol> From<lasync::executor::Error> for StartError<Protocol> {
    fn from(error: lasync::executor::Error) -> Self {
        StartError::Async(error)
    }
}
