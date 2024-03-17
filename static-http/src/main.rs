use huntsman_http::{
    HTTPParseError, HTTPRequest, HTTPResponse, HTTPResponseField, HTTPStatus, HTTP,
};
use std::{net::SocketAddr, sync::Arc};

struct Static;

const NOT_FOUND: &[u8] = include_bytes!("../404.html");

fn main() {
    huntsman::run(
        Static,
        huntsman::Options::default(),
        huntsman_http::HTTPOptions::default(),
    )
    .unwrap()
}

impl huntsman::App for Static {
    type Protocol = HTTP;

    type Client = SocketAddr;

    fn on_server_start(self: &Arc<Self>, address: &[SocketAddr]) {
        println!("Server listening on {}", address[0]);
    }

    fn handle_request<'a>(
        self: &Arc<Self>,
        client: &mut SocketAddr,
        request: HTTPRequest,
    ) -> HTTPResponse {
        println!(
            "{} request for {} from {}",
            request.method(),
            request.target(),
            client
        );
        for field in request.fields() {
            println!("  {}", field);
        }
        if request.body().len() > 0 {
            println!("{}", String::from_utf8_lossy(request.body()));
        }
        println!();

        let mut response = HTTPResponse::new(HTTPStatus::NotFound, NOT_FOUND);

        response.push_field(HTTPResponseField::new(
            "Content-Type".as_bytes(),
            "text/html; charset=utf-8".as_bytes(),
        ));

        response
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

    fn read_error(
        self: &Arc<Self>,
        client: &mut Self::Client,
        error: HTTPParseError,
    ) -> Option<HTTPResponse> {
        eprintln!(
            "An error occurred while parsing a request from {} - {}",
            client, error
        );

        Some(match error {
            HTTPParseError::HeadersTooLong => HTTPStatus::ContentTooLarge.into(),
            _ => HTTPStatus::BadRequest.into(),
        })
    }

    fn send_error(self: &Arc<Self>, client: &mut Self::Client, error: std::io::Error) {
        eprintln!(
            "An error occurred while sending a response to {} - {}",
            client, error
        );
    }
}
