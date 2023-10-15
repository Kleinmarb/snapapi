use fxhash::FxHashMap;

pub type QueryParams = FxHashMap<String, String>;
pub(crate) type Handler = fn(QueryParams) -> Response;
pub(crate) type Routes = FxHashMap<String, Handler>;

#[allow(dead_code)]
pub enum Response {
    Plain(String),
    StatusCode(String, String),
}

#[allow(unused)]
impl Response {
    pub fn from_status_code(status_code: u16, detail: &str) -> Self {
        Self::StatusCode(
            status_code.to_string(),
            detail.to_owned(),
        )
    }
}
