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
}
