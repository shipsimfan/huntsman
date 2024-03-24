use crate::StartError;
use std::{num::NonZeroUsize, sync::Arc};

mod options;

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
    mut huntsman_options: Options<Protocol>,
    protocol_options: Protocol::Options,
) -> Result<(), StartError<Protocol>> {
    // Prepare addresses and app
    let addresses = huntsman_options.addresses();
    let app = Arc::new(app);

    // Create the listeners
    let mut listeners = Vec::with_capacity(addresses.len());
    let mut real_addresses = Vec::with_capacity(addresses.len());
    for address in addresses {
        let mut listener = Protocol::start(address, &protocol_options)
            .await
            .map_err(StartError::Protocol)?;

        real_addresses.push(listener.address().await);
        listeners.push(listener);
    }

    // Signal the server start
    app.on_server_start(&real_addresses).await;

    todo!("Create workers");
}
