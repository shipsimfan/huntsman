use huntsman::Response;
use name::SERVER;
use std::{borrow::Cow, io::Write, net::TcpStream};

mod field;
mod name;
mod status;

pub use field::HTTPResponseField;
pub use status::HTTPStatus;

/// An HTTP response to be sent to the client
pub struct HTTPResponse {
    /// The status code of this response
    status: HTTPStatus,

    /// The fields holding metadata about this request
    fields: Vec<HTTPResponseField>,

    /// The body of the response
    body: Cow<'static, [u8]>,
}

impl HTTPResponse {
    /// Creates a new [`HTTPResponse`]
    pub fn new<T: Into<Cow<'static, [u8]>>>(status: HTTPStatus, body: T) -> Self {
        let fields = vec![HTTPResponseField::new(
            "Server".as_bytes(),
            SERVER.as_bytes(),
        )];

        HTTPResponse {
            status,
            fields,
            body: body.into(),
        }
    }

    /// Gets the status for this response
    pub fn status(&self) -> HTTPStatus {
        self.status
    }

    /// Gets the fields holding metadata about this response
    pub fn fields(&self) -> &[HTTPResponseField] {
        &self.fields
    }

    /// Gets the body of this response
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Sets the status for this response
    pub fn set_status(&mut self, status: HTTPStatus) {
        self.status = status;
    }

    /// Adds a field to the end of the fields for this response
    pub fn push_field(&mut self, field: HTTPResponseField) {
        assert_ne!(field.name(), b"Content-Length", "\"Content-Length\" fields cannot be inserted into a response, this is managed by huntsman-http");

        self.fields.push(field);
    }

    /// Sets the body of this response
    pub fn set_body<T: Into<Cow<'static, [u8]>>>(&mut self, body: T) {
        self.body = body.into();
    }
}

impl Response for HTTPResponse {
    type Client = TcpStream;

    fn send(self, transport: &mut TcpStream) -> Result<(), std::io::Error> {
        transport.write_all(b"HTTP/1.1 ")?;
        transport.write_all(&self.status.code_bytes())?;
        transport.write_all(b" ")?;
        transport.write_all(self.status.message().as_bytes())?;
        transport.write_all(b"\r\nContent-Length: ")?;
        write!(transport, "{}", self.body.len())?;
        transport.write_all(b"\r\n")?;

        for field in &self.fields {
            field.write(transport)?;
        }
        transport.write_all(b"\r\n")?;

        transport.write_all(&self.body)?;

        transport.flush()
    }
}

impl From<HTTPStatus> for HTTPResponse {
    fn from(status: HTTPStatus) -> Self {
        HTTPResponse::new(status, b"" as &'static [u8])
    }
}
