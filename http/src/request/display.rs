use crate::{HTTPClientAddress, HTTPRequest};
use std::fmt::{Display, Formatter, Result};

/// Displays a request for logging
///
/// The [`u8`] default is just an item that implements display, it could be anything but I chose
/// something small.
pub struct HTTPRequestDisplay<'a, Response: Display = u8> {
    /// The request to display
    request: &'a HTTPRequest<'a>,

    /// The source of the request
    source: HTTPClientAddress,

    /// The item to be displayed as the response (e.g. status code or file path)
    response: Option<Response>,

    /// Should headers be displayed?
    display_headers: bool,

    /// Should the body be displayed?
    display_body: bool,
}

impl<'a, Response: Display> HTTPRequestDisplay<'a, Response> {
    /// Creates a new [`RequestDisplay`]
    pub fn new(
        request: &'a HTTPRequest<'a>,
        source: HTTPClientAddress,
        response: Option<Response>,
        display_headers: bool,
        display_body: bool,
    ) -> Self {
        HTTPRequestDisplay {
            request,
            source,
            response,
            display_headers,
            display_body,
        }
    }
}

impl<'a, Response: Display> Display for HTTPRequestDisplay<'a, Response> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} request for \"{}\" from {}",
            self.request.method(),
            self.request.target(),
            self.source
        )?;

        if let Some(response) = &self.response {
            write!(f, " ({})", response)?;
        }

        if self.display_headers {
            for header in self.request.fields() {
                write!(f, "\n  {}", header)?;
            }
        }

        if self.display_body {
            if let Some(body) = &self.request.body {
                write!(f, "\nREQUEST BODY:\n{}", String::from_utf8_lossy(body))?;
            }
        }

        Ok(())
    }
}
