use crate::{Error, Protocol, Result, Transport};
use worker::worker;
use worker_pool::WorkerPool;

mod options;
mod worker;
mod worker_pool;

pub use options::Options;

/// Run a huntsman server on the current thread
pub fn run<App: crate::App>(options: Options) -> Result<!, App> {
    let mut workers = WorkerPool::<App>::new(
        options.max_connections().get(),
        options.initial_workers().get(),
        options.min_spare_workers(),
        options.max_spare_workers(),
    );

    let address = options.socket_address();
    let mut listener = <App::Protocol as Protocol>::Transport::bind(address)
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
