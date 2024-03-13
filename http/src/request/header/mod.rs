mod method;

pub use method::HTTPMethod;

/// The header of an HTTP request
pub struct HTTPRequestHeader {
    /// The request method
    method: HTTPMethod,
}
