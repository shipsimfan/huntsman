use lasync::futures::sync::Notify;
use std::{cell::RefCell, num::NonZeroUsize, rc::Rc, sync::Arc};

/// Records the current connections on a worker
struct Connections {
    /// The maximum number of connections this worker can handle
    max_connections: usize,

    /// The current number of connections being handled
    count: RefCell<usize>,

    /// The [`Notify`] to signal when a client disconnects
    notify: Notify,
}

pub(super) fn run<Protocol: crate::Protocol, App: crate::App<Protocol = Protocol>>(
    app: Arc<App>,
    listener: Arc<Protocol>,
    max_connections: NonZeroUsize,
) -> ! {
    lasync::executor::run(super::NUM_EVENTS, async move {
        async_run(app, listener, max_connections).await;
    })
    .unwrap();
    panic!("Executor returned in huntsman thread!");
}

pub(super) async fn async_run<Protocol: crate::Protocol, App: crate::App<Protocol = Protocol>>(
    app: Arc<App>,
    listener: Arc<Protocol>,
    max_connections: NonZeroUsize,
) -> ! {
    let connections = Rc::new(Connections::new(max_connections));

    loop {
        connections.wait_until_available().await;

        let (client_socket, address) = match listener.accept().await {
            Ok(client) => client,
            Err(error) => {
                app.accept_error(error).await;
                continue;
            }
        };

        let client = match app.on_client_connect(address).await {
            Some(client) => client,
            None => continue,
        };

        connections.new_connection();

        todo!("Spawn task to handle the connection");
    }
}

impl Connections {
    /// Creates a new [`Connections`] tracker with a count of 0
    pub(self) fn new(max_connections: NonZeroUsize) -> Self {
        Connections {
            max_connections: max_connections.get(),
            count: RefCell::new(0),
            notify: Notify::new(),
        }
    }

    /// Waits until a connection becomes available
    pub(self) async fn wait_until_available(&self) {
        while *self.count.borrow() >= self.max_connections {
            self.notify.notified().await;
        }
    }

    /// Signals a new connection has started
    pub(self) fn new_connection(&self) {
        let mut count = self.count.borrow_mut();
        assert!(*count < self.max_connections);
        *count += 1;
    }
}
