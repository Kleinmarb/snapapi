use std::io::{Read, Write};
use std::net::TcpStream;

pub(crate) fn request_server(_uri: &str) -> String {
    let route = "/"; // Extract from uri
    let host = "127.0.0.1:7878"; // Extract from uri

    let mut stream = TcpStream::connect(host).expect("uri is not available");
    stream.write(format!("GET {route} HTTP/1.1\r\nHost: {host}\r\n").as_bytes()).unwrap();

    let response = get_response(stream);
    response
}

fn get_response(mut stream: TcpStream) -> String {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = String::from_utf8_lossy(&buffer[..]).to_string();
    response // Format for the end user before returning
}
