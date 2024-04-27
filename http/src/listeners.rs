use crate::{HTTPClientAddress, HTTPProtocol, ListenAddress, Result};
use lasync::net::{TCPListener, TCPStream};

/// The sockets to listen for connections on
pub(super) struct Listeners {
    /// The listener for insecure HTTP/1.1 connections
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
    pub(super) async fn accept(&self) -> Result<(TCPStream, HTTPClientAddress)> {
        if let Some(http) = &self.http {
            http.accept().await.map(|(stream, socket_address)| {
                (
                    stream,
                    HTTPClientAddress::new(HTTPProtocol::HTTP, socket_address),
                )
            })
        } else {
            panic!("No listeners have been enabled!");
        }
    }
}
