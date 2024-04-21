use huntsman::{App, Protocol};
use huntsman_http::{
    HTTPParseError, HTTPRequest, HTTPResponse, HTTPStatus, HTTPTarget, ListenAddress, HTTP,
};
use std::{
    borrow::Cow,
    ffi::OsStr,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};

/// An HTTP app which serves static files from a path
pub struct Static {
    /// The path to server static files from
    base: PathBuf,

    /// The body to respond with when a bad request is submitted
    bad_request: (Cow<'static, [u8]>, &'static [u8]),

    /// The body to respond with when a file cannot be found for a request
    not_found: (Cow<'static, [u8]>, &'static [u8]),
}

/// Displays the target and fields of a request
fn display_request(request: &HTTPRequest, client: &SocketAddr) {
    println!();
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
}

/// Attempts to read the file at `path`
async fn read_file(path: &Path) -> Option<Vec<u8>> {
    lasync::fs::read(&path).await.ok()
}

/// Parses the extension of `path` and guesses the MIME type of its contents
fn parse_extension(path: &Path) -> &'static [u8] {
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
    }
}

impl Static {
    /// Creates a new [`Static`] http serving app
    pub fn new<S1: Into<Cow<'static, [u8]>>, S2: Into<Cow<'static, [u8]>>>(
        base: PathBuf,
        bad_request: (S1, &'static [u8]),
        not_found: (S2, &'static [u8]),
    ) -> Self {
        Static {
            base,
            bad_request: (bad_request.0.into(), bad_request.1),
            not_found: (not_found.0.into(), not_found.1),
        }
    }

    /// Attempts to parse the target into a path or returns a "400 Bad Request" response
    fn parse_path<'a>(&'a self, target: HTTPTarget) -> Result<PathBuf, HTTPResponse<'a>> {
        crate::path::parse(target, &self.base, 0).ok_or_else(|| {
            (
                HTTPStatus::BadRequest,
                self.bad_request.0.as_ref(),
                self.bad_request.1,
            )
                .into()
        })
    }

    /// Attempts to read the file at `path`
    async fn read_file<'a>(&'a self, path: &Path) -> Result<Vec<u8>, HTTPResponse<'a>> {
        read_file(path).await.ok_or_else(|| {
            (
                HTTPStatus::NotFound,
                self.not_found.0.as_ref(),
                self.not_found.1,
            )
                .into()
        })
    }
}

impl App for Static {
    type Protocol = HTTP;

    type Client = SocketAddr;

    async fn on_server_start(self: &Arc<Self>, address: ListenAddress) {
        println!("Server listening on:");

        if let Some(http) = &address.http {
            println!("  {}", http);
        }
    }

    async fn handle_request<'a, 'b>(
        self: &'a Arc<Self>,
        client: &'a mut Self::Client,
        request: <Self::Protocol as Protocol>::Request<'b>,
    ) -> HTTPResponse<'a> {
        display_request(&request, client);

        let path = match self.parse_path(request.target()) {
            Ok(path) => path,
            Err(response) => {
                eprintln!("Error: Bad path received from {}", client);
                return response;
            }
        };

        println!("Sending {} to {}", path.display(), client);

        let body = match self.read_file(&path).await {
            Ok(body) => body,
            Err(response) => {
                eprintln!("Error: {} not found or not readable", path.display());
                return response;
            }
        };

        HTTPResponse::new(HTTPStatus::OK, body, parse_extension(&path))
    }

    async fn on_client_connect(self: &Arc<Self>, source: SocketAddr) -> Option<SocketAddr> {
        println!("Client connected from {}", source);
        Some(source)
    }

    async fn on_client_disconnect(self: &Arc<Self>, client: &mut SocketAddr) {
        println!("{} disconnected", client);
    }

    async fn accept_error(self: &Arc<Self>, error: huntsman_http::Error) {
        eprintln!(
            "Error: An error occurred while accepting a client - {}",
            error
        );
    }

    async fn read_error<'a>(
        self: &'a Arc<Self>,
        client: &'a mut Self::Client,
        error: HTTPParseError,
    ) -> Option<HTTPResponse<'a>> {
        eprintln!(
            "Error: An error occurred while parsing a request from {} - {}",
            client, error
        );

        Some(match error {
            HTTPParseError::HeadersTooLong => HTTPStatus::ContentTooLarge.into(),
            _ => (
                HTTPStatus::BadRequest,
                self.bad_request.0.as_ref(),
                self.bad_request.1,
            )
                .into(),
        })
    }

    async fn send_error(self: &Arc<Self>, client: &mut Self::Client, error: huntsman_http::Error) {
        eprintln!(
            "Error: An error occurred while sending a response to {} - {}",
            client, error
        );
    }
}

impl Default for Static {
    fn default() -> Self {
        Static::new(
            "public/".into(),
            (include_bytes!("400.html") as &[u8], b"text/html"),
            (include_bytes!("404.html") as &[u8], b"text/html"),
        )
    }
}
