use huntsman::Response;
use std::net::TcpStream;

/// An HTTP response to be sent to the client
pub struct HTTPResponse {}

impl HTTPResponse {
    /// Creates a new [`HTTPResponse`]
    ///
    /// TODO: Replace this with actual creation functions (from_status, from_body, etc.)
    pub fn new() -> Self {
        HTTPResponse {}
    }
}

impl Response for HTTPResponse {
    type TransportClient = TcpStream;

    fn send(self, transport: &mut TcpStream) -> Result<(), std::io::Error> {
        // TODO: Send the response
        Ok(())
    }
}
