use huntsman::Response;
use std::{borrow::Cow, io::Write, net::TcpStream};

mod status;

pub use status::HTTPStatus;

/// An HTTP response to be sent to the client
pub struct HTTPResponse {
    /// The status code of this response
    status: HTTPStatus,

    /// The body of the response
    body: Cow<'static, [u8]>,
}

impl HTTPResponse {
    /// Creates a new [`HTTPResponse`]
    pub fn new<T: Into<Cow<'static, [u8]>>>(status: HTTPStatus, body: T) -> Self {
        HTTPResponse {
            status,
            body: body.into(),
        }
    }

    /// Gets the status for this response
    pub fn status(&self) -> HTTPStatus {
        self.status
    }

    /// Gets the body of this response
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Sets the status for this response
    pub fn set_status(&mut self, status: HTTPStatus) {
        self.status = status;
    }

    /// Sets the body of this response
    pub fn set_body<T: Into<Cow<'static, [u8]>>>(&mut self, body: T) {
        self.body = body.into();
    }
}

impl Response for HTTPResponse {
    type TransportClient = TcpStream;

    fn send(self, transport: &mut TcpStream) -> Result<(), std::io::Error> {
        transport.write_all(b"HTTP/1.1 ")?;
        transport.write_all(&self.status.code_bytes())?;
        transport.write_all(b" ")?;
        transport.write_all(self.status.message().as_bytes())?;
        transport.write_all(b"\r\nContent-Length: ")?;
        write!(transport, "{}", self.body.len())?;
        transport.write_all(b"\r\n\r\n")?;

        transport.write_all(&self.body)?;

        transport.flush()
    }
}

impl From<HTTPStatus> for HTTPResponse {
    fn from(status: HTTPStatus) -> Self {
        HTTPResponse::new(status, b"" as &'static [u8])
    }
}
