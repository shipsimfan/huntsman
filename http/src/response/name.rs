#[cfg(windows)]
macro_rules! os {
    () => {
        " (Windows)"
    };
}

#[cfg(unix)]
macro_rules! os {
    () => {
        " (Unix)"
    };
}

#[cfg(not(any(windows, unix)))]
macro_rules! os {
    () => {
        ""
    };
}

/// The value placed into the "Server" field
pub(super) const SERVER: &str = concat!(
    "Server: ",
    "Huntsman/",
    env!("CARGO_PKG_VERSION"),
    os!(),
    "\r\n"
);
