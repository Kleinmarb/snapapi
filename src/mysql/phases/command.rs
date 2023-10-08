use std::io::{Read, Write};
use std::net::TcpStream;

pub(crate) fn send_command(query: String, stream: &mut TcpStream) {
    let mut packet = Vec::new();

    packet.push(0x03);
    packet.extend(query.as_bytes());

    stream.write_all(&packet).unwrap();
    read_response(stream);
}

fn read_response(stream: &mut TcpStream) {
    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();

    println!("{:#?}", response);
}
