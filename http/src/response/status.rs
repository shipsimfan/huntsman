/// The status code of an HTTP response
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HTTPStatus {
    /// Continue
    Continue = 100,

    /// Switching Protocols
    SwitchingProtocols = 101,

    /// OK
    OK = 200,

    /// Created
    Created = 201,

    /// Accepted
    Accepted = 202,

    /// Non-Authoritative Information
    NonAuthoritativeInformation = 203,

    /// No Content
    NoContent = 204,

    /// Reset Content
    ResetContent = 205,

    /// Partial Content
    PartialContent = 206,

    /// Multiple Choices
    MultipleChoices = 300,

    /// Moved Permanently
    MovedPermanently = 301,

    /// Found
    Found = 302,

    /// See Other
    SeeOther = 303,

    /// Not Modified
    NotModified = 304,

    /// Use Proxy
    UseProxy = 305,

    /// Temporary Redirect
    TemporaryRedirect = 307,

    /// Permanent Redirect
    PermanentRedirect = 308,

    /// Bad Request
    BadRequest = 400,

    /// Unauthorized
    Unauthorized = 401,

    /// Payment Required
    PaymentRequired = 402,

    /// Forbidden
    Forbidden = 403,

    /// Not Found
    NotFound = 404,

    /// Method Not Allowed
    MethodNotAllowed = 405,

    /// Not Acceptable
    NotAcceptable = 406,

    /// Proxy Authentication Required
    ProxyAuthenticationRequired = 407,

    /// Request Timeout
    RequestTimeout = 408,

    /// Conflict
    Conflict = 409,

    /// Gone
    Gone = 410,

    /// Length Required
    LengthRequired = 411,

    /// Precondition Failed
    PreconditionFailed = 412,

    /// Content Too Large
    ContentTooLarge = 413,

    /// URI Too Long
    URITooLong = 414,

    /// Unsupported Media Type
    UnsupportedMediaType = 415,

    /// Range Not Satisfiable
    RangeNotSatisfiable = 416,

    /// Expectation Failed
    ExpectationFailed = 417,

    /// Misdirected Request
    MisdirectedRequest = 421,

    /// Unprocessable Content
    UnprocessableContent = 422,

    /// Upgrade Required
    UpgradeRequired = 426,

    /// Internal Server Error
    InternalServerError = 500,

    /// Not Implemented
    NotImplemented = 501,

    /// Bad Gateway
    BadGateway = 502,

    /// Service Unavailable
    ServiceUnavailable = 503,

    /// Gateway Timeout
    GatewayTimeout = 504,

    /// HTTP Version Not Supported
    HTTPVersionNotSupported = 505,
}

impl HTTPStatus {
    /// Gets the number representing this status
    pub fn code(&self) -> usize {
        *self as usize
    }

    /// Gets the number for the status as the three character bytes
    pub fn code_bytes(&self) -> [u8; 3] {
        let code = self.code();

        [
            (code / 100) as u8 + b'0',
            ((code % 100) / 10) as u8 + b'0',
            (code % 10) as u8 + b'0',
        ]
    }

    /// Gets the message for a status code
    pub fn message(&self) -> &str {
        match self {
            HTTPStatus::Continue => "Continue",
            HTTPStatus::SwitchingProtocols => "Switching Protocols",
            HTTPStatus::OK => "OK",
            HTTPStatus::Created => "Created",
            HTTPStatus::Accepted => "Accepted",
            HTTPStatus::NonAuthoritativeInformation => "Non-Authoritative Information",
            HTTPStatus::NoContent => "No Content",
            HTTPStatus::ResetContent => "Reset Content",
            HTTPStatus::PartialContent => "Partial Content",
            HTTPStatus::MultipleChoices => "Multiple Choices",
            HTTPStatus::MovedPermanently => "Moved Permanently",
            HTTPStatus::Found => "Found",
            HTTPStatus::SeeOther => "See Other",
            HTTPStatus::NotModified => "Not Modified",
            HTTPStatus::UseProxy => "Use Proxy",
            HTTPStatus::TemporaryRedirect => "Temporary Redirect",
            HTTPStatus::PermanentRedirect => "Permanent Redirect",
            HTTPStatus::BadRequest => "Bad Request",
            HTTPStatus::Unauthorized => "Unauthorized",
            HTTPStatus::PaymentRequired => "Payment Required",
            HTTPStatus::Forbidden => "Forbidden",
            HTTPStatus::NotFound => "Not Found",
            HTTPStatus::MethodNotAllowed => "Method Not Allowed",
            HTTPStatus::NotAcceptable => "Not Acceptable",
            HTTPStatus::ProxyAuthenticationRequired => "Proxy Authentication Required",
            HTTPStatus::RequestTimeout => "Request Timeout",
            HTTPStatus::Conflict => "Conflict",
            HTTPStatus::Gone => "Gone",
            HTTPStatus::LengthRequired => "Length Required",
            HTTPStatus::PreconditionFailed => "Precondition Failed",
            HTTPStatus::ContentTooLarge => "Content Too Large",
            HTTPStatus::URITooLong => "URI Too Long",
            HTTPStatus::UnsupportedMediaType => "Unsupported Media Type",
            HTTPStatus::RangeNotSatisfiable => "Range Not Satisfiable",
            HTTPStatus::ExpectationFailed => "Expectation Failed",
            HTTPStatus::MisdirectedRequest => "Misdirected Request",
            HTTPStatus::UnprocessableContent => "Unprocessable Content",
            HTTPStatus::UpgradeRequired => "Upgrade Required",
            HTTPStatus::InternalServerError => "Internal Server Error",
            HTTPStatus::NotImplemented => "Not Implemented",
            HTTPStatus::BadGateway => "Bad Gateway",
            HTTPStatus::ServiceUnavailable => "Service Unavailable",
            HTTPStatus::GatewayTimeout => "Gateway Timeout",
            HTTPStatus::HTTPVersionNotSupported => "HTTP Version Not Supported",
        }
    }
}

impl std::fmt::Display for HTTPStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}
