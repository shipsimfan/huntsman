/// An error during the runtime of the server
pub struct StartError<Protocol: crate::Protocol>(pub(crate) Protocol::ListenError);

impl<Protocol: crate::Protocol> std::error::Error for StartError<Protocol> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl<Protocol: crate::Protocol> std::fmt::Display for StartError<Protocol> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to start server - {}", self.0)
    }
}

impl<Protocol: crate::Protocol> std::fmt::Debug for StartError<Protocol> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
