use huntsman::Response;
use std::net::TcpStream;

/// An HTTP response to be sent to the client
pub struct HTTPResponse {}

impl Response for HTTPResponse {
    type TransportClient = TcpStream;

    fn send(self, transport: &mut TcpStream) -> Result<(), std::io::Error> {
        todo!("HTTPResponse::send()")
    }
}
