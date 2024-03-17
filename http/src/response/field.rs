use std::{borrow::Cow, io::Write, net::TcpStream};

/// A field describing metadata about an HTTP response
pub struct HTTPResponseField {
    /// The name of the field
    name: Cow<'static, [u8]>,

    /// THe value of the field
    value: Cow<'static, [u8]>,
}

impl HTTPResponseField {
    /// Creates a new [`HTTPResponseField`]
    pub fn new<T1: Into<Cow<'static, [u8]>>, T2: Into<Cow<'static, [u8]>>>(
        name: T1,
        value: T2,
    ) -> Self {
        HTTPResponseField {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Gets the name of this field
    pub fn name(&self) -> &[u8] {
        &self.name
    }

    /// Gets the value of this field
    pub fn value(&self) -> &[u8] {
        &self.value
    }

    /// Sets the name of this field
    pub fn set_name<T: Into<Cow<'static, [u8]>>>(&mut self, name: T) {
        self.name = name.into()
    }

    /// Sets the value of this field
    pub fn set_value<T: Into<Cow<'static, [u8]>>>(&mut self, value: T) {
        self.value = value.into()
    }

    /// Writes this field onto `stream`
    pub(super) fn write(&self, socket: &mut TcpStream) -> Result<(), std::io::Error> {
        socket.write_all(&self.name)?;
        socket.write_all(b": ")?;
        socket.write_all(&self.value)?;
        socket.write_all(b"\r\n")
    }
}
