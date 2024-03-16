//! HTTP implementation for huntsman

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use huntsman::Protocol;
use std::net::TcpListener;

mod request;
mod response;

pub use request::{HTTPMethod, HTTPParseError, HTTPRequest, HTTPRequestHeader, HTTPRequestParser};
pub use response::{HTTPResponse, HTTPStatus};

/// The HTTP protocol
pub struct HTTP;

impl Protocol for HTTP {
    type Transport = TcpListener;

    type RequestParser = HTTPRequestParser;

    type Response = HTTPResponse;
}
