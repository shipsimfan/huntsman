use crate::{Protocol, RequestParser, Transport};
use std::sync::Arc;

/// A huntsman application
pub trait App {
    /// The protocol this app runs on
    type Protocol: Protocol;

    /// Handle a request from a client
    fn handle_request<'a>(
        self: Arc<Self>,
        request: <<Self::Protocol as Protocol>::RequestParser as RequestParser>::Request<'a>,
    ) -> <Self::Protocol as Protocol>::Response;

    /// An error occurred while accepting a client
    fn accept_error(
        self: Arc<Self>,
        error: <<Self::Protocol as Protocol>::Transport as Transport>::Error,
    );

    /// An error occurred while parsing a request from a client
    fn parse_error(
        self: Arc<Self>,
        error: <<Self::Protocol as Protocol>::RequestParser as RequestParser>::Error,
    ) -> Option<<Self::Protocol as Protocol>::Response>;

    /// An error occurred while sending the response
    fn send_error(
        self: Arc<Self>,
        error: <<Self::Protocol as Protocol>::Transport as Transport>::Error,
    );
}
