use crate::HTTPRequest;
use huntsman::RequestParser;
use std::{convert::Infallible, net::TcpStream};

pub struct HTTPRequestParser;

impl RequestParser for HTTPRequestParser {
    type TransportClient = TcpStream;

    type Error = Infallible;

    type Request<'a> = HTTPRequest;

    fn new(_: &mut Self::TransportClient, _: std::net::SocketAddr) -> Result<Self, Self::Error> {
        Ok(HTTPRequestParser)
    }

    fn parse<'a>(
        &'a mut self,
        client: &mut Self::TransportClient,
    ) -> Result<Self::Request<'a>, Self::Error> {
        todo!("HTTPRequestParser::parse()");
    }
}
