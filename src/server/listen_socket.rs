use linux::{IPv4SocketAddress, Socket};

/// A socket bound to a port listening for clients
pub(super) struct ListenSocket(Socket);

impl ListenSocket {
    /// Creates a new [`ListenSocket`] listening on `port`
    pub(super) fn new(port: u16) -> linux::Result<Self> {
        let listen_socket =
            linux::Socket::new(linux::AddressFamily::INet, linux::SocketType::Stream, 0)?;

        listen_socket.bind(&IPv4SocketAddress {
            port,
            ..Default::default()
        })?;

        listen_socket.listen(128)?;

        Ok(ListenSocket(listen_socket))
    }
}
