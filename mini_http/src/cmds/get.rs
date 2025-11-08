use anyhow::Result;

use crate::http;
use crate::printer;
use crate::utils::parse_string_to_hash_map;

pub async fn get(url: String, headers: Option<String>) -> Result<()> {
    let http_client = http::HttpClient::new(url, None);
    let response = http_client
        .get("", parse_string_to_hash_map(headers), None)
        .await?;
    printer::print_response(&response.text().await?);
    Ok(())
}
