//! A library for making servers

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(never_type)]

mod error;
mod protocol;
mod runner;

pub use error::{Error, Result};
pub use protocol::{Protocol, Request, Response, Transport, TransportClient};
pub use runner::{run, Options};
