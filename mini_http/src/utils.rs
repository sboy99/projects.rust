use std::collections::HashMap;

pub fn parse_string_to_hash_map(str: Option<String>) -> Option<HashMap<String, String>> {
    if let Some(str) = str {
        let map = str.split("\n").collect::<Vec<&str>>();
        let map = map
            .iter()
            .map(|m| m.split(": ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let map = map
            .iter()
            .map(|m| (m[0].to_string(), m[1].to_string()))
            .collect::<HashMap<String, String>>();
        Some(map)
    } else {
        None
    }
}
