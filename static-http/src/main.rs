use huntsman::Protocol;
use huntsman_http::{HTTPParseError, HTTPResponse, HTTPStatus, ListenAddress, HTTP};
use std::{future::Future, net::SocketAddr, sync::Arc};

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

    fn on_server_start(self: &Arc<Self>, address: ListenAddress) -> impl Future<Output = ()> {
        async move {
            println!("Server listening on:");

            if let Some(http) = &address.http {
                println!("  {}", http);
            }
        }
    }

    fn handle_request<'a>(
        self: &Arc<Self>,
        client: &mut Self::Client,
        request: <Self::Protocol as Protocol>::Request<'a>,
    ) -> impl Future<Output = <Self::Protocol as Protocol>::Response> {
        async move {
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

            response.push_field(
                "Content-Type".as_bytes(),
                "text/html; charset=utf-8".as_bytes(),
            );

            response
        }
    }

    fn on_client_connect(
        self: &Arc<Self>,
        source: SocketAddr,
    ) -> impl Future<Output = Option<SocketAddr>> {
        async move {
            println!("Client connected from {}", source);
            Some(source)
        }
    }

    fn on_client_disconnect(self: &Arc<Self>, client: &mut SocketAddr) -> impl Future<Output = ()> {
        async move { println!("{} disconnected", client) }
    }

    fn accept_error(self: &Arc<Self>, error: huntsman_http::Error) -> impl Future<Output = ()> {
        async move { eprintln!("An error occurred while accepting a client - {}", error) }
    }

    fn read_error(
        self: &Arc<Self>,
        client: &mut Self::Client,
        error: HTTPParseError,
    ) -> impl Future<Output = Option<HTTPResponse>> {
        async move {
            eprintln!(
                "An error occurred while parsing a request from {} - {}",
                client, error
            );

            Some(match error {
                HTTPParseError::HeadersTooLong => HTTPStatus::ContentTooLarge.into(),
                _ => HTTPStatus::BadRequest.into(),
            })
        }
    }

    fn send_error(
        self: &Arc<Self>,
        client: &mut Self::Client,
        error: huntsman_http::Error,
    ) -> impl Future<Output = ()> {
        async move {
            eprintln!(
                "An error occurred while sending a response to {} - {}",
                client, error
            );
        }
    }
}
