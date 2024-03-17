use crate::HTTPParseError;
use std::{io::Read, net::TcpStream};

/// A buffer for more efficient reading from a socket
pub(in crate::request) struct HTTPHeaderBuffer {
    /// The buffer itself
    buffer: Box<[u8]>,

    /// The current length of the buffer
    length: usize,

    /// The index of the next byte to read
    index: usize,
}

impl HTTPHeaderBuffer {
    /// Creates a new [`Buffer`] with `capacity` bytes of space
    pub(in crate::request) fn new(capacity: usize) -> Self {
        let buffer = vec![0; capacity].into_boxed_slice();

        HTTPHeaderBuffer {
            buffer,
            length: 0,
            index: 0,
        }
    }

    /// Gets the current index into the buffer
    pub(super) fn index(&self) -> usize {
        self.index
    }

    /// Gets a subslice from the buffer
    ///
    /// # SAFETY
    /// This function will return a slice not connected to the `self` borrow's lifetime. The
    /// lifetime must be garunteed by the caller to be between resets.
    pub(super) unsafe fn subslice<'a>(&self, start: usize, end: usize) -> &'a [u8] {
        assert!(start <= end);
        assert!(end <= self.index);

        std::slice::from_raw_parts(&self.buffer[start], end - start)
    }

    /// Reads the next value from `stream` without consuming it
    pub(super) fn peek(&mut self, stream: &mut TcpStream) -> Result<u8, HTTPParseError> {
        if self.index == self.buffer.len() {
            return Err(HTTPParseError::HeadersTooLong);
        }

        if self.index == self.length {
            self.read(stream)?;
        }

        Ok(self.buffer[self.index])
    }

    /// Gets the next byte from the buffer or reads more bytes from `stream`
    pub(super) fn next(&mut self, stream: &mut TcpStream) -> Result<u8, HTTPParseError> {
        let ret = self.peek(stream)?;
        self.index += 1;
        Ok(ret)
    }

    /// Resets this buffer so that reading starts at the beginning
    pub(super) fn reset(&mut self) {
        if self.index == 0 {
            return;
        }

        self.length -= self.index;
        for i in 0..self.length {
            self.buffer[i] = self.buffer[self.index + i];
        }

        self.index = 0;
    }

    /// Copies the remaining bytes in this buffer into `buffer`
    pub(super) fn copy_body(&mut self, buffer: &mut [u8]) -> usize {
        let remaining = self.length - self.index;

        let copy_length = remaining.min(buffer.len());
        (&mut buffer[..copy_length])
            .copy_from_slice(&mut self.buffer[self.index..self.index + copy_length]);
        self.index += copy_length;

        copy_length
    }

    /// Extend the buffer by reading from `stream`
    fn read(&mut self, stream: &mut TcpStream) -> Result<(), HTTPParseError> {
        assert_ne!(self.length, self.buffer.len());

        let count = stream.read(&mut self.buffer[self.length..])?;

        if count == 0 {
            return Err(HTTPParseError::IncompleteHeader);
        }

        self.length += count;
        Ok(())
    }
}
