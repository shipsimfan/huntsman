use super::Stream;
use crate::HTTPParseError;
use std::ops::Deref;

/// The requested target of an HTTP request
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HTTPTarget<'a>(&'a [u8]);

impl<'a> HTTPTarget<'a> {
    /// Attempts to parse an [`HTTPTarget`] from `stream`
    pub(super) async fn parse(stream: &mut Stream<'a, '_>) -> Result<Self, HTTPParseError> {
        let uri = stream
            .collect_until_predicate_error(|c| match c {
                b' ' => Ok(true),
                x if x.is_ascii_alphanumeric() => Ok(false),
                b'!' | b'$' | b'%' | b'&' | b'\'' | b'(' | b')' | b'*' | b'+' | b',' | b'-'
                | b'.' | b'/' | b':' | b';' | b'=' | b'?' | b'@' | b'_' | b'~' => Ok(false),
                _ => Err(HTTPParseError::InvalidTarget),
            })
            .await?;

        Ok(HTTPTarget(&uri[..uri.len() - 1]))
    }

    /// Gets the target as a slice of bytes
    pub fn as_slice(&self) -> &'a [u8] {
        self.0
    }
}

impl<'a> Deref for HTTPTarget<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> std::fmt::Display for HTTPTarget<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { std::str::from_utf8_unchecked(self.0) })
    }
}

impl<'a> std::fmt::Debug for HTTPTarget<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
