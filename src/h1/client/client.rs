use crate::h1::client::connection::request_server;

pub fn get(uri: &str) -> String {
    let response = request_server(uri);
    response
}
