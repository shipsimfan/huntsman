use client::handle_client;
use connections::Connections;
use lasync::FutureQueue;
use std::{num::NonZeroUsize, sync::Arc};

mod accept;
mod client;
mod connections;

pub(super) use accept::accept_clients;

pub(super) fn run<Protocol: crate::Protocol, App: crate::App<Protocol = Protocol>>(
    app: Arc<App>,
    listener: Arc<Protocol>,
    max_connections: NonZeroUsize,
) -> ! {
    let future_queue = FutureQueue::new();
    accept_clients(app, listener, max_connections, &future_queue);

    lasync::run_queue(super::NUM_EVENTS, future_queue).unwrap();
    panic!("Executor returned in huntsman thread!");
}
