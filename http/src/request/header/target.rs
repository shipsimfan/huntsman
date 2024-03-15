// Allow characters:
// !                            33
// $                            36
// %                            37
// &                            38
// '                            39
// (                            40
// )                            41
// *                            42
// +                            43
// ,                            44
// -                            45
// .                            46
// '/' - Must start with one    47
// 0-9                          48 - 57
// :                            58
// ;                            59
// =                            61
// ?                            63
// @                            64
// A-Z                          65 - 90
// _                            95
// a-z                          97 - 122
// ~                            126

use super::Stream;
use crate::HTTPParseError;
use std::ops::Deref;

/// The requested target of an HTTP request
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HTTPTarget<'a>(&'a [u8]);

impl<'a> HTTPTarget<'a> {
    /// Attempts to parse an [`HTTPTarget`] from `stream`
    pub(super) fn parse(stream: &mut Stream<'a, '_>) -> Result<Self, HTTPParseError> {
        let uri = stream.collect_until_predicate_error(|c| match c {
            b' ' => Ok(true),
            x if x.is_ascii_alphanumeric() => Ok(false),
            b'!' | b'$' | b'%' | b'&' | b'\'' | b'(' | b')' | b'*' | b'+' | b',' | b'-' | b'.'
            | b'/' | b':' | b';' | b'=' | b'?' | b'@' | b'_' | b'~' => Ok(false),
            _ => Err(HTTPParseError::InvalidTarget),
        })?;

        Ok(HTTPTarget(&uri[..uri.len() - 1]))
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
