use std::net::TcpStream;
use crate::mysql::phases::command::send_command;
use threatpool::ThreadPool;

pub struct Cursor {
    stream: TcpStream,
    pool: ThreadPool,
}

#[allow(dead_code)]
impl Cursor {
    pub(crate) fn new(stream: TcpStream, size: usize) -> Self {
        Self {
            stream,
            pool: ThreadPool::new(size),
        }
    }

    pub fn query(&mut self, query: &str) {
        let mut query_stream = self
            .stream
            .try_clone()
            .unwrap();

        let query = query.to_owned();

        self.pool.execute(
            move || {
                send_command(query, &mut query_stream);
            }
        );
    }
}
