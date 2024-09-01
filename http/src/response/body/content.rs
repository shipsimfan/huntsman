use crate::HTTPChunkedResponseBody;
use std::borrow::Cow;

/// The content of an HTTP response body
pub enum HTTPResponseBodyContent<'a, B: HTTPChunkedResponseBody> {
    /// The content is in one block
    Slice(Cow<'a, [u8]>),

    /// The content is chunked across multiple blocks
    Chunked(B),
}

impl<'a, B: HTTPChunkedResponseBody> From<&'a [u8]> for HTTPResponseBodyContent<'a, B> {
    fn from(value: &'a [u8]) -> Self {
        HTTPResponseBodyContent::Slice(Cow::Borrowed(value))
    }
}

impl<'a, B: HTTPChunkedResponseBody> From<Cow<'a, [u8]>> for HTTPResponseBodyContent<'a, B> {
    fn from(value: Cow<'a, [u8]>) -> Self {
        HTTPResponseBodyContent::Slice(value)
    }
}

impl<'a, B: HTTPChunkedResponseBody> From<Vec<u8>> for HTTPResponseBodyContent<'a, B> {
    fn from(value: Vec<u8>) -> Self {
        HTTPResponseBodyContent::Slice(Cow::Owned(value))
    }
}

impl<'a, B: HTTPChunkedResponseBody> From<B> for HTTPResponseBodyContent<'a, B> {
    fn from(body: B) -> Self {
        HTTPResponseBodyContent::Chunked(body)
    }
}
