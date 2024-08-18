/// A type which can be parsed from a query parameter
pub trait FromHTTPQueryParam<'a>: Sized {
    /// An error that can occur while parsing
    type Error: std::error::Error;

    /// Attempts to parse `query_param` into `Self`
    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error>;
}

impl<'a> FromHTTPQueryParam<'a> for &'a [u8] {
    type Error = std::convert::Infallible;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(query_param)
    }
}

impl<'a> FromHTTPQueryParam<'a> for std::borrow::Cow<'a, [u8]> {
    type Error = std::convert::Infallible;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(std::borrow::Cow::Borrowed(query_param))
    }
}

impl<'a> FromHTTPQueryParam<'a> for Vec<u8> {
    type Error = std::convert::Infallible;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(query_param.to_vec())
    }
}

impl<'a> FromHTTPQueryParam<'a> for &'a str {
    type Error = std::str::Utf8Error;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(query_param)
    }
}

impl<'a> FromHTTPQueryParam<'a> for std::borrow::Cow<'a, str> {
    type Error = std::str::Utf8Error;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(query_param).map(|query_param| std::borrow::Cow::Borrowed(query_param))
    }
}

impl<'a> FromHTTPQueryParam<'a> for String {
    type Error = std::str::Utf8Error;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(query_param).map(|query_param| query_param.to_string())
    }
}

impl<'a> FromHTTPQueryParam<'a> for i8 {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        i8::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for i16 {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        i16::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for i32 {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        i32::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for i64 {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        i64::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for i128 {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        i128::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for isize {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        isize::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for u8 {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        u8::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for u16 {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        u16::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for u32 {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        u32::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for u64 {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        u64::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for u128 {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        u128::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for usize {
    type Error = std::num::ParseIntError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        usize::from_str_radix(unsafe { std::str::from_utf8_unchecked(query_param) }, 10)
    }
}

impl<'a> FromHTTPQueryParam<'a> for f32 {
    type Error = std::num::ParseFloatError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        unsafe { std::str::from_utf8_unchecked(query_param) }.parse()
    }
}

impl<'a> FromHTTPQueryParam<'a> for f64 {
    type Error = std::num::ParseFloatError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        unsafe { std::str::from_utf8_unchecked(query_param) }.parse()
    }
}

impl<'a> FromHTTPQueryParam<'a> for bool {
    type Error = std::str::ParseBoolError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        unsafe { std::str::from_utf8_unchecked(query_param) }.parse()
    }
}

impl<'a> FromHTTPQueryParam<'a> for std::net::Ipv4Addr {
    type Error = std::net::AddrParseError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        std::net::Ipv4Addr::parse_ascii(query_param)
    }
}

impl<'a> FromHTTPQueryParam<'a> for std::net::Ipv6Addr {
    type Error = std::net::AddrParseError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        std::net::Ipv6Addr::parse_ascii(query_param)
    }
}

impl<'a> FromHTTPQueryParam<'a> for std::net::IpAddr {
    type Error = std::net::AddrParseError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        std::net::IpAddr::parse_ascii(query_param)
    }
}

impl<'a> FromHTTPQueryParam<'a> for std::net::SocketAddrV4 {
    type Error = std::net::AddrParseError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        std::net::SocketAddrV4::parse_ascii(query_param)
    }
}

impl<'a> FromHTTPQueryParam<'a> for std::net::SocketAddrV6 {
    type Error = std::net::AddrParseError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        std::net::SocketAddrV6::parse_ascii(query_param)
    }
}

impl<'a> FromHTTPQueryParam<'a> for std::net::SocketAddr {
    type Error = std::net::AddrParseError;

    fn from_query_param(query_param: &'a [u8]) -> Result<Self, Self::Error> {
        std::net::SocketAddr::parse_ascii(query_param)
    }
}
