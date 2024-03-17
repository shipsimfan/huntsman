//! HTTP implementation for huntsman

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use client::Stream;
use huntsman::Protocol;
use std::net::{SocketAddr, TcpListener};

mod client;
mod options;
mod request;
mod response;

pub use client::HTTPClient;
pub use options::HTTPOptions;
pub use request::{HTTPMethod, HTTPParseError, HTTPRequest, HTTPRequestHeader};
pub use response::{HTTPResponse, HTTPResponseField, HTTPStatus};

/// The HTTP protocol
pub struct HTTP {
    /// The socket for accepting clients
    listener: TcpListener,

    /// The address to listen on
    address: [SocketAddr; 1],

    /// The maximum size for headers in requests
    max_header_size: usize,

    /// The maximum size for bodies in requests
    max_body_size: usize,
}

impl Protocol for HTTP {
    type Options = HTTPOptions;

    type Address = SocketAddr;

    type Request<'a> = HTTPRequest<'a>;

    type Response = HTTPResponse;

    type ListenError = std::io::Error;

    type ReadError = HTTPParseError;

    type SendError = std::io::Error;

    type Client = HTTPClient;

    fn start(options: Self::Options) -> Result<Self, Self::ListenError> {
        let listener = TcpListener::bind(options.address)?;

        Ok(HTTP {
            listener,
            address: [options.address],
            max_header_size: options.max_header_size,
            max_body_size: options.max_body_size,
        })
    }

    fn get_addresses(&mut self) -> &[Self::Address] {
        &self.address
    }

    fn accept(&mut self) -> Result<(Self::Client, Self::Address), Self::ListenError> {
        self.listener.accept().map(|(socket, address)| {
            (
                HTTPClient::new(socket, self.max_header_size, self.max_body_size),
                address,
            )
        })
    }
}
