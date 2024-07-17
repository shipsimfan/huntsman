use lasync::{
    io::{Read, Write},
    net::TCPStream,
};

/// A socket which is connected a client
pub(crate) enum HTTPSocket {
    /// The connection is an insecure HTTP/1.1 connection
    HTTP(TCPStream),
}

impl HTTPSocket {
    /// Attempts to read bytes into `buffer` from the socket
    pub(crate) async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, lasync::Error> {
        match self {
            HTTPSocket::HTTP(stream) => stream.read(buffer).await,
        }
    }
    /// Attempts to fill `buffer` by reading bytes from the socket
    pub(crate) async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), lasync::Error> {
        match self {
            HTTPSocket::HTTP(stream) => stream.read_exact(buffer).await,
        }
    }

    /// Attempts to write all bytes from `buffer` into the socket
    pub(crate) async fn write(&mut self, buffer: &[u8]) -> Result<(), lasync::Error> {
        match self {
            HTTPSocket::HTTP(stream) => stream.write_all(buffer).await,
        }
    }
}
