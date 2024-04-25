//! HTTP implementation for huntsman

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use client::Stream;
use huntsman::Protocol;
use listeners::Listeners;
use std::time::Duration;

mod client;
mod listen_address;
mod listeners;
mod options;
mod request;
mod response;

pub use client::{HTTPClient, HTTPClientAddress, HTTPProtocol};
pub use lasync::{Error, Result};
pub use listen_address::ListenAddress;
pub use options::HTTPOptions;
pub use request::{
    HTTPMethod, HTTPParseError, HTTPRequest, HTTPRequestField, HTTPRequestHeader, HTTPTarget,
};
pub use response::{HTTPResponse, HTTPStatus};

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

    /// The maximum amount of time allowed between header reads
    header_read_timeout: Duration,

    /// The maximum amount of time allowed between body reads
    body_read_timeout: Duration,

    /// The maximum amount of time allowed between writes
    write_timeout: Duration,
}

impl Protocol for HTTP {
    type Options = HTTPOptions;

    type ClientAddress = HTTPClientAddress;

    type ListenAddress = ListenAddress;

    type Request<'a> = HTTPRequest<'a>;

    type Response<'a> = HTTPResponse<'a>;

    type ListenError = lasync::Error;

    type ReadError = HTTPParseError;

    type SendError = lasync::Error;

    type Client = HTTPClient;

    async fn start(address: &Self::ListenAddress, options: Self::Options) -> Result<Self> {
        let (listeners, listen_address) = Listeners::new(address.clone())?;

        Ok(HTTP {
            listeners,
            listen_address,
            max_header_size: options.max_header_size,
            max_body_size: options.max_body_size,
            header_read_timeout: options.header_read_timeout,
            body_read_timeout: options.body_read_timeout,
            write_timeout: options.write_timeout,
        })
    }

    async fn address(&mut self) -> Self::ListenAddress {
        self.listen_address.clone()
    }

    async fn accept(&self) -> Result<(Self::Client, Self::ClientAddress)> {
        let (socket, address) = self.listeners.accept().await?;

        let client = HTTPClient::new(
            socket,
            self.max_header_size,
            self.max_body_size,
            self.header_read_timeout,
            self.body_read_timeout,
            self.write_timeout,
        )?;

        Ok((client, address))
    }
}
