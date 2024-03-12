//! A library for making servers

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(never_type)]

mod error;
mod runner;

pub use error::{Error, Result};
pub use runner::{run, Options};

#[test]
fn test() {
    match run(Options::default()) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error: {}", error);
            Result::<()>::Err(error).unwrap();
        }
    }
}
