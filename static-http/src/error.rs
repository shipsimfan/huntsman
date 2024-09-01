use crate::HTTPResponse;
use huntsman_http::HTTPClientAddress;
use std::{ffi::OsString, fmt::Debug};

/// The kind of error that occurred
#[derive(Debug)]
enum HandleErrorKind {
    /// The path in the request was not valid
    BadPath,

    /// The file was not found or cannot be accessed
    NotFoundOrUnreadable(OsString),
}

/// An error that can occur during the handling of a client
pub struct HandleError<'a> {
    /// The response to give to the client
    response: HTTPResponse<'a>,

    /// The client that caused the error
    client: HTTPClientAddress,

    /// The kind of the error
    kind: HandleErrorKind,
}

impl<'a> HandleError<'a> {
    /// Creates a [`HandleError`] for when path parsing fails
    pub fn bad_path(response: HTTPResponse<'a>, client: HTTPClientAddress) -> Self {
        HandleError {
            response,
            client,
            kind: HandleErrorKind::BadPath,
        }
    }

    /// Creates a [`HandleError`] when the requested file cannot be read
    pub fn not_found_or_unreadable(
        response: HTTPResponse<'a>,
        path: OsString,
        client: HTTPClientAddress,
    ) -> Self {
        HandleError {
            response,
            client,
            kind: HandleErrorKind::NotFoundOrUnreadable(path),
        }
    }

    /// Gets the response for this error
    pub fn response(&self) -> &HTTPResponse<'a> {
        &self.response
    }

    /// Unwraps the response for this error
    pub fn unwrap_response(self) -> HTTPResponse<'a> {
        self.response
    }
}

impl<'a> std::error::Error for HandleError<'a> {}

impl<'a> std::fmt::Display for HandleError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            HandleErrorKind::BadPath => write!(f, "Bad path received from {}", self.client),
            HandleErrorKind::NotFoundOrUnreadable(path) => write!(
                f,
                "{:?} not found or not readable for {}",
                path, self.client
            ),
        }
    }
}

impl<'a> std::fmt::Debug for HandleError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
