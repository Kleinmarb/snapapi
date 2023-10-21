use crate::http::client::connection::request_server;

pub fn request(uri: &str) -> (String, u16) {
    let response = request_server(uri);
    response
}
