use huntsman_http::HTTPTarget;
use std::{
    ffi::{OsStr, OsString},
    os::unix::ffi::OsStringExt,
    path::Path,
};

/// Parses a `url` into an acceptable path under `base`
///
/// If there is an error with the `url`, [`None`] is returned and a "400 Bad Request" response
/// should be given to the client.
///
/// `extra_padding` allows more space to be allocated for the path to allow additions to the end
/// after parsing.
pub fn parse(url: HTTPTarget, base: &Path, extra_padding: usize) -> Option<OsString> {
    // The capacity will either be:
    //  - one greater (from a trailing slash) if no "%XX" url segments exits, or
    //  - it will shrink because "%XX" will take three bytes and convert them to one.
    let capacity = base.as_os_str().len() + url.len() + 1 + extra_padding;
    let mut path = Vec::with_capacity(capacity);

    // Insert the base path
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

    // An iterator over the provided url
    let mut url = url.into_iter().map(|c| *c).peekable();

    while let Some(c) = url.next() {
        // Check for the end of a segment
        match c {
            b'/' => {}
            b'?' => break, // Reached the GET parameters
            _ => return None,
        }

        // Get the start of the segment
        let segment_index = path.len();

        // Collect until we reach the end of the segment
        while let Some(&c) = url.peek() {
            match c {
                b'/' | b'?' => break,
                b'%' => {
                    // Get the next two digits
                    url.next();
                    let c1 = url.next().unwrap_or(0);
                    let c2 = url.next().unwrap_or(0);

                    // Verify they are actually digits
                    if !c1.is_ascii_digit() || !c2.is_ascii_digit() {
                        return None;
                    }

                    let c = (c1 - b'0') * 10 + (c2 - b'0');
                    if c == 0 {
                        return None;
                    }

                    path.push(c);
                }
                0 => return None,
                _ => {
                    path.push(c);
                    url.next();
                }
            }
        }

        // Ignore the segment if it was empty
        if path.len() == segment_index {
            continue;
        }

        // Check if the segment is a ".."
        let segment = &path[segment_index..];
        if segment == b".." {
            // Truncate back to the last index or the base path
            if let Some(last_index) = segments.pop() {
                path.truncate(last_index);
            } else {
                path.truncate(base_length);
            }
            continue;
        }

        // If it wasn't empty or "..", push its index and append a '/'
        segments.push(segment_index);
        path.push(b'/');
    }

    // Remove the final trailing "/"
    path.pop();
    Some(OsString::from_vec(path))
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
