use super::HTTPHeaderBuffer;
use crate::HTTPParseError;
use std::net::TcpStream;

/// A stream of bytes from a [`TcpStream`]
pub(in crate::request) struct Stream<'a, 'b> {
    /// The buffer to improve read efficiency
    buffer: &'a mut HTTPHeaderBuffer,

    /// The stream to read from
    stream: &'b mut TcpStream,
}

impl<'a, 'b> Stream<'a, 'b> {
    /// Creates a new [`Stream`]
    pub(in crate::request) fn new(
        buffer: &'a mut HTTPHeaderBuffer,
        stream: &'b mut TcpStream,
    ) -> Self {
        buffer.reset();

        Stream { buffer, stream }
    }

    /// Attempts to collect bytes from the stream until a `predicate` returns true
    ///
    /// This function returns a slice with the bytes up to and including the byte the `predicate`
    /// returns true on. The next character returned by this stream will be the one following the
    /// character the predicate returned true on.
    ///
    /// If the end of the buffer is reached before the `predicate` returns true, the function will
    /// return an [`HTTPParseError::HeadersTooLong`]
    pub(super) fn collect_until_predicate<F: Fn(u8) -> bool>(
        &mut self,
        predicate: F,
    ) -> Result<&'a [u8], HTTPParseError> {
        let start = self.buffer.index();

        while !predicate(self.buffer.next(self.stream)?) {}

        Ok(unsafe { self.buffer.subslice(start, self.buffer.index()) })
    }

    /// Attempts to collect bytes from the stream until a `c` byte is encountered
    ///
    /// This function returns a slice with the bytes up to and including `c`. The next character
    /// returned by this stream will be the one following `c`.
    ///
    /// If the end of the buffer is reached before a `c` byte is encountered, the function will
    /// return an [`HTTPParseError::HeadersTooLong`]
    pub(super) fn collect_until(&mut self, c: u8) -> Result<&'a [u8], HTTPParseError> {
        self.collect_until_predicate(|value| c == value)
    }
}
