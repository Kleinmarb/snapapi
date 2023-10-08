use crate::h1::http;
use fxhash::FxHashMap;

pub(crate) fn extract_path(headers: Vec<&str>) -> String {
    match headers.get(0) {
        Some(request_line) => {
            let request_line: Vec<&str> = request_line.split(" ").collect();
            match request_line.as_slice() {
                [_, path, ..] => path.to_string(),
                _ => "".to_owned(),
            }
        }
        None => "".to_owned(),
    }
}

pub(crate) fn parse_query_string(path: &str) -> http::QueryParams {
    let mut result: http::QueryParams = FxHashMap::default();
    if let Some(query) = path.split('?').nth(1) {
        for pair in query.split('&') {
            let mut key_value = pair.splitn(2, '=');
            if let (Some(key), Some(value)) = (key_value.next(), key_value.next()) {
                result.insert(key.to_string(), value.to_string());
            }
        }
    }
    result
}
