use std::collections::HashMap;

pub fn parse_headers(headers: Option<String>) -> Option<HashMap<String, String>> {
    if let Some(headers) = headers {
        let headers = headers.split("\n").collect::<Vec<&str>>();
        let headers = headers
            .iter()
            .map(|h| h.split(": ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let headers = headers
            .iter()
            .map(|h| (h[0].to_string(), h[1].to_string()))
            .collect::<HashMap<String, String>>();
        Some(headers)
    } else {
        None
    }
}
