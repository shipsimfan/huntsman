use crate::LoggerOutput;
use argparse::{config_flag, help_flag, parser, parsing_flag, simple_flag, version_flag};
use huntsman_http::{HTTPOptions, HTTP};
use oak::{FilterListType, LogLevel};
use std::{net::SocketAddr, num::NonZeroUsize, path::PathBuf, time::Duration};

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

    /* Huntsman Flags */
    /// The huntsman options
    pub huntsman_options: huntsman::Options<HTTP>,

    /* HTTP Flags */
    /// The HTTP options
    pub http_options: HTTPOptions,

    /* Logging Flags */
    /// Should request headers be logged?
    pub log_headers: bool,

    /// Should request bodies be logged?
    pub log_bodies: bool,

    /// Should response codes and paths be logged?
    pub log_reponses: bool,

    /// The minimum severity to log
    pub min_log_level: LogLevel,

    /// The maximum severity to log
    pub max_log_level: Option<LogLevel>,

    /// The scope filter list type
    pub log_filter_type: FilterListType,

    /// The scope filter list
    pub log_filter: Vec<String>,

    /// The outputs for logging
    pub log_outputs: Vec<LoggerOutput>,
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
        parsing_flag!("400", "bad-request" "FILE" "missing FILE for bad-request"
                      "Specify the file to serve when a request is bad"
                      |options: StaticHuntsmanOptions, path: PathBuf| { options.bad_request = Some(path); }
        ).group("SERVING FLAGS"),
        parsing_flag!("404", "not-found" "FILE" "missing FILE for not-found"
                      "Specify the file to serve when a requested file cannot be found"
                      |options: StaticHuntsmanOptions, path: PathBuf| { options.not_found = Some(path); }
        ).group("SERVING FLAGS"),

        // Huntsman flags
        parsing_flag!(, "workers" "COUNT" "missing COUNT for workers"
                      ["Specify the number of worker threads",
                       "Defaults to a system provided value, usually the number of CPUs"]
                      |options: StaticHuntsmanOptions, count: NonZeroUsize| { options.huntsman_options.set_workers(count); }
        ).group("HUNTSMAN FLAGS"),
        parsing_flag!(, "worker-connections" "COUNT" "missing COUNT for worker-connections"
                      ["Specify the maximum number of connections per worker",
                       "Defaults to 64"]
                      |options: StaticHuntsmanOptions, count: NonZeroUsize| { options.huntsman_options.set_connections_per_worker(count); }
        ).group("HUNTSMAN FLAGS"),
        parsing_flag!(, "http" "ADDRESS:PORT" "missing ADDRESS for http"
                      "Specify the address to listen for insecure HTTP/1.1 connections on"
                      |options: StaticHuntsmanOptions, address: SocketAddr| { options.huntsman_options.address_mut().http = Some(address); }
        ).group("HUNTSMAN FLAGS"),

        // HTTP Flags
        parsing_flag!(, "max-header-size" "SIZE" "missing size for max-header-size"
                      ["Specify the maximum size of request headers in bytes to accpet",
                       "Defaults to 8,192 bytes (8 Kb)"]
                      |options: StaticHuntsmanOptions, size: usize| { options.http_options.max_header_size = size; }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "max-body-size" "SIZE" "missing SIZE for max-body-size"
                      ["Specify the maximum size of request bodies in bytes to accept",
                       "Defaults to 1,048,576 bytes (1 Mb)"]
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

        // Logging Flags
        simple_flag!(, "log-headers"
                     "Enable logging request headers"
                     |options: StaticHuntsmanOptions, _| { options.log_headers = true; }
        ).group("LOGGING FLAGS"),
        simple_flag!(, "log-bodies"
                     "Enable logging request bodies"
                     |options: StaticHuntsmanOptions, _| { options.log_bodies = true; }
        ).group("LOGGING FLAGS"),
        simple_flag!(, "log-responses"
                     "Enable logging responses statuses and paths"
                     |options: StaticHuntsmanOptions, _| { options.log_reponses = true; }
        ).group("LOGGING FLAGS"),
        parsing_flag!(, "min-log-level" "LEVEL" "missing LEVEL for min-log-level"
                      ["Sets the minimum severity of messages to log",
                       "LEVEL can be \"trace\", \"debug\", \"info\", \"warn\", \"err\", or \"fatal\"",
                       "Defaults to \"info\""]
                      |options: StaticHuntsmanOptions, level: LogLevel| { options.min_log_level = level; }
        ).group("LOGGING FLAGS"),
        parsing_flag!(, "max-log-level" "LEVEL" "missing LEVEL for max-log-level"
                      ["Sets the maximum severity of messages to log",
                       "LEVEL can be \"trace\", \"debug\", \"info\", \"warn\", \"err\", or \"fatal\"",
                       "Defaults to allowing all log messages"]
                      |options: StaticHuntsmanOptions, level: LogLevel| { options.max_log_level = Some(level); }
        ).group("LOGGING FLAGS"),
        parsing_flag!(, "log-filter-type" "TYPE" "missing TYPE for log-filter-type"
                      ["Sets the log filter type",
                       "TYPE can be \"blacklist\" or \"whitelist\"",
                       "Defaults to \"blacklist\""]
                      |options: StaticHuntsmanOptions, filter_type: FilterListType| { options.log_filter_type = filter_type; }
        ),
        parsing_flag!(, "log-filter" "SCOPE" "missing SCOPE for log-filter"
                     "Add SCOPE to the log filter list"
                     |options: StaticHuntsmanOptions, scope: String| { options.log_filter.push(scope); }
        ).group("LOGGING FLAGS").repeatable(true),
        parsing_flag!(, "log-output" "OUTPUT" "missing OUTPUT for log-output"
                      ["Add OUTPUT as a log output",
                       "Can be set to \"stdout\", \"stderr\", or a path"]
                      |options: StaticHuntsmanOptions, output: LoggerOutput| { options.log_outputs.push(output); }
        ).group("LOGGING FLAGS").repeatable(true),

        // Other Flags
        config_flag!(, "config").group("OTHER FLAGS"),
        help_flag!("h", "help").group("OTHER FLAGS"),
        version_flag!(, "version" concat!("Huntsman Static Server v", env!("CARGO_PKG_VERSION"))).group("OTHER FLAGS"),
    ]
}

/// Parse the command line arguments into options
pub fn parse<'a>() -> Result<Option<StaticHuntsmanOptions>, argparse::Error<'a>> {
    PARSER
        .usage("USAGE:\n    %0 [OPTIONS]...")
        .parse_env(StaticHuntsmanOptions::default())
}

impl Default for StaticHuntsmanOptions {
    fn default() -> Self {
        StaticHuntsmanOptions {
            base: "public/".into(),
            indexes: Vec::new(),
            bad_request: None,
            not_found: None,
            huntsman_options: huntsman::Options::default(),
            http_options: HTTPOptions::default(),
            log_headers: false,
            log_bodies: false,
            log_reponses: false,
            min_log_level: LogLevel::Info,
            max_log_level: None,
            log_filter_type: FilterListType::Blacklist,
            log_filter: Vec::new(),
            log_outputs: Vec::new(),
        }
    }
}
