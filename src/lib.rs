//! A library for making servers

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod error;
mod options;
mod runner;

pub use error::{Error, Result};
pub use options::Options;
pub use runner::run;
