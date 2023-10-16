use crate::h1::client::request_server;

pub fn get(uri: &str) -> String {
    let response = request_server(uri);
    response
}
