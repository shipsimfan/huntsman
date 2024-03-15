/// A buffer for more efficient reading from a socket
pub(in crate::request) struct HTTPHeaderBuffer {
    /// The buffer itself
    buffer: Box<[u8]>,

    /// The current length of the buffer
    length: usize,
}

impl HTTPHeaderBuffer {
    /// Creates a new [`Buffer`] with `capacity` bytes of space
    pub(in crate::request) fn new(capacity: usize) -> Self {
        let buffer = vec![0; capacity].into_boxed_slice();

        HTTPHeaderBuffer { buffer, length: 0 }
    }
}
