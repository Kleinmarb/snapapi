use std::io::{Read, Write};
use std::net::TcpStream;
use crate::h1::utils::extract_ip_and_path;

pub(crate) fn request_server(uri: &str) -> String {
    let (ip, path) = extract_ip_and_path(uri);

    let mut stream = TcpStream::connect(ip).expect("uri is not available");
    stream.write(format!("GET {path} HTTP/1.1\r\nHost: {ip}\r\n").as_bytes()).unwrap();

    let response = get_response(stream);
    response
}

fn get_response(mut stream: TcpStream) -> String {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = String::from_utf8_lossy(&buffer[..]).to_string();
    response
}
