use std::io::{Read, Write};
use std::net::TcpStream;
use crate::http;
use crate::http::utils::{extract_route, extract_query_params, get_handler_by_route};


pub(crate) fn handle_client(mut stream: TcpStream, routes: Vec<(String, http::Handler)>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]).to_string();

    let response = handle_request(request, routes);
    stream.write(response.as_bytes()).unwrap();
}

fn handle_request(request: String, routes: Vec<(String, http::Handler)>) -> String {
    let headers: Vec<&str> = request.split("\r\n").collect();

    let route = extract_route(headers);
    let query_pairs = extract_query_params(&route);

    handle_route(query_pairs, route, routes)
}

fn handle_route(query_pairs: http::QueryParams, route: &str, routes: Vec<(String, http::Handler)>) -> String {
    let route = get_handler_by_route(routes, route);

    match route {
        None => "HTTP/1.1 404 Not Found".to_owned(),

        Some(handler) => {
            let response = handler(query_pairs);
            match response {
                http::Response::Plain(content) => {
                    format!("HTTP/1.1 200 OK\r\n\r\n{}", content)
                },
                http::Response::StatusCode(code, detail) => {
                    format!("HTTP/1.1 {} {}\r\n\r\n {}", code, detail, detail)
                }
            }
        }
    }
}
