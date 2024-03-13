use crate::TransportClient;

/// A response sent to a client
pub trait Response {
    /// The client transport to send on
    type TransportClient: TransportClient;

    /// Send this response on `transport`
    fn send(
        self,
        transport: &mut Self::TransportClient,
    ) -> Result<(), <Self::TransportClient as TransportClient>::Error>;
}
