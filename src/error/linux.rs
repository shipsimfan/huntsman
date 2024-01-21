use linux::{errno::errno, string::strerror_r};
use std::ffi::{c_int, CStr};

/// A specialized result for Linux errors
pub type LinuxResult<T> = Result<T, LinuxError>;

/// An error reported by Linux
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinuxError(c_int);

/// Convert a linux system call result (-1 on error) into a [`LinuxResult<c_int>`]
macro_rules! try_linux {
    ($expr: expr) => {
        match unsafe { $expr } {
            -1 => Err($crate::LinuxError::errno()),
            result => Ok(result),
        }
    };
}

impl LinuxError {
    /// Creates a [`LinuxError`] from the current value of [`errno`]
    pub(crate) fn errno() -> Self {
        LinuxError(*unsafe { errno() })
    }
}

impl std::error::Error for LinuxError {}

impl std::fmt::Display for LinuxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = [0; 64];

        let message =
            unsafe { CStr::from_ptr(strerror_r(self.0, buffer.as_mut_ptr(), buffer.len())) };

        write!(f, "{}", message.to_string_lossy())
    }
}

pub(crate) use try_linux;
