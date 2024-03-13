use crate::{Error, Protocol, Result, Transport};
use std::{
    net::{SocketAddr, SocketAddrV4},
    sync::Arc,
};
use worker::worker;
use worker_pool::WorkerPool;

mod options;
mod worker;
mod worker_pool;

pub use options::Options;

/// Run a huntsman server on the current thread
pub fn run<App: crate::App>(app: App, options: Options) -> Result<!, App> {
    let app = Arc::new(app);

    let mut workers = WorkerPool::new(
        options.max_connections().get(),
        options.initial_workers().get(),
        options.min_spare_workers(),
        options.max_spare_workers(),
        &app,
    );

    let address = options.socket_address();
    let mut listener = <App::Protocol as Protocol>::Transport::bind(address)
        .map_err(|error| Error::ListenBindFailed((error, address)))?;

    app.on_server_start(
        listener
            .get_socket_address()
            .unwrap_or(SocketAddr::V4(SocketAddrV4::new(0.into(), 0))),
    );

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
