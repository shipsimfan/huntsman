use argparse::{help_flag, parser, parsing_flag, version_flag};
use huntsman_http::HTTPOptions;
use std::{path::PathBuf, time::Duration};

/// Options that control how the server will run
pub struct StaticHuntsmanOptions {
    /* Serving Flags */
    /// The directory to serve files from
    pub base: PathBuf,

    /// The files in a directory to serve when the request path is a directory
    pub indexes: Vec<PathBuf>,

    /// The file to serve if a request is bad
    pub bad_request: Option<PathBuf>,

    /// The file to serve if a requested path isn't found
    pub not_found: Option<PathBuf>,

    /* HTTP Flags */
    /// The HTTP options
    pub http_options: HTTPOptions,
    /* Huntsman Flags */
}

parser! {
    PARSER -> StaticHuntsmanOptions
    "Huntsman Static Server"
    "A server built on huntsman which serves static files",
    [
        // Serving flags
        parsing_flag!("p", "path" "PATH" "missing PATH for path"
                      ["Specify the path to serve files from",
                       "Defaults to \"public/\""]
                      |options: StaticHuntsmanOptions, path: PathBuf| { options.base = path; }
        ).group("SERVING FLAGS"),
        parsing_flag!("i", "index" "FILE" "missing FILE for index"
                      "Specify a default file to serve when a directory is requested"
                      |options: StaticHuntsmanOptions, path: PathBuf| { options.indexes.push(path); }
        ).group("SERVING FLAGS").repeatable(true),
        parsing_flag!(, "bad-request" "FILE" "missing FILE for bad-request"
                      "Specify the file to serve when a request is bad"
                      |options: StaticHuntsmanOptions, path: PathBuf| { options.bad_request = Some(path); }
        ).group("SERVING FLAGS"),
        parsing_flag!(, "not-found" "FILE" "missing FILE for not-found"
                      "Specify the file to serve when a requested file cannot be found"
                      |options: StaticHuntsmanOptions, path: PathBuf| { options.not_found = Some(path); }
        ).group("SERVING FLAGS"),

        // HTTP Flags
        parsing_flag!(, "max-header-size" "SIZE" "missing size for max-header-size"
                      ["Specify the maximum size of request headers in bytes to accpet",
                       "Defaults to 8,192 bytes (8 Kb)"]
                      |options: StaticHuntsmanOptions, size: usize| { options.http_options.max_header_size = size; }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "max-body-size" "SIZE" "missing SIZE for max-body-size"
                      ["Specify the maximum size of request bodies in bytes to accept",
                       "Defaults to 1,048,576 (1 Mb)"]
                      |options: StaticHuntsmanOptions, size: usize| { options.http_options.max_body_size = size; }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "timeout" "TIMEOUT" "missing TIMEOUT for timeout"
                      ["Specify all timeouts to wait TIMEOUT milliseconds",
                       "Defaults to 60,000 milliseconds (1 minute)",
                       "If any conflicting flags are specified, the latest one specified will take precedence"]
                      |options: StaticHuntsmanOptions, timeout: u64| {
                          let timeout = Duration::from_millis(timeout);
                          options.http_options.header_read_timeout = timeout;
                          options.http_options.body_read_timeout = timeout;
                          options.http_options.write_timeout = timeout;
                      }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "read-timeout" "TIMEOUT" "missing TIMEOUT for read-timeout"
                      ["Specify request read timeouts to wait TIMEOUT milliseconds",
                       "Defaults to 60,000 milliseconds (1 minute)",
                       "If any conflicting flags are specified, the latest one specified will take precedence"]
                      |options: StaticHuntsmanOptions, timeout: u64| {
                          let timeout = Duration::from_millis(timeout);
                          options.http_options.header_read_timeout = timeout;
                          options.http_options.body_read_timeout = timeout;
                      }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "header-read-timeout" "TIMEOUT" "missing TIMEOUT for header-read-timeout"
                      ["Specify request header read timeout to wait TIMEOUT milliseconds",
                       "Defaults to 60,000 milliseconds (1 minute)",
                       "If any conflicting flags are specified, the latest one specified will take precedence"]
                      |options: StaticHuntsmanOptions, timeout: u64| {
                          let timeout = Duration::from_millis(timeout);
                          options.http_options.header_read_timeout = timeout;
                      }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "body-read-timeout" "TIMEOUT" "missing TIMEOUT for body-read-timeout"
                      ["Specify request body read timeout to wait TIMEOUT milliseconds",
                       "Defaults to 60,000 milliseconds (1 minute)",
                       "If any conflicting flags are specified, the latest one specified will take precedence"]
                      |options: StaticHuntsmanOptions, timeout: u64| {
                          let timeout = Duration::from_millis(timeout);
                          options.http_options.body_read_timeout = timeout;
                      }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "write-timeout" "TIMEOUT" "missing TIMEOUT for write-timeout"
                      ["Specify response write timeout to wait TIMEOUT milliseconds",
                       "Defaults to 60,000 milliseconds (1 minute)",
                       "If any conflicting flags are specified, the latest one specified will take precedence"]
                      |options: StaticHuntsmanOptions, timeout: u64| {
                          let timeout = Duration::from_millis(timeout);
                          options.http_options.write_timeout = timeout;
                      }
        ).group("HTTP FLAGS"),

        // Other Flags
        help_flag!("h", "help").group("OTHER FLAGS"),
        version_flag!(, "version" concat!("Huntsman Static Server v", env!("CARGO_PKG_VERSION"))).group("OTHER FLAGS"),
    ]
}

/// Parse the command line arguments into options
pub fn parse<'a>() -> Result<Option<StaticHuntsmanOptions>, argparse::Error<'a>> {
    PARSER.parse_env(StaticHuntsmanOptions::default())
}

impl Default for StaticHuntsmanOptions {
    fn default() -> Self {
        StaticHuntsmanOptions {
            base: "public/".into(),
            indexes: Vec::new(),
            bad_request: None,
            not_found: None,
            http_options: HTTPOptions::default(),
        }
    }
}
