use std::{fmt::Display, path::Path};

pub struct ResponseDisplay<'a> {
    code: usize,
    path: Option<&'a Path>,
}

impl<'a> ResponseDisplay<'a> {
    pub fn new(code: usize, path: Option<&'a Path>) -> Self {
        ResponseDisplay { code, path }
    }
}

impl<'a> Display for ResponseDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)?;
        if let Some(path) = self.path {
            write!(f, " returning \"{}\"", path.display())?;
        }

        Ok(())
    }
}
