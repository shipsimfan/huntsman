//! HTTP implementation for huntsman

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use client::Stream;
use huntsman::Protocol;
use listeners::Listeners;
use std::{future::Future, net::SocketAddr};

mod client;
mod listen_address;
mod listeners;
mod options;
mod request;
mod response;

pub use client::HTTPClient;
pub use lasync::executor::{Error, Result};
pub use listen_address::ListenAddress;
pub use options::HTTPOptions;
pub use request::{HTTPMethod, HTTPParseError, HTTPRequest, HTTPRequestHeader};
pub use response::{HTTPResponse, HTTPResponseField, HTTPStatus};

/// The HTTP protocol
pub struct HTTP {
    /// The socket for accepting clients
    listeners: Listeners,

    /// The addresses the server is listening on
    listen_address: ListenAddress,

    /// The maximum size for headers in requests and responses
    max_header_size: usize,

    /// The maximum size for bodies in requests
    max_body_size: usize,
}

impl Protocol for HTTP {
    type Options = HTTPOptions;

    type ClientAddress = SocketAddr;

    type ListenAddress = ListenAddress;

    type Request<'a> = HTTPRequest<'a>;

    type Response = HTTPResponse;

    type ListenError = lasync::executor::Error;

    type ReadError = HTTPParseError;

    type SendError = lasync::executor::Error;

    type Client = HTTPClient;

    fn start(
        address: &Self::ListenAddress,
        options: Self::Options,
    ) -> impl Future<Output = Result<Self>> {
        async move {
            let listeners = Listeners::new(address.clone())?;

            Ok(HTTP {
                listeners,
                listen_address: address.clone(),
                max_header_size: options.max_header_size,
                max_body_size: options.max_body_size,
            })
        }
    }

    fn address(&mut self) -> impl std::future::Future<Output = Self::ListenAddress> {
        async { self.listen_address.clone() }
    }

    fn accept(&self) -> impl Future<Output = Result<(Self::Client, Self::ClientAddress)>> {
        async {
            let (socket, address) = self.listeners.accept().await?;

            let client = HTTPClient::new(socket, self.max_header_size, self.max_body_size);

            Ok((client, address))
        }
    }
}
