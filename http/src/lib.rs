use huntsman::Protocol;
use std::net::TcpListener;

mod request;
mod response;

pub use request::{HTTPRequest, HTTPRequestParser};
pub use response::HTTPResponse;

/// The HTTP protocol
pub struct HTTP;

impl Protocol for HTTP {
    type Transport = TcpListener;

    type RequestParser = HTTPRequestParser;

    type Response = HTTPResponse;
}
