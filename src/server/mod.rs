use crate::LinuxResult;
use listen_socket::ListenSocket;

mod listen_socket;

/// A huntsman server
pub(crate) struct Server {
    listen_socket: ListenSocket,
}

impl Server {
    /// Creates a new [`Server`]
    pub(crate) fn new(port: u16) -> LinuxResult<Self> {
        let listen_socket = ListenSocket::new(port)?;

        Ok(Server { listen_socket })
    }
}
