/// The options to determine how HTTP will operate
#[derive(Clone, PartialEq, Eq)]
pub struct HTTPOptions {
    /// The maximum size for HTTP headers in requests
    pub max_header_size: usize,

    /// The maximum size for bodies in HTTP requests
    pub max_body_size: usize,
}

impl Default for HTTPOptions {
    fn default() -> Self {
        HTTPOptions {
            max_header_size: 8192,      // 8 Kb
            max_body_size: 1024 * 1024, // 1 Mb
        }
    }
}
