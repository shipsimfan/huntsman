use crate::{ListenAddress, Result};
use lasync::futures::net::{TCPListener, TCPStream};
use std::net::SocketAddr;

/// The sockets to listen for connections on
pub(super) struct Listeners {
    /// The listener for unsecured HTTP/1.1 connections
    http: Option<TCPListener>,
}

impl Listeners {
    /// Creates a new set of [`Listeners`] bound to `addresses`
    pub(super) fn new(addresses: ListenAddress) -> Result<(Self, ListenAddress)> {
        let mut listen_address = ListenAddress::empty();

        let http = match addresses.http {
            Some(address) => {
                let socket = TCPListener::bind(address.into())?;
                listen_address.http = Some(socket.local_addr().unwrap());
                Some(socket)
            }
            None => None,
        };

        Ok((Listeners { http }, listen_address))
    }

    /// Accepts a client that is attempting to connect to this socket
    pub(super) async fn accept(&self) -> Result<(TCPStream, SocketAddr)> {
        if let Some(http) = &self.http {
            http.accept().await
        } else {
            panic!("No listeners have been enabled!");
        }
    }
}
