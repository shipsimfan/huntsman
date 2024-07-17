use body::HTTPResponseBody;
use name::SERVER;
use std::borrow::Cow;

mod body;
mod name;
mod status;

pub use status::HTTPStatus;

/// An HTTP response to be sent to the client
#[derive(Debug)]
pub struct HTTPResponse<'a> {
    /// The header of the response
    header: Vec<u8>,

    /// The body of the response
    body: Option<HTTPResponseBody<'a>>,

    /// The status this response was created with
    status: HTTPStatus,
}

impl<'a> HTTPResponse<'a> {
    /// Creates a new [`HTTPResponse`] with a body
    pub fn new<T: Into<Cow<'a, [u8]>>>(
        status: HTTPStatus,
        body: T,
        content_type: &'static [u8],
    ) -> Self {
        let mut response = Self::new_status(status);
        response.set_body(body, content_type);
        response
    }

    /// Creates a new [`HTTPResponse`] without a body
    pub fn new_status(status: HTTPStatus) -> Self {
        let mut header = status.generate();
        header.extend_from_slice(SERVER.as_bytes());

        HTTPResponse {
            header,
            body: None,
            status,
        }
    }

    /// Gets the body of this response
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_ref().map(HTTPResponseBody::body)
    }

    /// Gets the type of this reponse's body's content
    pub fn content_type(&self) -> Option<&[u8]> {
        self.body.as_ref().map(HTTPResponseBody::content_type)
    }

    /// Gets the status of this response
    pub fn status(&self) -> HTTPStatus {
        self.status
    }

    /// Adds a field to the end of the fields for this response
    pub fn push_field(&mut self, name: &[u8], content: &[u8]) {
        assert_ne!(name, b"Content-Length", "\"Content-Length\" fields cannot be inserted into a response, this is managed by huntsman-http");
        assert_ne!(name, b"Content-Type", "\"Content-Type\" fields cannot be inserted into a response, this is managed by huntsman-http");
        assert_ne!(name, b"Server", "\"Server\" fields cannot be inserted into a response, this is managed by huntsman-http");

        self.header.extend_from_slice(name);
        self.header.extend_from_slice(b": ");
        self.header.extend_from_slice(content);
        self.header.extend_from_slice(b"\r\n");
    }

    /// Sets the body of this response
    pub fn set_body<T: Into<Cow<'a, [u8]>>>(&mut self, body: T, content_type: &'static [u8]) {
        let body = body.into();
        if body.len() == 0 {
            return;
        }

        self.body = Some(HTTPResponseBody::new(body, content_type));
    }

    /// Generates the response header to write and returns the tuple `(header, body)`
    pub(super) fn generate_header(mut self) -> (Vec<u8>, Option<HTTPResponseBody<'a>>) {
        let body_length = if let Some(body) = self.body.as_ref() {
            self.header.extend_from_slice(b"Content-Type: ");
            self.header.extend_from_slice(body.content_type());
            self.header.extend_from_slice(b"\r\n");

            body.len()
        } else {
            0
        };

        self.header.extend_from_slice(b"Content-Length: ");
        self.header
            .extend_from_slice(body_length.to_string().as_bytes());
        self.header.extend_from_slice(b"\r\n\r\n");
        (self.header, self.body)
    }
}

impl<'a> From<HTTPStatus> for HTTPResponse<'a> {
    fn from(status: HTTPStatus) -> Self {
        HTTPResponse::new_status(status)
    }
}

impl<'a, T: Into<Cow<'a, [u8]>>> From<(HTTPStatus, T, &'static [u8])> for HTTPResponse<'a> {
    fn from(value: (HTTPStatus, T, &'static [u8])) -> Self {
        HTTPResponse::new(value.0, value.1, value.2)
    }
}

impl<'a, const N: usize, T: Into<Cow<'a, [u8]>>> From<(HTTPStatus, T, &'static [u8; N])>
    for HTTPResponse<'a>
{
    fn from(value: (HTTPStatus, T, &'static [u8; N])) -> Self {
        HTTPResponse::new(value.0, value.1, value.2)
    }
}
