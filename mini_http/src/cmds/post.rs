use anyhow::Result;

use crate::http;
use crate::printer;
use crate::utils::parse_string_to_hash_map;

pub async fn post(url: String, body: Option<String>, headers: Option<String>) -> Result<()> {
    let http_client = http::HttpClient::new(url, None);
    let response = http_client
        .post(
            "",
            parse_string_to_hash_map(body),
            parse_string_to_hash_map(headers),
        )
        .await?;
    printer::print_response(&response.text().await?);
    Ok(())
}
