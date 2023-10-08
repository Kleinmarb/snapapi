use fxhash::FxHashMap;

pub type QueryParams = FxHashMap<String, String>;
pub(crate) type Handler = fn(QueryParams) -> Response;
pub(crate) type Routes = FxHashMap<String, Handler>;

#[allow(dead_code)]
pub enum Response {
    Plain(String),
    StatusCode((String, String)),
}
