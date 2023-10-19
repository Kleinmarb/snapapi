use crate::h1::http;
use fxhash::FxHashMap;

#[inline]
pub(crate) fn extract_path(headers: Vec<&str>) -> String {
    match headers.get(0) {
        Some(request_line) => {
            match request_line.split_whitespace().collect::<Vec<&str>>().as_slice() {
                [_, path, ..] => path.to_string(),
                _ => String::new(),
            }
        }
        None => String::new(),
    }
}

#[inline]
pub(crate) fn parse_query_string(path: &str) -> http::QueryParams {
    let mut result: http::QueryParams = FxHashMap::default();
    if let Some(query) = path.split('?').nth(1) {
        for pair in query.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                result.insert(key.to_string(), value.to_string());
            }
        }
    }
    result
}
