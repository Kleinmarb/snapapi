use std::io::{Read, Write};
use std::net::TcpStream;

pub(crate) fn request_server(uri: &str) -> String {
    let mut stream = TcpStream::connect(uri).expect("uri is not available");
    stream.write(b"GET / HTTP/1.1\r\n\r\n").unwrap();

    let response = get_response(stream);
    response
}

fn get_response(mut stream: TcpStream) -> String {
    let mut buffer = &[0; 1024];
    stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(buffer).to_string()
}
