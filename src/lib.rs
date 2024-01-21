//! A library for making servers

#![deny(missing_docs)]
#![feature(never_type)]

mod error;
mod run;
mod server;

pub use error::{LinuxError, LinuxResult};
pub use run::run;

pub(crate) use error::try_linux;
pub(crate) use server::Server;
