use std::{borrow::Cow, iter::Peekable};

/// A query parameter passed in an HTTP request
pub struct HTTPQueryParam<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> HTTPQueryParam<'a> {
    /// Parse an [`HTTPQueryParam`] from `stream`
    pub(super) fn parse<I: Iterator<Item = u8>>(stream: Peekable<I>) -> Self {
        todo!()
    }

    /// Gets the key of this query parameter
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Gets the value of this query parameter
    pub fn value(&self) -> &str {
        &self.value
    }
}
