// rustdoc imports
#[allow(unused_imports)]
use super::HTTPResponse;

mod chunked;
mod content;

pub use chunked::{
    EmptyHTTPChunkedResponseBody, HTTPChunkedResponseBody, ReadHTTPChunkedResponseBody,
};
pub use content::HTTPResponseBodyContent;

/// The body of an [`HTTPResponse`]
pub(crate) struct HTTPResponseBody<'a, B: HTTPChunkedResponseBody> {
    /// The content of the body
    content: HTTPResponseBodyContent<'a, B>,

    /// The encoding type of the content
    content_type: &'static [u8],
}

impl<'a, B: HTTPChunkedResponseBody> HTTPResponseBody<'a, B> {
    /// Creates a new [`HTTPResponseBody`]
    pub(super) fn new<T: Into<HTTPResponseBodyContent<'a, B>>>(
        body: T,
        content_type: &'static [u8],
    ) -> Self {
        assert!(content_type.len() > 0, "\"content_type\" cannot be empty");

        let body = body.into();

        HTTPResponseBody {
            content: body,
            content_type,
        }
    }

    /// Gets the contained body
    pub(crate) fn content(&self) -> &HTTPResponseBodyContent<'a, B> {
        &self.content
    }

    /// Gets the type of the body
    pub(super) fn content_type(&self) -> &[u8] {
        &self.content_type
    }

    /// Unwraps the content of this body
    pub(super) fn unwrap(self) -> HTTPResponseBodyContent<'a, B> {
        self.content
    }
}
