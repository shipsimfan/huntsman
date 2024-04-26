use argparse::parser;

pub struct StaticHuntsmanOptions {}

parser! {
    PARSER -> StaticHuntsmanOptions
    "Huntsman Static Server"
    "A server built on huntsman which serves static files",
}

/// Parse the command line arguments into options
pub fn parse<'a>() -> Result<Option<StaticHuntsmanOptions>, argparse::Error<'a>> {
    PARSER.parse_env(StaticHuntsmanOptions::default())
}

impl Default for StaticHuntsmanOptions {
    fn default() -> Self {
        StaticHuntsmanOptions {}
    }
}
