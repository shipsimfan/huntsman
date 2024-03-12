use huntsman::Protocol;
use std::net::TcpListener;

/// The HTTP protocol
pub struct HTTP;

impl Protocol for HTTP {
    type Transport = TcpListener;
}
