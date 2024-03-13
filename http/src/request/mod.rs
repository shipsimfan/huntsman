mod error;
mod header;
mod parser;

pub use error::HTTPParseError;
pub use header::{HTTPMethod, HTTPRequestHeader};
pub use parser::HTTPRequestParser;

/// An HTTP request received from a client
pub struct HTTPRequest {}
