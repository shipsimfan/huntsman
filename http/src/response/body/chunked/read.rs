use crate::HTTPChunkedResponseBody;
use lasync::io::Read;
use std::num::NonZeroUsize;

/// Transfers data from a type which implements [`Read`] in maximum sized chunks.
pub struct ReadHTTPChunkedResponseBody<R: 'static + Read> {
    buffer: Vec<u8>,
    reader: R,
}

impl<R: Read> ReadHTTPChunkedResponseBody<R> {
    /// Creates a new [`ReadHTTPChunkedResponseBody`] from `reader` send chunks at most
    /// `max_chunk_size` bytes long
    pub fn new(reader: R, max_chunk_size: NonZeroUsize) -> Self {
        unsafe { ReadHTTPChunkedResponseBody::new_unchecked(reader, max_chunk_size.get()) }
    }
    /// Creates a new [`ReadHTTPChunkedResponseBody`] from `reader` send chunks at most
    /// `max_chunk_size` bytes long without checking the length is greater than 0
    pub unsafe fn new_unchecked(reader: R, max_chunk_size: usize) -> Self {
        let mut buffer = Vec::with_capacity(max_chunk_size);
        buffer.set_len(max_chunk_size);

        ReadHTTPChunkedResponseBody { buffer, reader }
    }
}

impl<R: Read> HTTPChunkedResponseBody for ReadHTTPChunkedResponseBody<R> {
    async fn next(&mut self) -> Result<Option<&[u8]>, lasync::Error> {
        let count = self.reader.read(&mut self.buffer).await?;
        if count == 0 {
            Ok(None)
        } else {
            Ok(Some(&self.buffer[..count]))
        }
    }
}
