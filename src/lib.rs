//! A library for making servers

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(never_type)]
#![feature(associated_type_defaults)]

mod app;
mod error;
mod protocol;
mod runner;

pub use app::App;
pub use error::{Error, Result};
pub use protocol::{Protocol, RequestParser, Response, Transport, TransportClient};
pub use runner::{run, Options};
