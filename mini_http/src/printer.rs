use colored::*;
use serde_json::Value;

pub fn print_response(body: &str) {
    if let Ok(json) = serde_json::from_str::<Value>(body) {
        println!("{}", serde_json::to_string_pretty(&json).unwrap().green());
    } else {
        println!("{}", body.yellow());
    }
}
