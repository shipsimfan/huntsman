use crate::{error::HandleError, path::parse_extension, response_display::ResponseDisplay};
use huntsman::{App, Protocol};
use huntsman_http::{
    HTTPClientAddress, HTTPListenAddress, HTTPParseError, HTTPRequestDisplay, HTTPResponse,
    HTTPStatus, HTTPTarget, HTTP,
};
use lasync::{
    fs::{File, Metadata},
    io::Read,
};
use oak::{info, LogController, LogLevel, Logger};
use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    os::unix::ffi::{OsStrExt, OsStringExt},
    path::PathBuf,
    sync::Arc,
};

/// An HTTP app which serves static files from a path
pub struct StaticHuntsman {
    /// The path to server static files from
    base: PathBuf,

    /// The values to try when a request for a folder occurs
    indexes: Vec<PathBuf>,

    /// The body to respond with when a bad request is submitted
    bad_request: (Cow<'static, [u8]>, &'static [u8]),

    /// The body to respond with when a file cannot be found for a request
    not_found: (Cow<'static, [u8]>, &'static [u8]),

    /// Log for connections
    connections_logger: Logger,

    /// Log for requests
    access_logger: Logger,

    /// Log for errors
    error_logger: Logger,

    /// Should request headers be logged in the access logger?
    log_headers: bool,

    /// Should request bodies be logged in the access logger?
    log_bodies: bool,

    /// Should response codes and paths be logged in the access logger?
    log_responses: bool,
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
        log_controller: Arc<LogController>,
        log_headers: bool,
        log_bodies: bool,
        log_responses: bool,
    ) -> Self {
        let connections_logger = log_controller.create_logger("connections");
        let access_logger = log_controller.create_logger("access");
        let error_logger = log_controller.create_logger("error");

        StaticHuntsman {
            base,
            indexes,
            bad_request: (bad_request.0.into(), bad_request.1),
            not_found: (not_found.0.into(), not_found.1),
            connections_logger,
            access_logger,
            error_logger,
            log_headers,
            log_bodies,
            log_responses,
        }
    }

    /// Attempts to parse the target into a path or returns a "400 Bad Request" response
    fn parse_path<'a>(&'a self, target: HTTPTarget) -> Result<OsString, HTTPResponse<'a>> {
        crate::path::parse(target, &self.base).ok_or_else(|| {
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

    /// Handles a request from a client
    async fn do_handle_request<'a, 'b>(
        self: &'a Arc<Self>,
        client: HTTPClientAddress,
        request: &<HTTP as Protocol>::Request<'b>,
    ) -> Result<(HTTPResponse<'a>, Option<PathBuf>), HandleError<'a>> {
        let path = self
            .parse_path(request.target())
            .map_err(|response| HandleError::bad_path(response, client))?;

        let (body, path) = self.read_file(path).await.map_err(|(response, path)| {
            HandleError::not_found_or_unreadable(response, path, client)
        })?;

        Ok((
            HTTPResponse::new(HTTPStatus::OK, body, parse_extension(&path)),
            Some(path.into()),
        ))
    }
}

impl App for StaticHuntsman {
    type Protocol = HTTP;

    type Client = HTTPClientAddress;

    async fn on_server_start(self: &Arc<Self>, addresses: &[HTTPListenAddress]) {
        info!(
            self.connections_logger,
            "Sever listening on: {:?}", addresses
        );
    }

    async fn handle_request<'a, 'b>(
        self: &'a Arc<Self>,
        client: &'a mut Self::Client,
        request: <Self::Protocol as Protocol>::Request<'b>,
    ) -> HTTPResponse<'a> {
        let result = self.do_handle_request(*client, &request).await;

        let response_display = if self.log_responses {
            Some(match &result {
                Ok((response, response_path)) => ResponseDisplay::new(
                    response.status().code(),
                    response_path.as_deref().map(|path| path),
                ),
                Err(error) => ResponseDisplay::new(error.response().status().code(), None),
            })
        } else {
            None
        };

        self.access_logger.log(
            LogLevel::Info,
            &HTTPRequestDisplay::new(
                &request,
                *client,
                response_display,
                self.log_headers,
                self.log_bodies,
            ),
        );

        match result {
            Ok((response, _)) => response,
            Err(error) => {
                self.error_logger.log(LogLevel::Error, &error);
                error.unwrap_response()
            }
        }
    }

    async fn on_client_connect(
        self: &Arc<Self>,
        source: HTTPClientAddress,
    ) -> Option<HTTPClientAddress> {
        self.connections_logger.log(
            LogLevel::Info,
            &format_args!("Client connected from {}", source),
        );
        Some(source)
    }

    async fn on_client_disconnect(self: &Arc<Self>, client: &mut HTTPClientAddress) {
        self.connections_logger
            .log(LogLevel::Info, &format_args!("{} disconnected", client));
    }

    async fn accept_error(self: &Arc<Self>, error: huntsman_http::Error) {
        self.error_logger.log(
            LogLevel::Error,
            &format_args!("An error occurred while accepting a client - {}", error),
        );
    }

    async fn read_error<'a>(
        self: &'a Arc<Self>,
        client: &'a mut Self::Client,
        error: HTTPParseError,
    ) -> Option<HTTPResponse<'a>> {
        self.error_logger.log(
            LogLevel::Error,
            &format_args!(
                "An error occurred while parsing a request from {} - {}",
                client, error
            ),
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
        self.error_logger.log(
            LogLevel::Error,
            &format_args!(
                "An error occurred while sending a response to {} - {}",
                client, error
            ),
        );
    }
}
