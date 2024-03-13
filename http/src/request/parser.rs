use crate::HTTPRequest;
use huntsman::RequestParser;
use std::{
    convert::Infallible,
    net::{SocketAddr, TcpStream},
};

/// A parser for HTTP requests
pub struct HTTPRequestParser;

impl RequestParser for HTTPRequestParser {
    type TransportClient = TcpStream;

    type Error = Infallible;

    type Request<'a> = HTTPRequest;

    fn new(_: &mut TcpStream, _: SocketAddr) -> Result<Self, Infallible> {
        Ok(HTTPRequestParser)
    }

    fn parse<'a>(&'a mut self, client: &mut TcpStream) -> Result<HTTPRequest, Infallible> {
        todo!("HTTPRequestParser::parse()");
    }
}
