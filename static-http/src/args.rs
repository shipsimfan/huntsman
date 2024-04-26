use argparse::{help_flag, parser, parsing_flag, version_flag};
use std::path::PathBuf;

/// Options that control how the server will run
pub struct StaticHuntsmanOptions {
    /* Serving Options */
    /// The directory to serve files from
    pub base: PathBuf,

    /// The files in a directory to serve when the request path is a directory
    pub indexes: Vec<PathBuf>,

    /// The file to serve if a request is bad
    pub bad_request: Option<PathBuf>,

    /// The file to serve if a requested path isn't found
    pub not_found: Option<PathBuf>,
    /* HTTP Options */

    /* Huntsman Options */
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
        }
    }
}
