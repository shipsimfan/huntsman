use super::HeaderBuffer;
use crate::HTTPParseError;
use std::net::TcpStream;

/// A stream of bytes from a [`TcpStream`]
pub(crate) struct Stream<'a, 'b> {
    /// The buffer to improve read efficiency
    buffer: &'a mut HeaderBuffer,

    /// The stream to read from
    socket: &'b mut TcpStream,
}

impl<'a, 'b> Stream<'a, 'b> {
    /// Creates a new [`Stream`]
    pub(super) fn new(buffer: &'a mut HeaderBuffer, socket: &'b mut TcpStream) -> Self {
        buffer.reset();

        Stream { buffer, socket }
    }

    /// Reads the next value from the stream without consuming it
    pub(crate) fn peek(&mut self) -> Result<u8, HTTPParseError> {
        self.buffer.peek(self.socket)
    }

    pub(crate) fn next(&mut self) -> Result<u8, HTTPParseError> {
        self.buffer.next(self.socket)
    }

    /// Skips any whitespace (space or tab) characters in the stream until reaching a
    /// non-whitespace character
    pub(crate) fn skip_whitespace(&mut self) -> Result<(), HTTPParseError> {
        loop {
            match self.peek()? {
                b' ' | b'\t' => self.next()?,
                _ => break,
            };
        }

        Ok(())
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
    pub(crate) fn collect_until_predicate_double<F: Fn(u8, u8) -> bool>(
        &mut self,
        predicate: F,
    ) -> Result<&'a [u8], HTTPParseError> {
        let start = self.buffer.index();

        let mut prev = self.next()?;
        let mut next = self.next()?;

        while !predicate(prev, next) {
            prev = next;
            next = self.next()?;
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
    pub(crate) fn collect_until_predicate_error<F: Fn(u8) -> Result<bool, HTTPParseError>>(
        &mut self,
        predicate: F,
    ) -> Result<&'a [u8], HTTPParseError> {
        let start = self.buffer.index();

        while !predicate(self.next()?)? {}

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
    pub(crate) fn collect_until_predicate<F: Fn(u8) -> bool>(
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
    pub(crate) fn collect_until(&mut self, c: u8) -> Result<&'a [u8], HTTPParseError> {
        self.collect_until_predicate(|value| c == value)
    }

    /// Attempts to collect bytes from the stream until `a` followed by `b` is encountered.
    ///
    /// This function returns a slice with the bytes up to and including the `b` encountered. The
    /// next character returned by this stream will be the one following that `b`.
    ///
    /// If the end of the buffer is reached before the pattern is matched, the function will return
    /// an [`HTTPParseError::HeadersTooLong`]
    pub(crate) fn collect_until_double(
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
    pub(crate) fn collect_until_newline(&mut self) -> Result<&'a [u8], HTTPParseError> {
        self.collect_until_double(b'\r', b'\n')
    }

    /// Creates a buffer for the body, copies any data currently in the header buffer for the body,
    /// and returns the body buffer and the number of bytes copied.
    pub(crate) fn body(self, content_length: usize) -> (&'b mut TcpStream, Box<[u8]>, usize) {
        let mut buffer = vec![0; content_length].into_boxed_slice();

        let length = self.buffer.copy_body(&mut buffer);

        (self.socket, buffer, length)
    }
}
