use threatpool::ThreadPool;
use std::net::TcpListener;
use std::sync::Arc;
use crate::http;
use crate::http::server::connection::handle_client;

pub struct SnapAPI {
    routes: Vec<http::Route>,
    port: u16,
}

impl SnapAPI {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
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
        self.routes.push(http::Route { path: path.to_owned(), handler });
        self
    }

    pub fn run(&self, size: usize) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        let pool = ThreadPool::new(size);

        let routes = Arc::new(self.routes.clone());

        loop {
            let (stream, _) = listener.accept().unwrap();
            let routes = Arc::clone(&routes);

            pool.execute(move || {
                handle_client(stream, routes);
            });
        }
    }
}
