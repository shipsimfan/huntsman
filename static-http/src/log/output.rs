use oak::{FileLogOutput, LogOutput, ReadableLogFormatter, StderrLogOutput, StdoutLogOutput};
use std::{convert::Infallible, path::PathBuf, str::FromStr};

/// An output for logs
pub enum LoggerOutput {
    /// Standard out
    Stdout,

    /// Standard error
    Stderr,

    /// A file at the path
    File(PathBuf),
}

impl FromStr for LoggerOutput {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "stderr" => LoggerOutput::Stderr,
            "stdout" => LoggerOutput::Stdout,
            _ => LoggerOutput::File(s.into()),
        })
    }
}

impl TryInto<Box<dyn LogOutput>> for LoggerOutput {
    type Error = std::io::Error;

    fn try_into(self) -> std::io::Result<Box<dyn LogOutput>> {
        match self {
            LoggerOutput::Stdout => Ok(StdoutLogOutput::default()),
            LoggerOutput::Stderr => Ok(StderrLogOutput::default()),
            LoggerOutput::File(path) => {
                FileLogOutput::open(path, ReadableLogFormatter::new(), "file")
            }
        }
    }
}
