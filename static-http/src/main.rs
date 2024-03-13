use huntsman_http::{HTTPRequest, HTTPResponse, HTTP};
use std::{convert::Infallible, net::SocketAddr, sync::Arc};

struct Static;

fn main() {
    huntsman::run(Static, huntsman::Options::default()).unwrap()
}

impl huntsman::App for Static {
    type Protocol = HTTP;

    type Client = SocketAddr;

    fn on_server_start(self: &Arc<Self>, address: SocketAddr) {
        println!("Server listening on {}", address);
    }

    fn handle_request<'a>(
        self: &Arc<Self>,
        client: &mut SocketAddr,
        request: HTTPRequest,
    ) -> HTTPResponse {
        todo!("handle_request()");
    }

    fn on_client_connect(self: &Arc<Self>, source: SocketAddr) -> Option<SocketAddr> {
        println!("Client connected from {}", source);
        Some(source)
    }

    fn on_client_disconnect(self: &Arc<Self>, client: &mut SocketAddr) {
        println!("{} disconnected", client);
    }

    fn accept_error(self: &Arc<Self>, error: std::io::Error) {
        eprintln!("An error occurred while accepting a client - {}", error);
    }

    fn parse_error(
        self: &Arc<Self>,
        client: &mut Self::Client,
        error: Infallible,
    ) -> Option<HTTPResponse> {
        eprintln!(
            "An error occurred while parsing a request from {} - {}",
            client, error
        );

        None
    }

    fn send_error(self: &Arc<Self>, client: &mut Self::Client, error: std::io::Error) {
        eprintln!(
            "An error occurred while sending a response to {} - {}",
            client, error
        );
    }
}
