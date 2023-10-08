use threadpool::ThreadPool;
use fxhash::FxHashMap;
use crate::http;
use crate::utils::*;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream, routes: http::Routes) {
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

    if handler.is_none() {
        let handler = routes.get(&format!("{}/", path));
        handle_route_response(handler.copied(), query_pairs)
    } else {
        handle_route_response(handler.copied(), query_pairs)
    }
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

pub struct SnapAPI {
    routes: http::Routes,
}

#[allow(dead_code)]
impl SnapAPI {
    pub fn new() -> Self {
        Self {
            routes: FxHashMap::default(),
        }
    }

    pub async fn route(&mut self, path: &str, handler: http::Handler) -> &mut Self{
        self.routes.insert(path.to_owned(), handler);
        self
    }

    pub async fn run(&self, size: usize) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        let pool = ThreadPool::new(size);

        loop {
            let (stream, _) = listener.accept().unwrap();
            let routes = self.routes.clone();

            pool.execute(move || {
                handle_client(stream, routes);
            });
        }
    }
}
