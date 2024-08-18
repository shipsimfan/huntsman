use super::parse_segment_until;
use crate::HTTPTarget;
use std::{borrow::Cow, fmt::Debug};

mod from;
mod from_many;

pub use from::FromHTTPQueryParam;
pub use from_many::FromHTTPQueryParams;

/// A query parameter passed in an HTTP request
#[derive(Clone, PartialEq, Eq)]
pub struct HTTPQueryParam<'a> {
    key: Cow<'a, [u8]>,
    value: Cow<'a, [u8]>,
}

impl<'a> HTTPQueryParam<'a> {
    /// Parse an [`HTTPQueryParam`] from `stream`
    pub(super) fn parse(i: usize, target: HTTPTarget<'a>) -> (Self, usize) {
        let (key, i) = parse_segment_until(i, target, |x| x == b'=' || x == b'&');
        let (value, i) = if target[i] == b'=' {
            parse_segment_until(i + 1, target, |x| x == b'&')
        } else {
            (Cow::Borrowed(&[] as &[u8]), i)
        };

        (HTTPQueryParam { key, value }, i + 1)
    }

    /// Gets the key of this query parameter
    pub fn key(&self) -> &[u8] {
        &self.key
    }

    /// Gets the value of this query parameter
    pub fn value(&self) -> &[u8] {
        &self.value
    }
}

impl<'a> Debug for HTTPQueryParam<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HTTPQueryParam ")?;
        f.debug_map()
            .entry(&"key", &String::from_utf8_lossy(&self.key))
            .entry(&"value", &String::from_utf8_lossy(&self.value))
            .finish()
    }
}
