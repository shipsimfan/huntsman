use crate::{try_linux, LinuxResult};
use linux::{
    netinet::r#in::sockaddr_in,
    sys::socket::{bind, listen, socket, AF_INET, SOCK_STREAM},
    unistd::close,
};
use std::ffi::c_int;

/// A socket bound to a port listening for clients
pub(super) struct ListenSocket(c_int);

impl ListenSocket {
    /// Creates a new [`ListenSocket`] listening on `port`
    pub(super) fn new(port: u16) -> LinuxResult<Self> {
        let socket = try_linux!(socket(AF_INET, SOCK_STREAM, 0))?;

        let address = sockaddr_in {
            port: port.to_be(),
            ..Default::default()
        };
        try_linux!(bind(
            socket,
            &address as *const _ as _,
            std::mem::size_of_val(&address) as _
        ))?;

        try_linux!(listen(socket, 16))?;

        Ok(ListenSocket(socket))
    }
}

impl Drop for ListenSocket {
    fn drop(&mut self) {
        unsafe { close(self.0) };
    }
}
