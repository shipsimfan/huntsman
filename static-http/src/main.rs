use huntsman::Protocol;
use huntsman_http::{HTTPParseError, HTTPResponse, HTTPStatus, HTTPTarget, ListenAddress, HTTP};
use std::{
    ffi::{OsStr, OsString},
    future::Future,
    net::SocketAddr,
    os::unix::ffi::OsStringExt,
    path::{Path, PathBuf},
    sync::Arc,
};

/// An HTTP app which serves static files from a path
struct Static {
    /// The path to server static files from
    base: PathBuf,

    /// The body to respond with when a bad request is submitted
    bad_request: Vec<u8>,

    /// The body to respond with when a file cannot be found for a request
    not_found: Vec<u8>,
}

fn main() {
    huntsman::run(
        Static::default(),
        huntsman::Options::default(),
        huntsman_http::HTTPOptions::default(),
    )
    .unwrap()
}

/// Parses a `url` into an acceptable path under `base`
///
/// If there is an error with the `url`, [`None`] is returned and a "400 Bad Request" response
/// should be given to the client.
fn parse_path(url: HTTPTarget, base: &Path) -> Option<PathBuf> {
    println!("URL: {}", url);

    // The capacity will either be:
    //  - one greater (from a trailing slash) if no "%XX" url segments exits, or
    //  - it will shrink because "%XX" will take three bytes and convert them to one.
    let mut path = Vec::with_capacity(base.as_os_str().len() + url.len() + 1);
    path.extend_from_slice(base.as_os_str().as_encoded_bytes());

    let base_length = path.len();

    // Verify the ending of the path is a '/'
    match path.last() {
        Some(last) => {
            if *last != b'/' {
                path.push(b'/');
            }
        }
        // Make sure it starts from this directory, not the root of the filesystem
        None => path.extend_from_slice(b"./"),
    }

    // The indices of the path segments
    let mut segments = Vec::new();
    let mut url = url.into_iter().map(|c| *c).peekable();

    while let Some(c) = url.next() {
        match c {
            b'/' => {}
            b'?' => break, // Reached the GET parameters
            _ => return None,
        }

        let segment_index = path.len();

        while let Some(&c) = url.peek() {
            match c {
                b'/' | b'?' => break,
                b'%' => todo!(),
                _ => {
                    path.push(c);
                    url.next();
                }
            }
        }

        if path.len() == segment_index {
            continue;
        }

        let segment = &path[segment_index..];
        if segment == b".." {
            if let Some(last_index) = segments.pop() {
                path.truncate(last_index);
            } else {
                path.truncate(base_length);
            }
            continue;
        }

        segments.push(segment_index);
        path.push(b'/');
    }

    // Remove the trailing "/"
    path.pop();
    Some(OsString::from_vec(path).into())
}

fn not_found(body: &[u8]) -> HTTPResponse {
    let mut response = HTTPResponse::new(HTTPStatus::NotFound, &*body);

    response.push_field(
        "Content-Type".as_bytes(),
        "text/html; charset=utf-8".as_bytes(),
    );

    response
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

    fn handle_request<'a, 'b>(
        self: &'a Arc<Self>,
        client: &'a mut Self::Client,
        request: <Self::Protocol as Protocol>::Request<'b>,
    ) -> impl Future<Output = <Self::Protocol as Protocol>::Response<'a>> {
        async move {
            let path = match parse_path(request.target(), &self.base) {
                Some(path) => path,
                None => {
                    println!(
                        "Bad request target \"{}\" from {}",
                        request.target(),
                        client
                    );
                    return HTTPResponse::new(HTTPStatus::BadRequest, &self.bad_request);
                }
            };

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
            println!("Sending {}", path.display());
            println!();

            let body = match lasync::fs::read(&path).await {
                Ok(body) => body,
                Err(_) => return not_found(&self.not_found),
            };

            let mut response = HTTPResponse::new(HTTPStatus::OK, body);
            response.push_field(
                b"Content-Type",
                match path
                    .extension()
                    .unwrap_or(OsStr::new(""))
                    .as_encoded_bytes()
                {
                    b"css" => b"text/css",
                    b"htm" | b"html" => b"text/html",
                    b"js" | b"mjs" => b"text/javascript",
                    b"txt" => b"text/plain",
                    _ => b"application/octet-stream",
                },
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

    fn read_error<'a>(
        self: &'a Arc<Self>,
        client: &'a mut Self::Client,
        error: HTTPParseError,
    ) -> impl Future<Output = Option<HTTPResponse<'a>>> {
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

impl Default for Static {
    fn default() -> Self {
        Static {
            base: "public/".into(),
            not_found: include_bytes!("404.html").to_vec(),
            bad_request: include_bytes!("400.html").to_vec(),
        }
    }
}
