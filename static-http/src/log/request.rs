use huntsman_http::{HTTPClientAddress, HTTPMethod, HTTPRequestField, HTTPTarget};
use std::path::Path;

/// Displays a request for logging
pub struct RequestDisplay<'a> {
    /// The method of the request
    method: HTTPMethod,

    // The source of the request
    source: HTTPClientAddress,

    /// The target of the request
    target: HTTPTarget<'a>,

    /// The headers of the request if they should be displayed
    headers: Option<&'a [HTTPRequestField<'a>]>,

    /// The body of the request if it should be displayed
    body: Option<&'a [u8]>,

    /// The response to the request if it should be displayed
    response: Option<(usize, Option<&'a Path>)>,
}

impl<'a> RequestDisplay<'a> {
    /// Creates a new [`RequestDisplay`]
    pub fn new(
        method: HTTPMethod,
        source: HTTPClientAddress,
        target: HTTPTarget<'a>,
        headers: Option<&'a [HTTPRequestField]>,
        body: Option<&'a [u8]>,
        response: Option<(usize, Option<&'a Path>)>,
    ) -> Self {
        RequestDisplay {
            method,
            source,
            target,
            headers,
            body,
            response,
        }
    }
}

impl<'a> std::fmt::Display for RequestDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} request for \"{}\" from {}",
            self.method, self.target, self.source
        )?;

        if let Some((response, response_path)) = self.response {
            write!(f, " ({}", response)?;
            if let Some(response_path) = response_path {
                write!(f, " returning \"{}\"", response_path.display())?;
            }
            write!(f, " )")?;
        }

        if let Some(headers) = self.headers {
            for header in headers {
                write!(f, "\n  {}", header)?;
            }
        }

        if let Some(body) = self.body {
            write!(f, "\nREQUEST BODY:\n{}", String::from_utf8_lossy(body))?;
        }

        Ok(())
    }
}
