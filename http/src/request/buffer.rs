/// A buffer for more efficient reading from a socket
pub(super) struct Buffer {
    /// The buffer itself
    buffer: Box<[u8]>,

    /// The current length of the buffer
    length: usize,

    /// The next byte in the buffer
    index: usize,
}

impl Buffer {
    /// Creates a new [`Buffer`] with `capacity` bytes of space
    pub(super) fn new(capacity: usize) -> Self {
        let buffer = vec![0; capacity].into_boxed_slice();

        Buffer {
            buffer,
            length: 0,
            index: 0,
        }
    }
}
