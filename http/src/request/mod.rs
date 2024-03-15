use buffer::Buffer;
use stream::Stream;

mod buffer;
mod error;
mod header;
mod parser;
mod stream;

pub use error::HTTPParseError;
pub use header::{HTTPMethod, HTTPRequestHeader};
pub use parser::HTTPRequestParser;

/// An HTTP request received from a client
pub struct HTTPRequest {}
