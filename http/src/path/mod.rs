use crate::HTTPTarget;
use std::{borrow::Cow, fmt::Debug, ops::Index};

mod query_param;

pub use query_param::HTTPQueryParam;

/// A parsed HTTP request path
#[derive(Clone, PartialEq, Eq)]
pub struct HTTPPath<'a> {
    segments: Vec<Cow<'a, [u8]>>,
    query_params: Vec<HTTPQueryParam<'a>>,
}

fn from_hexdigit(x: u8) -> u8 {
    if x.is_ascii_digit() {
        x - b'0'
    } else if x.is_ascii_lowercase() {
        x - b'f' + 10
    } else {
        x - b'F' + 10
    }
}

fn parse_segment_until<'a, F: Fn(u8) -> bool>(
    mut i: usize,
    target: HTTPTarget<'a>,
    predicate: F,
) -> (Cow<'a, [u8]>, usize) {
    let start = i;
    let mut segment = None;

    while target.len() > i && !predicate(target[i]) {
        if target[i] == b'%'
            && target.len() > i + 2
            && target[i + 1].is_ascii_hexdigit()
            && target[i + 2].is_ascii_hexdigit()
        {
            let high = from_hexdigit(target[i + 1]);
            let low = from_hexdigit(target[i + 2]);

            if segment.is_none() {
                let mut new_segment = Vec::with_capacity(i - start);
                new_segment.extend_from_slice(&target[start..i]);
                segment = Some(new_segment);
            }

            segment.as_mut().unwrap().push(low | (high << 4));
            i += 3;
            continue;
        }

        if let Some(segment) = &mut segment {
            segment.push(target[i]);
        }
        i += 1;
    }

    let segment = match segment {
        Some(segment) => segment.into(),
        None => target.as_slice()[start..i].into(),
    };
    (segment, i)
}

impl<'a> HTTPPath<'a> {
    /// Parses `target` into an [`HTTPPath`]
    pub fn parse(target: HTTPTarget<'a>) -> Self {
        if target.len() == 0 {
            return HTTPPath {
                segments: Vec::new(),
                query_params: Vec::new(),
            };
        }

        let mut i = 0;
        let mut leading_slash = target[0] == b'/';
        let mut segments = Vec::new();
        while target.len() > i && target[i] != b'?' {
            if leading_slash {
                i += 1;
            }

            let (segment, new_i) = parse_segment_until(i, target, |x| x == b'/' || x == b'?');
            i = new_i;

            if segment.len() > 0 {
                segments.push(segment);
            }

            leading_slash = true;
        }

        if target.len() > i && target[i] == b'?' {
            i += 1;
        }

        let mut query_params = Vec::new();
        while target.len() > i {
            let (query_param, new_i) = HTTPQueryParam::parse(i, target);
            i = new_i;
            query_params.push(query_param);
        }

        HTTPPath {
            segments,
            query_params,
        }
    }

    /// Gets the number of segments which make up this path
    pub fn num_segments(&self) -> usize {
        self.segments.len()
    }

    /// Gets the segment at index `i`
    pub fn segment(&self, i: usize) -> Option<&[u8]> {
        self.segments.get(i).map(|segment| segment.as_ref())
    }

    /// Gets an iterator over the segments of this path
    pub fn segments(&self) -> impl Iterator<Item = &[u8]> {
        self.segments.iter().map(|segment| segment.as_ref())
    }

    /// Gets the query parameters of the request path
    pub fn query_params(&self) -> &[HTTPQueryParam<'a>] {
        &self.query_params
    }

    /// Attempts to get a query parameter with `key`
    pub fn query_param(&self, key: &[u8]) -> Option<&[u8]> {
        for query_param in &self.query_params {
            if query_param.key() == key {
                return Some(query_param.value());
            }
        }

        None
    }
}

impl<'a> Index<usize> for HTTPPath<'a> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        self.segments[index].as_ref()
    }
}

impl<'a> Debug for HTTPPath<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HTTPPath { segments: [")?;
        for i in 0..self.segments.len() {
            write!(f, "\"{}\"", String::from_utf8_lossy(&self.segments[i]))?;

            if i < self.segments.len() - 1 {
                f.write_str(", ")?;
            }
        }

        write!(f, "], query_params: {:?}", self.query_params)?;
        f.write_str(" }")
    }
}
