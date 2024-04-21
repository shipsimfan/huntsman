use app::Static;

mod app;
mod path;

fn main() {
    huntsman::run(
        Static::default(),
        huntsman::Options::default(),
        huntsman_http::HTTPOptions::default(),
    )
    .unwrap()
}
