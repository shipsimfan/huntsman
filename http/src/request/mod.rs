use header::{HTTPHeaderBuffer, Stream};

mod error;
mod header;
mod parser;

pub use error::HTTPParseError;
pub use header::{HTTPMethod, HTTPRequestHeader};
pub use parser::HTTPRequestParser;

/// An HTTP request received from a client
#[derive(Debug)]
pub struct HTTPRequest<'a> {
    header: HTTPRequestHeader<'a>,
}

impl<'a> HTTPRequest<'a> {
    /// Attempts to parse an [`HTTPRequest`] from `stream`
    pub(self) fn parse(stream: &mut Stream<'a, '_>) -> Result<Self, HTTPParseError> {
        let header = HTTPRequestHeader::parse(stream)?;

        Ok(HTTPRequest { header })
    }
}
