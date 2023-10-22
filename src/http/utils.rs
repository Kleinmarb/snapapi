use crate::http::http;
use rustc_hash::FxHashMap;

#[inline]
pub(crate) fn extract_path(headers: Vec<&str>) -> &str {
    match headers.get(0) {
        Some(request_line) => {
            match request_line.split_whitespace().collect::<Vec<&str>>().as_slice() {
                [_, path, ..] => {
                    let path = path.split('?').next().unwrap_or(&path);
                    path
                },

                _ => "",
            }
        },

        None => "",
    }
}

#[inline]
pub(crate) fn extract_query_params(path: &str) -> http::QueryParams {
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

#[inline]
pub(crate) fn extract_ip_and_path(uri: &str) -> (&str, &str) {
    if let Some(start) = uri.find("http://") {
        if let Some(end) = uri[start + 7..].find('/') {
            return (&uri[start + 7..start + 7 + end], &uri[start + 7 + end..]);
        }
    }

    (&uri, "")
}

#[inline]
pub(crate) fn extract_content_and_status_code(http_response: String) -> (String, u16) {
    let mut parts = http_response.split("\r\n\r\n");
    let header = parts.next().unwrap_or("");
    let content = parts.next().unwrap_or("");

    let status_line = header.lines().next().unwrap_or("");
    let status_code: u16 = status_line
        .split(' ')
        .nth(1)
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    (content.to_owned(), status_code)
}
