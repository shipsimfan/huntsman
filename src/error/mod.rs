mod linux;

pub use linux::{LinuxError, LinuxResult};

pub(crate) use linux::try_linux;
