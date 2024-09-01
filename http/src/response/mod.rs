use body::HTTPResponseBody;
use name::SERVER;

mod body;
mod name;
mod status;

pub use body::{
    EmptyHTTPChunkedResponseBody, HTTPChunkedResponseBody, HTTPResponseBodyContent,
    ReadHTTPChunkedResponseBody,
};
pub use status::HTTPStatus;

/// An HTTP response to be sent to the client
pub struct HTTPResponse<'a, B: HTTPChunkedResponseBody = EmptyHTTPChunkedResponseBody> {
    /// The header of the response
    header: Vec<u8>,

    /// The body of the response
    body: Option<HTTPResponseBody<'a, B>>,

    /// The status this response was created with
    status: HTTPStatus,
}

impl<'a, B: HTTPChunkedResponseBody> HTTPResponse<'a, B> {
    /// Creates a new [`HTTPResponse`] with a body
    pub fn new<T: Into<HTTPResponseBodyContent<'a, B>>>(
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
    pub fn set_body<T: Into<HTTPResponseBodyContent<'a, B>>>(
        &mut self,
        body: T,
        content_type: &'static [u8],
    ) {
        self.body = Some(HTTPResponseBody::new(body.into(), content_type));
    }

    /// Generates the response header to write and returns the tuple `(header, body)`
    pub(super) fn generate_header(mut self) -> (Vec<u8>, Option<HTTPResponseBodyContent<'a, B>>) {
        let body = match self.body {
            Some(body) => body,
            None => {
                self.header.extend_from_slice(b"Content-Length: 0\r\n\r\n");
                return (self.header, None);
            }
        };

        match body.content() {
            HTTPResponseBodyContent::Chunked(_) => {
                self.header.extend_from_slice(b"Transfer-Encoding: chunked")
            }
            HTTPResponseBodyContent::Slice(slice) => {
                self.header.extend_from_slice(b"Content-Length: ");
                self.header
                    .extend_from_slice(slice.len().to_string().as_bytes());
            }
        }

        self.header.extend_from_slice(b"\r\nContent-Type: ");
        self.header.extend_from_slice(body.content_type());
        self.header.extend_from_slice(b"\r\n\r\n");

        (self.header, Some(body.unwrap()))
    }
}

impl<'a, B: HTTPChunkedResponseBody> From<HTTPStatus> for HTTPResponse<'a, B> {
    fn from(status: HTTPStatus) -> Self {
        HTTPResponse::new_status(status)
    }
}

impl<'a, B: HTTPChunkedResponseBody, T: Into<HTTPResponseBodyContent<'a, B>>>
    From<(HTTPStatus, T, &'static [u8])> for HTTPResponse<'a, B>
{
    fn from(value: (HTTPStatus, T, &'static [u8])) -> Self {
        HTTPResponse::new(value.0, value.1, value.2)
    }
}

impl<'a, B: HTTPChunkedResponseBody, T: Into<HTTPResponseBodyContent<'a, B>>, const N: usize>
    From<(HTTPStatus, T, &'static [u8; N])> for HTTPResponse<'a, B>
{
    fn from(value: (HTTPStatus, T, &'static [u8; N])) -> Self {
        HTTPResponse::new(value.0, value.1, value.2)
    }
}
