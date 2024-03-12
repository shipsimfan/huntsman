use huntsman::Protocol;
use std::net::TcpListener;

mod request;
mod response;

pub use request::HTTPRequest;
pub use response::HTTPResponse;

/// The HTTP protocol
pub struct HTTP;

impl Protocol for HTTP {
    type Transport = TcpListener;

    type Request = HTTPRequest;

    type Response = HTTPResponse;
}
