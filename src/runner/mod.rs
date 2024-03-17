use crate::StartError;
use std::sync::Arc;
use worker::worker;
use worker_pool::WorkerPool;

mod options;
mod worker;
mod worker_pool;

pub use options::Options;

/// Run a huntsman server on the current thread
pub fn run<Protocol: crate::Protocol, App: crate::App<Protocol = Protocol>>(
    app: App,
    huntsman_options: Options,
    protocol_options: Protocol::Options,
) -> Result<!, StartError<Protocol>> {
    let app = Arc::new(app);

    let mut workers = WorkerPool::new(
        huntsman_options.max_connections().get(),
        huntsman_options.initial_workers().get(),
        huntsman_options.min_spare_workers(),
        huntsman_options.max_spare_workers(),
        &app,
    );

    let mut listener = Protocol::start(protocol_options).map_err(StartError)?;

    app.on_server_start(listener.get_addresses());

    loop {
        let client = match listener.accept() {
            Ok(client) => client,
            Err(error) => {
                app.accept_error(error);
                continue;
            }
        };

        workers.accept(client);
    }
}
