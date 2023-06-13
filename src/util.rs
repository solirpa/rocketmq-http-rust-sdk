use reqwest::header::HeaderMap;

#[inline]
pub fn get_canonicalized_mq_headers(headers: &HeaderMap) -> String {
    headers.iter()
        .filter(|(key, _)| key.as_str().starts_with("x-mq-"))
        .map(|(key, value)| format!("{}:{}\n", key, value.to_str().unwrap_or("")))
        .collect()
}
