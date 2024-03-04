use crate::{Options, Result};
use std::{cell::RefCell, num::NonZeroUsize, rc::Rc};

const MAX_EVENTS: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1024) };

/// Starts the huntsman server
pub fn run(options: Options) -> Result<()> {
    let result = Rc::new(RefCell::new(Ok(())));
    let child_result = result.clone();

    let future_queue = lasync::executor::FutureQueue::new();
    let child_future_queue = future_queue.clone();
    future_queue.push(async move {
        let inner_result = async_run(child_future_queue, options).await;
        *child_result.borrow_mut() = inner_result;
    });

    lasync::executor::run_queue(MAX_EVENTS, future_queue)?;

    Rc::try_unwrap(result).unwrap().into_inner()
}

/// Start of the asynchrounous runtime
pub(crate) async fn async_run(
    future_queue: lasync::executor::FutureQueue,
    options: Options,
) -> Result<()> {
    let listen_socket = lasync::futures::net::TCPListener::bind(options.socket_address())?;

    todo!("run()");
}
