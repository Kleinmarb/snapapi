use std::io::{Read, Write};
use std::net::TcpStream;
use crate::http::http;
use crate::http::utils::{extract_path, extract_query_params};


pub(crate) fn handle_client(mut stream: TcpStream, routes: http::Routes) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]).to_string();

    let response = handle_request(request, routes);
    stream.write(response.as_bytes()).unwrap();
}

fn handle_request(request: String, routes: http::Routes) -> String {
    let headers: Vec<&str> = request.split("\r\n").collect();

    let path = extract_path(headers);
    let query_pairs = extract_query_params(&path);

    handle_route(query_pairs, path, routes)
}

fn handle_route(query_pairs: http::QueryParams, path: &str, routes: http::Routes) -> String {
    let route = routes.get(path);

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
