use std::io::Write;
use std::net::TcpStream;

pub(crate) fn request_server(uri: &str) {
   let stream = TcpStream::connect(uri).expect("uri is not available");

    stream.write()
}
