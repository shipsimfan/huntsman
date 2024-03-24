use std::{num::NonZeroUsize, sync::Arc};

pub(super) fn run<Protocol: crate::Protocol>(
    listeners: Arc<Vec<Protocol>>,
    max_connections: NonZeroUsize,
) -> ! {
    lasync::executor::run(super::NUM_EVENTS, async move {
        async_run(listeners, max_connections).await;
    })
    .unwrap();
    panic!("Executor returned in huntsman thread!");
}

pub(super) async fn async_run<Protocol: crate::Protocol>(
    listeners: Arc<Vec<Protocol>>,
    max_connections: NonZeroUsize,
) -> ! {
    loop {}
}
