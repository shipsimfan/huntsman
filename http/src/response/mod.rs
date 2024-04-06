use crate::Result;
use lasync::{io::Write, net::TCPStream, platform::errno::ETIMEDOUT, time::Timeout, Error};
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
    body: Cow<'a, [u8]>,
}

impl<'a> HTTPResponse<'a> {
    /// Creates a new [`HTTPResponse`]
    pub fn new<T: Into<Cow<'a, [u8]>>>(status: HTTPStatus, body: T) -> Self {
        let mut header = status.generate();

        header.extend_from_slice(SERVER.as_bytes());

        HTTPResponse {
            header,
            body: body.into(),
        }
    }

    /// Gets the body of this response
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Adds a field to the end of the fields for this response
    pub fn push_field(&mut self, name: &[u8], content: &[u8]) {
        assert_ne!(name, b"Content-Length", "\"Content-Length\" fields cannot be inserted into a response, this is managed by huntsman-http");

        self.header.extend_from_slice(name);
        self.header.extend_from_slice(b": ");
        self.header.extend_from_slice(content);
        self.header.extend_from_slice(b"\r\n");
    }

    /// Sets the body of this response
    pub fn set_body<T: Into<Cow<'static, [u8]>>>(&mut self, body: T) {
        self.body = body.into();
    }

    /// Writes this response to `socket`
    pub(super) async fn send(
        mut self,
        socket: &mut TCPStream,
        write_timeout: Duration,
    ) -> Result<()> {
        self.header.extend_from_slice(b"Content-Length: ");
        self.header
            .extend_from_slice(format!("{}", self.body.len()).as_bytes());
        self.header.extend_from_slice(b"\r\n\r\n");

        Timeout::new(
            async move {
                socket.write_all(&self.header).await?;
                socket.write_all(&self.body).await
            },
            write_timeout,
        )?
        .await
        .unwrap_or(Err(Error::new(ETIMEDOUT)))
    }
}

impl<'a> From<HTTPStatus> for HTTPResponse<'a> {
    fn from(status: HTTPStatus) -> Self {
        HTTPResponse::new(status, b"" as &'static [u8])
    }
}
