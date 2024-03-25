use crate::StartError;
use std::{num::NonZeroUsize, sync::Arc};

mod options;
mod worker;

pub use options::Options;

const NUM_EVENTS: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(8192) };

/// Run a huntsman server on the current thread
pub fn run<Protocol: crate::Protocol, App: crate::App<Protocol = Protocol>>(
    app: App,
    huntsman_options: Options<Protocol>,
    protocol_options: Protocol::Options,
) -> Result<(), StartError<Protocol>> {
    let mut result = Ok(());

    lasync::executor::run(NUM_EVENTS, async {
        result = async_run(app, huntsman_options, protocol_options).await;
    })?;

    result
}

pub async fn async_run<Protocol: crate::Protocol, App: crate::App<Protocol = Protocol>>(
    app: App,
    huntsman_options: Options<Protocol>,
    protocol_options: Protocol::Options,
) -> Result<(), StartError<Protocol>> {
    // Create the listener
    let mut listener = Protocol::start(huntsman_options.address(), protocol_options)
        .await
        .map_err(StartError::Protocol)?;
    let address = listener.address().await;

    // Prepare shared values
    let app = Arc::new(app);
    let listener = Arc::new(listener);
    let connections_per_worker = huntsman_options.connections_per_worker();

    // Signal the server start
    app.on_server_start(address).await;

    // Create workers
    for i in 0..huntsman_options.workers().get() - 1 {
        let child_listener = listener.clone();
        let child_app = app.clone();

        std::thread::Builder::new()
            .name(format!("worker {}", i + 1))
            .spawn(move || worker::run(child_app, child_listener, connections_per_worker))?;
    }

    worker::async_run(app, listener, connections_per_worker).await
}
