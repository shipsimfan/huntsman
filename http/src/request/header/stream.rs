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
    /// This function will provide the `predicate` with the next two bytes from the stream. The
    /// predicate will still be called for every byte, meaning the second parameter passed to the
    /// predicate in one call will be the first parameter in the next call.
    ///
    /// This function returns a slice with the bytes up to and including the second byte the
    /// `predicate` returns true on. The next character returned by this stream will be the one
    /// following the character the predicate returned true on.
    ///
    /// If the end of the buffer is reached before the `predicate` returns true, the function will
    /// return an [`HTTPParseError::HeadersTooLong`]
    pub(super) fn collect_until_predicate_double<F: Fn(u8, u8) -> bool>(
        &mut self,
        predicate: F,
    ) -> Result<&'a [u8], HTTPParseError> {
        let start = self.buffer.index();

        let mut prev = self.buffer.next(self.stream)?;
        let mut next = self.buffer.next(self.stream)?;

        while !predicate(prev, next) {
            prev = next;
            next = self.buffer.next(self.stream)?;
        }

        Ok(unsafe { self.buffer.subslice(start, self.buffer.index()) })
    }

    /// Attempts to collect bytes from the stream until a `predicate` returns true
    ///
    /// This function returns a slice with the bytes up to and including the byte the `predicate`
    /// returns true on. The next character returned by this stream will be the one following the
    /// character the predicate returned true on.
    ///
    /// If the end of the buffer is reached before the `predicate` returns true, the function will
    /// return an [`HTTPParseError::HeadersTooLong`]
    pub(super) fn collect_until_predicate_error<F: Fn(u8) -> Result<bool, HTTPParseError>>(
        &mut self,
        predicate: F,
    ) -> Result<&'a [u8], HTTPParseError> {
        let start = self.buffer.index();

        while !predicate(self.buffer.next(self.stream)?)? {}

        Ok(unsafe { self.buffer.subslice(start, self.buffer.index()) })
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
        self.collect_until_predicate_error(|value| Ok(predicate(value)))
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

    /// Attempts to collect bytes from the stream until `a` followed by `b` is encountered.
    ///
    /// This function returns a slice with the bytes up to and including the `b` encountered. The
    /// next character returned by this stream will be the one following that `b`.
    ///
    /// If the end of the buffer is reached before the pattern is matched, the function will return
    /// an [`HTTPParseError::HeadersTooLong`]
    pub(super) fn collect_until_double(
        &mut self,
        a: u8,
        b: u8,
    ) -> Result<&'a [u8], HTTPParseError> {
        self.collect_until_predicate_double(|na, nb| na == a && nb == b)
    }

    /// Attempts to collect bytes from the stream until a newline ("\r\n") is encountered.
    ///
    /// This function returns a slice with the bytes up to and including the newline that was
    /// encountered. The next character returned by this stream will be the one following the
    /// newline.
    ///
    /// If the end of the buffer is reached before a newline is found, the function will return an
    /// [`HTTPParseError::HeadersTooLong`]
    pub(super) fn collect_until_newline(&mut self) -> Result<&'a [u8], HTTPParseError> {
        self.collect_until_double(b'\r', b'\n')
    }
}
