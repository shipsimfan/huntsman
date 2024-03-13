struct Static;

fn main() {
    huntsman::run::<Static>(huntsman::Options::default()).unwrap()
}

impl huntsman::App for Static {
    type Protocol = huntsman_http::HTTP;

    fn handle_request<'a>(
        self: std::sync::Arc<Self>,
        request: <<Self::Protocol as huntsman::Protocol>::RequestParser as huntsman::RequestParser>::Request<'a>,
    ) -> <Self::Protocol as huntsman::Protocol>::Response {
        todo!("handle_request()");
    }

    fn accept_error(
        self: std::sync::Arc<Self>,
        error: <<Self::Protocol as huntsman::Protocol>::Transport as huntsman::Transport>::Error,
    ) {
        todo!("accept_error()");
    }

    fn parse_error(
        self: std::sync::Arc<Self>,
        error: <<Self::Protocol as huntsman::Protocol>::RequestParser as huntsman::RequestParser>::Error,
    ) -> Option<<Self::Protocol as huntsman::Protocol>::Response> {
        todo!("parse_error()");
    }

    fn send_error(
        self: std::sync::Arc<Self>,
        error: <<Self::Protocol as huntsman::Protocol>::Transport as huntsman::Transport>::Error,
    ) {
        todo!("send_error()");
    }
}
