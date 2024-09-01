//! HTTP implementation for huntsman

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(addr_parse_ascii)]

use client::Stream;
use huntsman::Protocol;
use listener::HTTPListener;

mod client;
mod listen_address;
mod listener;
mod options;
mod path;
mod request;
mod response;

pub use client::{HTTPClient, HTTPClientAddress, HTTPProtocol};
pub use lasync::{Error, Result};
pub use listen_address::HTTPListenAddress;
pub use options::HTTPOptions;
pub use path::{HTTPPath, HTTPQueryParam};
pub use request::{
    HTTPMethod, HTTPParseError, HTTPRequest, HTTPRequestDisplay, HTTPRequestField,
    HTTPRequestHeader, HTTPTarget,
};
pub use response::{
    EmptyHTTPChunkedResponseBody, HTTPChunkedResponseBody, HTTPResponse, HTTPResponseBodyContent,
    HTTPStatus, ReadHTTPChunkedResponseBody,
};

/// The HTTP protocol
pub struct HTTP<B: HTTPChunkedResponseBody = EmptyHTTPChunkedResponseBody> {
    /// The sockets for accepting clients
    listeners: Vec<HTTPListener<B>>,

    /// The addresses the server is listening on
    listen_addresses: Vec<HTTPListenAddress>,

    /// The options to define how this server should run
    options: HTTPOptions,
}

impl<B: HTTPChunkedResponseBody> Protocol for HTTP<B> {
    type Options = HTTPOptions;

    type ClientAddress = HTTPClientAddress;
    type Request<'a> = HTTPRequest<'a>;
    type Response<'a> = HTTPResponse<'a, B>;
    type ReadError = HTTPParseError;
    type SendError = lasync::Error;
    type Client = HTTPClient<B>;

    type ListenAddress = HTTPListenAddress;
    type ListenError = lasync::Error;
    type Listener = HTTPListener<B>;

    async fn start(addresses: &[Self::ListenAddress], options: Self::Options) -> Result<Self> {
        let mut listeners = Vec::with_capacity(addresses.len());
        let mut listen_addresses = Vec::with_capacity(addresses.len());
        for address in addresses {
            let (listener, listen_address) = HTTPListener::new(address)?;
            listeners.push(listener);
            listen_addresses.push(listen_address);
        }

        Ok(HTTP {
            listeners,
            listen_addresses,
            options,
        })
    }

    fn addresses(&self) -> &[Self::ListenAddress] {
        &self.listen_addresses
    }

    fn listeners(&self) -> &[Self::Listener] {
        &self.listeners
    }

    fn options(&self) -> &Self::Options {
        &self.options
    }
}

unsafe impl<B: HTTPChunkedResponseBody> Send for HTTP<B> {}
unsafe impl<B: HTTPChunkedResponseBody> Sync for HTTP<B> {}
