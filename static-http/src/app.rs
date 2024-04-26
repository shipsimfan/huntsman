use huntsman::{App, Protocol};
use huntsman_http::{
    HTTPClientAddress, HTTPParseError, HTTPRequest, HTTPResponse, HTTPStatus, HTTPTarget,
    ListenAddress, HTTP,
};
use lasync::{
    fs::{File, Metadata},
    io::Read,
};
use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    os::unix::ffi::{OsStrExt, OsStringExt},
    path::PathBuf,
    sync::Arc,
};

use crate::path::parse_extension;

/// An HTTP app which serves static files from a path
pub struct StaticHuntsman {
    /// The path to server static files from
    base: PathBuf,

    /// The values to try when a request for a folder occurs
    indexes: Vec<PathBuf>,

    /// The length of the longest index
    longest_index: usize,

    /// The body to respond with when a bad request is submitted
    bad_request: (Cow<'static, [u8]>, &'static [u8]),

    /// The body to respond with when a file cannot be found for a request
    not_found: (Cow<'static, [u8]>, &'static [u8]),
}

/// Displays the target and fields of a request
fn display_request(request: &HTTPRequest, client: &HTTPClientAddress) {
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

/// Attempts to read the file at `path`, or one of the `indexes` if the `path` is a directory.
async fn read_file_or_index(
    path: OsString,
    indexes: &[PathBuf],
) -> Result<(Vec<u8>, OsString), OsString> {
    let file = match File::open(&path).await {
        Ok(file) => file,
        Err(_) => return Err(path),
    };

    let metadata = match file.metadata().await {
        Ok(metadata) => metadata,
        Err(_) => return Err(path),
    };

    if metadata.is_file() {
        return match read_file(file, Some(metadata)).await {
            Some(content) => Ok((content, path)),
            None => Err(path),
        };
    }

    read_indexes(path, indexes).await
}

/// Attempts the read one of the `indexes` in `base_path`
async fn read_indexes(
    path: OsString,
    indexes: &[PathBuf],
) -> Result<(Vec<u8>, OsString), OsString> {
    let mut path = path.into_vec();
    path.push(b'/');
    let base_path_length = path.len();

    for index in indexes {
        path.truncate(base_path_length);
        path.extend_from_slice(index.as_os_str().as_encoded_bytes());

        let file = match File::open(OsStr::from_bytes(&path)).await {
            Ok(file) => file,
            Err(_) => continue,
        };

        let path = OsString::from_vec(path);
        return match read_file(file, None).await {
            Some(content) => Ok((content, path)),
            None => Err(path),
        };
    }

    path.truncate(base_path_length - 1);
    Err(OsString::from_vec(path))
}

/// Attempts to read the file at `path`
async fn read_file(mut file: File, metadata: Option<Metadata>) -> Option<Vec<u8>> {
    let metadata = match metadata {
        Some(metadata) => metadata,
        None => file.metadata().await.ok()?,
    };

    if metadata.is_dir() {
        return None;
    }

    let mut buffer = Vec::with_capacity(metadata.len() as _);
    unsafe { buffer.set_len(metadata.len() as _) };
    file.read_exact(&mut buffer).await.ok()?;

    Some(buffer)
}

impl StaticHuntsman {
    /// Creates a new [`Static`] http serving app
    pub fn new<S1: Into<Cow<'static, [u8]>>, S2: Into<Cow<'static, [u8]>>>(
        base: PathBuf,
        indexes: Vec<PathBuf>,
        bad_request: (S1, &'static [u8]),
        not_found: (S2, &'static [u8]),
    ) -> Self {
        let mut longest_index = 0;
        for index in &indexes {
            let length = index.as_os_str().len();
            if longest_index < length {
                longest_index = length;
            }
        }

        StaticHuntsman {
            base,
            indexes,
            longest_index,
            bad_request: (bad_request.0.into(), bad_request.1),
            not_found: (not_found.0.into(), not_found.1),
        }
    }

    /// Attempts to parse the target into a path or returns a "400 Bad Request" response
    fn parse_path<'a>(&'a self, target: HTTPTarget) -> Result<OsString, HTTPResponse<'a>> {
        crate::path::parse(target, &self.base, self.longest_index + 1).ok_or_else(|| {
            (
                HTTPStatus::BadRequest,
                self.bad_request.0.as_ref(),
                self.bad_request.1,
            )
                .into()
        })
    }

    /// Attempts to read the file at `path`
    async fn read_file<'a>(
        &'a self,
        path: OsString,
    ) -> Result<(Vec<u8>, OsString), (HTTPResponse<'a>, OsString)> {
        read_file_or_index(path, &self.indexes)
            .await
            .map_err(|path| {
                (
                    (
                        HTTPStatus::NotFound,
                        self.not_found.0.as_ref(),
                        self.not_found.1,
                    )
                        .into(),
                    path,
                )
            })
    }
}

impl App for StaticHuntsman {
    type Protocol = HTTP;

    type Client = HTTPClientAddress;

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

        let (body, path) = match self.read_file(path).await {
            Ok(body) => body,
            Err((response, path)) => {
                eprintln!("Error: {:?} not found or not readable", path);
                return response;
            }
        };

        println!("Sending {:?} to {}", path, client);

        HTTPResponse::new(HTTPStatus::OK, body, parse_extension(&path))
    }

    async fn on_client_connect(
        self: &Arc<Self>,
        source: HTTPClientAddress,
    ) -> Option<HTTPClientAddress> {
        println!("Client connected from {}", source);
        Some(source)
    }

    async fn on_client_disconnect(self: &Arc<Self>, client: &mut HTTPClientAddress) {
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
