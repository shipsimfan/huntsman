use huntsman_http::{HTTPRequest, HTTPResponse, HTTP};
use std::{net::SocketAddr, sync::Arc};

struct Static;

fn main() {
    huntsman::run(Static, huntsman::Options::default()).unwrap()
}

impl huntsman::App for Static {
    type Protocol = HTTP;

    fn handle_request<'a>(
        self: &Arc<Self>,
        source: SocketAddr,
        request: HTTPRequest,
    ) -> HTTPResponse {
        todo!("handle_request()");
    }
}
