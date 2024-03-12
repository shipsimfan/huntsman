use crate::{Error, Result, Transport};
use worker::worker;

mod options;
mod worker;
mod worker_pool;

pub use options::Options;

use self::worker_pool::WorkerPool;

/// Run a huntsman server on the current thread
pub fn run<Protocol: crate::Protocol>(options: Options) -> Result<!, Protocol> {
    let mut workers = WorkerPool::<Protocol>::new(
        options.max_connections().get(),
        options.initial_workers().get(),
        options.min_spare_workers(),
        options.max_spare_workers(),
    );

    let address = options.socket_address();
    let mut listener = Protocol::Transport::bind(address)
        .map_err(|error| Error::ListenBindFailed((error, address)))?;

    loop {
        let client = match listener.accept() {
            Ok(client) => client,
            Err(error) => {
                eprintln!("Error while accepting client - {}", error);
                continue;
            }
        };

        workers.accept(client);
    }
}
