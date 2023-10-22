pub mod server;
pub mod client;
pub(crate) mod utils;

use rustc_hash::FxHashMap;

pub type QueryParams = FxHashMap<String, String>;
pub(crate) type Handler = fn(QueryParams) -> Response;
pub(crate) type Routes = FxHashMap<String, Handler>;

pub enum Response {
    Plain(String),
    StatusCode(String, String),
}

impl Response {
    pub fn from_status_code(status_code: u16, detail: &str) -> Self {
        Self::StatusCode(
            status_code.to_string(),
            detail.to_owned(),
        )
    }
}
