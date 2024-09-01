use crate::Error;
use std::future::Future;

mod empty;
mod read;

pub use empty::EmptyHTTPChunkedResponseBody;
pub use read::ReadHTTPChunkedResponseBody;

/// A body which is returned in chunks instead of one full block
pub trait HTTPChunkedResponseBody: 'static {
    /// Get the next chunk of the response
    fn next(&mut self) -> impl Future<Output = Result<Option<&[u8]>, Error>>;
}
