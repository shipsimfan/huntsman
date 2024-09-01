use crate::HTTPChunkedResponseBody;

/// A implementation of chunked response bodies that does nothing. This is used as the default and
/// is designed for apps which don't chunk response bodies.
pub struct EmptyHTTPChunkedResponseBody;

impl HTTPChunkedResponseBody for EmptyHTTPChunkedResponseBody {
    async fn next(&mut self) -> Result<Option<&[u8]>, lasync::Error> {
        Ok(None)
    }
}
