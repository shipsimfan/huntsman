use super::Stream;
use crate::HTTPParseError;

/// A field containing metadata about an HTTP request
pub struct HTTPRequestField<'a> {
    /// The name of the field
    name: &'a [u8],

    /// The contained value
    value: &'a [u8],
}

impl<'a> HTTPRequestField<'a> {
    /// Attempts to parse an [`HTTPField`] from `stream`
    pub(super) async fn parse(stream: &mut Stream<'a, '_>) -> Result<Self, HTTPParseError> {
        let name = stream
            .collect_until_predicate_error(|c| match c {
                b':' => Ok(true),
                x if x.is_ascii_alphanumeric() => Ok(false),
                b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'.' | b'^'
                | b'_' | b'`' | b'|' | b'~' => Ok(false),
                _ => Err(HTTPParseError::InvalidField),
            })
            .await?;

        stream.skip_whitespace().await?;

        let value = stream
            .collect_until_predicate_error(|c| match c {
                b'\r' => Ok(true),
                x if x >= 0x21 => Ok(false),
                b' ' | b'\t' => Ok(false),
                _ => Err(HTTPParseError::InvalidField),
            })
            .await?;

        if stream.next().await? != b'\n' {
            println!("newline?");
            return Err(HTTPParseError::InvalidField);
        }

        Ok(HTTPRequestField {
            name: &name[..name.len() - 1],
            value: &value[..value.len() - 1],
        })
    }

    /// Gets the name of this field
    pub fn name(&self) -> &'a [u8] {
        self.name
    }

    /// Gets the value of this field
    pub fn value(&self) -> &'a [u8] {
        self.value
    }
}

impl<'a> std::fmt::Display for HTTPRequestField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            String::from_utf8_lossy(self.name),
            String::from_utf8_lossy(self.value)
        )
    }
}

impl<'a> std::fmt::Debug for HTTPRequestField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HTTPField")
            .field("name", &String::from_utf8_lossy(self.name))
            .field("value", &String::from_utf8_lossy(self.value))
            .finish()
    }
}
