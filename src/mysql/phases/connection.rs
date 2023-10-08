use std::net::TcpStream;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Write, Read};

const CLIENT_PROTOCOL_41: u32 = 0x00000200;
const CLIENT_CONNECT_WITH_DB: u32 = 0x00000008;
const UTF8MB4_GENERAL_CI: u8 = 0xE0;
const MAX_PACKET_SIZE: u32 = 1 << 24; // 16MB

pub(crate) fn connect(stream: &mut TcpStream, db: &Option<String>, username: &str, password: &str) {
    read_handshake(stream, db, username, password);
}

fn read_handshake(stream: &mut TcpStream, db: &Option<String>, username: &str, password: &str) {
    // let mut response = Vec::new();
    // let mut buffer = [0; 1024];

    // let n = stream.read(&mut buffer).expect("f");
    // response.extend_from_slice(&buffer[..n]);
    // println!("{:#?}", response);

    respond_handshake(stream, db, username, password);
}

fn respond_handshake(stream: &mut TcpStream, db: &Option<String>, username: &str, password: &str) {
    let mut response = Vec::new();

    let mut capabilities: u32 = 0;
    capabilities |= CLIENT_PROTOCOL_41;

    if !db.is_none() {
        capabilities |= CLIENT_CONNECT_WITH_DB;
    }

    response.write_u32::<LittleEndian>(capabilities).unwrap();
    response.write_u32::<LittleEndian>(MAX_PACKET_SIZE).unwrap();
    response.push(UTF8MB4_GENERAL_CI);
    response.extend(username.as_bytes());

    // Filler bytes
    for _ in 0..23 {
        response.push(0);
    }

    response.extend(password.as_bytes());

    if !db.is_none() {
        response.extend(db.clone().unwrap().as_bytes());
    }

    stream.write_all(&response).unwrap();
    read_server_response(stream, password);
}

fn read_server_response(stream: &mut TcpStream, _: &str) {
    let mut response = Vec::new();
    let mut buffer = [0; 1024];

    let n = stream.read(&mut buffer).expect("f");
    response.extend_from_slice(&buffer[..n]);
    println!("{:#?}", response);

}
