use core::str;
use huntsman_http::{HTTPPath, HTTPTarget};
use std::{
    ffi::{OsStr, OsString},
    path::Path,
};

/// Parses a `url` into an acceptable path under `base`
pub fn parse(url: HTTPTarget, base: &Path) -> Option<OsString> {
    let http_path = HTTPPath::parse(url);

    let mut path = base.to_path_buf();
    for segment in http_path.segments() {
        if segment.len() == 0 || segment == b"." || segment == b".." {
            continue;
        }

        let segment = Path::new(match str::from_utf8(segment) {
            Ok(segment) => segment,
            Err(_) => return None,
        });

        if segment.has_root() {
            return None;
        }

        path.push(segment);
    }

    Some(path.into_os_string())
}

/// Parses the extension of `path` and guesses the MIME type of its contents
pub fn parse_extension<P: AsRef<Path>>(path: P) -> &'static [u8] {
    match path
        .as_ref()
        .extension()
        .unwrap_or(OsStr::new(""))
        .as_encoded_bytes()
    {
        b"css" => b"text/css",
        b"htm" | b"html" => b"text/html",
        b"js" | b"mjs" => b"text/javascript",
        b"txt" => b"text/plain",
        b"wasm" => b"application/wasm",
        b"png" => b"image/png",
        _ => b"application/octet-stream",
    }
}
