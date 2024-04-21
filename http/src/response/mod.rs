use crate::Result;
use lasync::{io::Write, net::TCPStream, time::Timeout, Error};
use name::SERVER;
use std::{borrow::Cow, time::Duration};

mod name;
mod status;

pub use status::HTTPStatus;

/// An HTTP response to be sent to the client
pub struct HTTPResponse<'a> {
    /// The header of the response
    header: Vec<u8>,

    /// The body of the response
    body: Option<Cow<'a, [u8]>>,
}

impl<'a> HTTPResponse<'a> {
    /// Creates a new [`HTTPResponse`] with a body
    pub fn new<T: Into<Cow<'a, [u8]>>>(status: HTTPStatus, body: T) -> Self {
        let mut response = Self::new_status(status);
        response.set_body(body);
        response
    }

    /// Creates a new [`HTTPResponse`] without a body
    pub fn new_status(status: HTTPStatus) -> Self {
        let mut header = status.generate();
        header.extend_from_slice(SERVER.as_bytes());

        HTTPResponse { header, body: None }
    }

    /// Gets the body of this response
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_ref().map(AsRef::as_ref)
    }

    /// Adds a field to the end of the fields for this response
    pub fn push_field(&mut self, name: &[u8], content: &[u8]) {
        assert_ne!(name, b"Content-Length", "\"Content-Length\" fields cannot be inserted into a response, this is managed by huntsman-http");
        assert_ne!(name, b"Server", "\"Server\" fields cannot be inserted into a response, this is managed by huntsman-http");

        self.header.extend_from_slice(name);
        self.header.extend_from_slice(b": ");
        self.header.extend_from_slice(content);
        self.header.extend_from_slice(b"\r\n");
    }

    /// Sets the body of this response
    pub fn set_body<T: Into<Cow<'a, [u8]>>>(&mut self, body: T) {
        let body = body.into();
        if body.len() == 0 {
            return;
        }

        self.body = Some(body);
    }

    /// Writes this response to `socket`
    pub(super) async fn send(
        mut self,
        socket: &mut TCPStream,
        write_timeout: Duration,
    ) -> Result<()> {
        if let Some(body_len) = self.body.as_ref().map(|body| body.len()) {
            self.header.extend_from_slice(b"Content-Length: ");
            self.header
                .extend_from_slice(format!("{}", body_len).as_bytes());
        }

        self.header.extend_from_slice(b"\r\n\r\n");

        Timeout::new(
            async move {
                socket.write_all(&self.header).await?;

                if let Some(body) = self.body.as_ref() {
                    socket.write_all(body).await
                } else {
                    Ok(())
                }
            },
            write_timeout,
        )?
        .await
        .unwrap_or(Err(Error::ETIMEDOUT))
    }
}

impl<'a> From<HTTPStatus> for HTTPResponse<'a> {
    fn from(status: HTTPStatus) -> Self {
        HTTPResponse::new_status(status)
    }
}

impl<'a, T: Into<Cow<'a, [u8]>>> From<(HTTPStatus, T)> for HTTPResponse<'a> {
    fn from(value: (HTTPStatus, T)) -> Self {
        HTTPResponse::new(value.0, value.1)
    }
}
