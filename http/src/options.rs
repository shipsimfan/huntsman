use std::time::Duration;

/// The options to determine how HTTP will operate
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HTTPOptions {
    /// The maximum size for HTTP headers in requests
    pub max_header_size: usize,

    /// The maximum size for bodies in HTTP requests
    pub max_body_size: usize,

    /// The maximum amount of time allowed between header reads
    pub header_read_timeout: Duration,

    /// The maximum amount of time allowed between body reads
    pub body_read_timeout: Duration,

    /// The maximum amount of time allowed between writes
    pub write_timeout: Duration,
}

impl Default for HTTPOptions {
    fn default() -> Self {
        HTTPOptions {
            max_header_size: 8192,      // 8 Kb
            max_body_size: 1024 * 1024, // 1 Mb
            header_read_timeout: Duration::from_secs(60),
            body_read_timeout: Duration::from_secs(60),
            write_timeout: Duration::from_secs(60),
        }
    }
}
