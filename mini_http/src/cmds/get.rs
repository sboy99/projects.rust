use crate::http;
use crate::utils::parse_headers;
use anyhow::Result;

pub async fn get(url: String, headers: Option<String>) -> Result<()> {
    let http_client = http::HttpClient::new(url, None);
    let response = http_client.get("", parse_headers(headers), None).await?;
    println!("Response: {}", response.text().await?);
    Ok(())
}
