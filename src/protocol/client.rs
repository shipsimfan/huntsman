use std::{error::Error, future::Future};

/// A client connection
pub trait ProtocolClient: 'static + Sized + Send {
    /// An error while reading and parsing
    type ReadError: 'static + Error;

    /// An error while sending
    type SendError: 'static + Error;

    /// A request sent from a client
    type Request<'a>: Sized;

    /// A response sent to a client
    type Response<'a>;

    /// Attempt to read and parse the next request from the client
    fn read<'a>(
        &'a mut self,
    ) -> impl Future<Output = Result<Option<Self::Request<'a>>, Self::ReadError>>;

    /// Send this response on `transport`
    fn send<'a>(
        &mut self,
        response: Self::Response<'a>,
    ) -> impl Future<Output = Result<(), Self::SendError>>;
}
