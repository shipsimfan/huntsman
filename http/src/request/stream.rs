use super::Buffer;
use std::net::TcpStream;

/// A stream of bytes from a [`TcpStream`]
pub(super) struct Stream<'a> {
    /// The buffer to improve read efficiency
    buffer: &'a mut Buffer,

    /// The stream to read from
    stream: &'a mut TcpStream,
}

impl<'a> Stream<'a> {
    /// Creates a new [`Stream`]
    pub(super) fn new(buffer: &'a mut Buffer, stream: &'a mut TcpStream) -> Self {
        Stream { buffer, stream }
    }

    /// Attempts to collect bytes into `buffer` from the stream until a `c` byte is encountered
    ///
    /// This function returns the amount of bytes collected into the buffer.
    ///
    /// If the end of the buffer is reached before a `c` byte is encountered, the function will
    /// return the length of the buffer.
    ///
    /// The `buffer` will not contain the character searched for but the next byte returned by
    /// this stream will be the byte that follows the character searched for.
    pub(super) fn collect_until(
        &mut self,
        c: u8,
        buffer: &mut [u8],
    ) -> Result<usize, std::io::Error> {
        todo!("Stream::collect_until");
    }
}
