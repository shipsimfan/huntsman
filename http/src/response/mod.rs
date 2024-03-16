use huntsman::Response;
use std::{io::Write, net::TcpStream};

mod status;

pub use status::HTTPStatus;

/// An HTTP response to be sent to the client
pub struct HTTPResponse {
    /// The status code of this response
    status: HTTPStatus,
}

impl HTTPResponse {
    /// Creates a new [`HTTPResponse`]
    pub fn new(status: HTTPStatus) -> Self {
        HTTPResponse { status }
    }

    /// Gets the status for this response
    pub fn status(&self) -> HTTPStatus {
        self.status
    }

    /// Sets the status for this response
    pub fn set_status(&mut self, status: HTTPStatus) {
        self.status = status;
    }
}

impl Response for HTTPResponse {
    type TransportClient = TcpStream;

    fn send(self, transport: &mut TcpStream) -> Result<(), std::io::Error> {
        transport.write_all(b"HTTP/1.1 ")?;
        transport.write_all(&self.status.code_bytes())?;
        transport.write_all(b" ")?;
        transport.write_all(self.status.message().as_bytes())?;
        transport.write_all(b"\r\nContent-Length: 0\r\n\r\n")?;

        transport.flush()
    }
}

impl From<HTTPStatus> for HTTPResponse {
    fn from(status: HTTPStatus) -> Self {
        HTTPResponse::new(status)
    }
}
