use crate::{
    client::HTTPSocket, HTTPChunkedResponseBody, HTTPClient, HTTPClientAddress, HTTPListenAddress,
    HTTPOptions, HTTPProtocol, Result,
};
use huntsman::ProtocolListener;
use lasync::net::TCPListener;
use std::{marker::PhantomData, net::SocketAddr};

/// The sockets to listen for connections on
pub enum HTTPListener<B: HTTPChunkedResponseBody> {
    /// The listener for insecure HTTP/1.1 connections
    HTTP(TCPListener, PhantomData<B>),
}

impl<B: HTTPChunkedResponseBody> HTTPListener<B> {
    /// Creates a new [`Listener`] for `address`
    pub(crate) fn new(address: &HTTPListenAddress) -> Result<(Self, HTTPListenAddress)> {
        match address {
            HTTPListenAddress::HTTP(address) => HTTPListener::new_http(*address),
        }
    }

    /// Creates a new [`Listener`] for insecure HTTP/1.1 connectio
    fn new_http(address: SocketAddr) -> Result<(Self, HTTPListenAddress)> {
        let socket = TCPListener::bind(address)?;
        let listen_address = socket.local_addr().unwrap();
        Ok((
            HTTPListener::HTTP(socket, PhantomData),
            HTTPListenAddress::HTTP(listen_address),
        ))
    }
}

impl<B: HTTPChunkedResponseBody> ProtocolListener for HTTPListener<B> {
    type Address = HTTPListenAddress;
    type Client = HTTPClient<B>;
    type ClientAddress = HTTPClientAddress;
    type Error = lasync::Error;
    type Options = HTTPOptions;

    async fn accept(
        &self,
        options: &Self::Options,
    ) -> std::result::Result<(Self::Client, Self::ClientAddress), Self::Error> {
        let (socket, client_address) = match self {
            HTTPListener::HTTP(listener, _) => {
                let (mut socket, socket_address) = listener.accept().await?;
                socket.set_nodelay(true)?;
                (
                    HTTPSocket::HTTP(socket),
                    HTTPClientAddress::new(HTTPProtocol::HTTP, socket_address),
                )
            }
        };

        let client = HTTPClient::new(
            socket,
            options.max_header_size,
            options.max_body_size,
            options.header_read_timeout,
            options.body_read_timeout,
            options.write_timeout,
        )?;

        Ok((client, client_address))
    }
}
