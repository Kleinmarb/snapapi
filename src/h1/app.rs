use threatpool::ThreadPool;
use fxhash::FxHashMap;
use std::net::TcpListener;
use crate::h1::{
    http,
    connection::handle_client,
};

pub struct SnapAPI {
    routes: http::Routes,
    port: u16,
}

impl SnapAPI {
    pub fn new() -> Self {
        Self {
            routes: FxHashMap::default(),
            port: 7878,
        }
    }

    #[inline]
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    #[inline]
    pub fn route(&mut self, path: &str, handler: http::Handler) -> &mut Self {
        self.routes.insert(path.to_owned(), handler);
        self
    }

    pub fn run(&self, size: usize) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
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
