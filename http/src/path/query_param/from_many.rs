use super::HTTPQueryParam;

/// A type which can be parsed from a set of query parameters
pub trait FromHTTPQueryParams<'a>: Sized {
    /// An error that can occur while parsing http query parameters
    type Error: std::error::Error;

    /// Attempts to parse `query_params` into `Self`
    fn from_query_params(query_params: &'a [HTTPQueryParam]) -> Result<Self, Self::Error>;
}
