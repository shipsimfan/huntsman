use crate::HTTPParseError;

mod buffer;
mod field;
mod method;
mod stream;
mod target;

pub use field::HTTPField;
pub use method::HTTPMethod;
pub use target::HTTPTarget;

pub(super) use buffer::HTTPHeaderBuffer;
pub(super) use stream::Stream;

/// The header of an HTTP request
#[derive(Debug)]
pub struct HTTPRequestHeader<'a> {
    /// The request method
    method: HTTPMethod,

    /// The target for the request
    target: HTTPTarget<'a>,

    /// The fields containing metadata about this request
    fields: Vec<HTTPField<'a>>,
}

impl<'a> HTTPRequestHeader<'a> {
    /// Attempts to parse an [`HTTPRequestHeader`] from `stream`
    pub(super) fn parse(stream: &mut Stream<'a, '_>) -> Result<Self, HTTPParseError> {
        let method = HTTPMethod::parse(stream)?;

        let target = HTTPTarget::parse(stream)?;

        let version = stream.collect_until_newline()?;
        if &version[..version.len() - 2] != b"HTTP/1.1" {
            return Err(HTTPParseError::InvalidVersion);
        }

        let mut fields = Vec::new();
        loop {
            if stream.peek()? == b'\r' {
                let end = stream.collect_until_newline()?;
                if end != b"\r\n" {
                    return Err(HTTPParseError::InvalidField);
                }

                break;
            }

            fields.push(HTTPField::parse(stream)?);
        }

        Ok(HTTPRequestHeader {
            method,
            target,
            fields,
        })
    }

    /// Gets the method of this request
    pub fn method(&self) -> HTTPMethod {
        self.method
    }

    /// Gets the target of this request
    pub fn target(&self) -> HTTPTarget<'a> {
        self.target
    }

    /// Gets the fields of this request
    pub fn fields(&self) -> &[HTTPField<'a>] {
        &self.fields
    }
}
