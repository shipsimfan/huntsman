use lasync::futures::sync::Notify;
use std::{cell::RefCell, num::NonZeroUsize, rc::Rc};

/// Records the current connections on a worker
pub(super) struct Connections {
    /// The maximum number of connections this worker can handle
    max_connections: usize,

    /// The current number of connections being handled
    count: RefCell<usize>,

    /// The [`Notify`] to signal when a client disconnects
    notify: Notify,
}

impl Connections {
    /// Creates a new [`Connections`] tracker with a count of 0
    pub(super) fn new(max_connections: NonZeroUsize) -> Rc<Self> {
        Rc::new(Connections {
            max_connections: max_connections.get(),
            count: RefCell::new(0),
            notify: Notify::new(),
        })
    }

    /// Waits until a connection becomes available
    pub(super) async fn wait_until_available(&self) {
        while *self.count.borrow() >= self.max_connections {
            self.notify.notified().await;
        }
    }

    /// Signals a new connection has started
    pub(super) fn new_connection(&self) {
        let mut count = self.count.borrow_mut();
        assert!(*count < self.max_connections);
        *count += 1;
    }

    /// Signals a connection has ended
    pub(super) fn end_connection(&self) {
        let mut count = self.count.borrow_mut();
        *count -= 1;
        self.notify.notify_all();
    }
}
