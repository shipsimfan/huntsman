use std::borrow::Cow;

// rustdoc imports
#[allow(unused_imports)]
use super::HTTPResponse;

/// The body of an [`HTTPResponse`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HTTPResponseBody<'a> {
    /// The body itself
    body: Cow<'a, [u8]>,

    /// The content type of the body
    content_type: &'static [u8],
}

impl<'a> HTTPResponseBody<'a> {
    /// Creates a new [`HTTPResponseBody`]
    pub(super) fn new<T: Into<Cow<'a, [u8]>>>(body: T, content_type: &'static [u8]) -> Self {
        let body = body.into();
        assert!(body.len() > 0, "response body cannot be empty");
        assert!(content_type.len() > 0, "\"content_type\" cannot be empty");

        HTTPResponseBody { body, content_type }
    }

    /// Gets the contained body
    pub(crate) fn body(&self) -> &[u8] {
        &self.body
    }

    /// Gets the type of the body
    pub(super) fn content_type(&self) -> &[u8] {
        &self.content_type
    }

    /// Gets the length of the contained body
    pub(super) fn len(&self) -> usize {
        self.body.len()
    }
}
