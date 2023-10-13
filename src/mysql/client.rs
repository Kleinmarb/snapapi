use std::net::TcpStream;
use crate::mysql::phases::connection::connect;
use crate::mysql::cursor::Cursor;

pub struct MySQL {
    stream: Option<TcpStream>,
    db: Option<String>,
    password: Option<String>,
    username: Option<String>,
    port: u16,
    size: usize,
}

#[allow(dead_code)]
impl MySQL {
    pub fn new() -> Self {
        Self {
            stream: None,
            db: None,
            password: None,
            username: None,
            port: 3306,
            size: 0,
        }
    }

    #[inline]
    pub fn db(&mut self, db: &str) -> &mut Self {
        self.db = Some(db.to_owned());
        self
    }

    #[inline]
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    #[inline]
    pub fn password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.to_owned());
        self
    }

    #[inline]
    pub fn username(&mut self, username: &str) -> &mut Self {
        self.username = Some(username.to_owned());
        self
    }

    #[inline]
    pub fn size(&mut self, size: usize) -> &mut Self {
        self.size = size;
        self
    }

    pub fn connect(&mut self) -> Cursor {
        if self.size == 0 {
            panic!("please enter a size")
        }

        if self.username.is_none() {
            panic!("please enter a username")
        }

        if self.password.is_none() {
            panic!("please enter a password")
        }

        self.stream = Some(TcpStream::connect(("127.0.0.1", self.port)).unwrap());
        let mut connection_stream = self
            .stream
            .as_mut()
            .unwrap()
            .try_clone()
            .unwrap();

        connect(
            &mut connection_stream,
            &self.db,
            self.username.as_mut().unwrap(),
            self.password.as_mut().unwrap()
        );

        Cursor::new(self.stream.as_mut().unwrap().try_clone().unwrap(), self.size)
    }
}
