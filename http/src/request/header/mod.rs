use crate::{HTTPParseError, Stream};

mod field;
mod method;
mod target;

pub use field::HTTPRequestField;
pub use method::HTTPMethod;
pub use target::HTTPTarget;

/// The header of an HTTP request
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HTTPRequestHeader<'a> {
    /// The request method
    method: HTTPMethod,

    /// The target for the request
    target: HTTPTarget<'a>,

    /// The fields containing metadata about this request
    fields: Vec<HTTPRequestField<'a>>,
}

impl<'a> HTTPRequestHeader<'a> {
    /// Attempts to parse an [`HTTPRequestHeader`] from `stream`
    pub(super) async fn parse(stream: &mut Stream<'a, '_>) -> Result<Self, HTTPParseError> {
        let method = HTTPMethod::parse(stream).await?;

        let target = HTTPTarget::parse(stream).await?;

        let version = stream.collect_until_newline().await?;
        if &version[..version.len() - 2] != b"HTTP/1.1" {
            return Err(HTTPParseError::InvalidVersion);
        }

        let mut fields = Vec::new();
        loop {
            if stream.peek().await? == b'\r' {
                let end = stream.collect_until_newline().await?;
                if end != b"\r\n" {
                    return Err(HTTPParseError::InvalidField);
                }

                break;
            }

            fields.push(HTTPRequestField::parse(stream).await?);
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
    pub fn fields(&self) -> &[HTTPRequestField<'a>] {
        &self.fields
    }

    /// Gets a contained field based on its name
    pub fn field(&self, name: &[u8]) -> Option<&HTTPRequestField<'a>> {
        for field in &self.fields {
            if field.name() == name {
                return Some(field);
            }
        }

        None
    }
}
