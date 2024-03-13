use huntsman::Response;
use std::net::TcpStream;

/// An HTTP response to be sent to the client
pub struct HTTPResponse {}

impl Response for HTTPResponse {
    type TransportClient = TcpStream;

    fn send(
        self,
        transport: &mut Self::TransportClient,
    ) -> Result<(), <Self::TransportClient as huntsman::TransportClient>::Error> {
        todo!("HTTPResponse::send()")
    }
}
