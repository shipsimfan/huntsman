use crate::{LinuxResult, Server};

/// Creates an new [`Server`] and runs it
pub fn run(port: u16) -> LinuxResult<!> {
    let mut server = Server::new(port)?;

    loop {}
}
