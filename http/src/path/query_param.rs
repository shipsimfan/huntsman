use super::parse_segment_until;
use crate::HTTPTarget;
use std::borrow::Cow;

/// A query parameter passed in an HTTP request
pub struct HTTPQueryParam<'a> {
    key: Cow<'a, [u8]>,
    value: Cow<'a, [u8]>,
}

impl<'a> HTTPQueryParam<'a> {
    /// Parse an [`HTTPQueryParam`] from `stream`
    pub(super) fn parse(i: usize, target: &'a HTTPTarget<'a>) -> (Self, usize) {
        let (key, i) = parse_segment_until(i, target, |x| x == b'=' || x == b'&');
        let (value, i) = if target[i] == b'=' {
            parse_segment_until(i, target, |x| x == b'&')
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
