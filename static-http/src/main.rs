use app::StaticHuntsman;
use log::{ListenerDisplay, LoggerOutput, RequestDisplay};
use oak::LogController;
use path::parse_extension;
use std::{borrow::Cow, path::PathBuf};

mod app;
mod args;
mod error;
mod log;
mod path;

/// Attempts to read a file and parse it's extension
fn read_file(path: Option<PathBuf>, default: &'static [u8]) -> (Cow<'static, [u8]>, &'static [u8]) {
    let path = match path {
        Some(path) => path,
        None => return (Cow::Borrowed(default), b"text/html" as _),
    };

    let content = match std::fs::read(&path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error: Unable to read \"{}\" - {}", path.display(), error);
            std::process::exit(1);
        }
    };

    (Cow::Owned(content), parse_extension(path))
}

fn main() {
    let args = match args::parse() {
        Ok(args) => match args {
            Some(args) => args,
            None => return,
        },
        Err(error) => {
            eprintln!("Argument error: {}", error);
            std::process::exit(1);
        }
    };

    let bad_request = read_file(args.bad_request, include_bytes!("400.html"));
    let not_found = read_file(args.not_found, include_bytes!("404.html"));

    let log_outputs = match args
        .log_outputs
        .into_iter()
        .map(|output| output.try_into())
        .collect::<Result<_, _>>()
    {
        Ok(log_outputs) => log_outputs,
        Err(error) => {
            eprintln!("Unable to open log output - {}", error);
            std::process::exit(1);
        }
    };

    let log_controller = match LogController::new(
        "Static Huntsman",
        args.min_log_level,
        args.max_log_level,
        args.log_filter_type,
        args.log_filter,
        log_outputs,
    ) {
        Ok(controller) => controller,
        Err(error) => {
            eprintln!("Error: Failed to create logger - {}", error);
            std::process::exit(1);
        }
    };

    let app = StaticHuntsman::new(
        args.base,
        args.indexes,
        bad_request,
        not_found,
        log_controller,
        args.log_headers,
        args.log_bodies,
        args.log_reponses,
    );

    huntsman::run(app, args.huntsman_options, args.http_options).unwrap()
}
