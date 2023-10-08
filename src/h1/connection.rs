use std::io::{Read, Write};
use std::net::TcpStream;
use crate::h1::{
    utils::*,
    http,
};

pub(crate) fn handle_client(mut stream: TcpStream, routes: http::Routes) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).unwrap();
    let request = &String::from_utf8_lossy(&buffer[..]);

    let response = handle_request(request.to_string(), routes);
    stream.write(response.as_bytes()).unwrap();
}

fn handle_request(request: String, routes: http::Routes) -> String {
    let headers: Vec<&str> = request.split("\r\n").collect();
    let path = extract_path(headers);
    let query_pairs = parse_query_string(&path);
    let stripped_path: &str = path.split('?').next().unwrap_or(&path);
    handle_route(query_pairs, stripped_path, routes)
}

fn handle_route(query_pairs: http::QueryParams, path: &str, routes: http::Routes) -> String {
    let handler = routes.get(path);

    handle_route_response(handler.copied(), query_pairs)
}

fn handle_route_response(route: Option<http::Handler>, query_params: http::QueryParams) -> String {
    match route {
        None => "HTTP/1.1 404 Not Found".to_owned(),

        Some(handler) => {
            let response = handler(query_params);
            match response {
                http::Response::Plain(content) => {
                    format!("HTTP/1.1 200 OK\r\n\r\n{}", content)
                },
                http::Response::StatusCode((code, detail)) => {
                    format!("HTTP/1.1 {}\r\n\r\n{}", code, detail)
                }
            }
        }
    }
}
