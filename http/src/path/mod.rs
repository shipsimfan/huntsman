use crate::HTTPTarget;
use std::borrow::Cow;

mod query_param;

pub use query_param::HTTPQueryParam;

/// A parsed HTTP request path
pub struct HTTPPath<'a> {
    segments: Vec<Cow<'a, str>>,
    query_params: Vec<HTTPQueryParam<'a>>,
}

impl<'a> HTTPPath<'a> {
    /// Parses `target` into an [`HTTPPath`]
    pub fn parse(target: &HTTPTarget<'a>) -> Self {
        todo!()
    }

    /// Gets the number of segments which make up this path
    pub fn num_segments(&self) -> usize {
        self.segments.len()
    }

    /// Gets the segment at index `i`
    pub fn segment(&self, i: usize) -> &str {
        &self.segments[i]
    }

    /// Gets an iterator over the segments of this path
    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.segments.iter().map(|str| str.as_ref())
    }

    /// Gets the query parameters of the request path
    pub fn query_params(&self) -> &[HTTPQueryParam<'a>] {
        &self.query_params
    }

    /// Attempts to get a query parameter with `key`
    pub fn query_param(&self, key: &str) -> Option<&str> {
        for query_param in &self.query_params {
            if query_param.key() == key {
                return Some(query_param.value());
            }
        }

        None
    }
}
