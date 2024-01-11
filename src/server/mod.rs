use listen_socket::ListenSocket;

mod listen_socket;

/// A huntsman server
pub struct Server {
    listen_socket: ListenSocket,
}

impl Server {
    /// Creates an new [`Server`] and runs it
    pub fn run(port: u16) -> linux::Result<!> {
        let mut server = Server::new(port)?;

        loop {}
    }

    /// Creates a new [`Server`]
    fn new(port: u16) -> linux::Result<Self> {
        let listen_socket = ListenSocket::new(port)?;

        Ok(Server { listen_socket })
    }
}
