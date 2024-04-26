use app::StaticHuntsman;

mod app;
mod args;
mod path;

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

    huntsman::run(
        StaticHuntsman::default(),
        huntsman::Options::default(),
        huntsman_http::HTTPOptions::default(),
    )
    .unwrap()
}
