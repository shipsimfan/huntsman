use crate::Result;
use std::{cell::RefCell, num::NonZeroUsize, rc::Rc};

const MAX_EVENTS: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1024) };

/// Starts the huntsman server
pub fn run() -> Result<()> {
    let result = Rc::new(RefCell::new(Ok(())));
    let child_result = result.clone();

    lasync::executor::run(MAX_EVENTS, async move {
        let inner_result = async_run().await;
        *child_result.borrow_mut() = inner_result;
    })?;

    Rc::try_unwrap(result).unwrap().into_inner()
}

/// Start of the asynchrounous runtime
pub(crate) async fn async_run() -> Result<()> {
    todo!("run()");
}
