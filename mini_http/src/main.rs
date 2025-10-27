mod cli;
mod http;
mod printer;

use anyhow::Result;
use http::HttpClient;

#[tokio::main]
async fn main() -> Result<()> {
    let http_client = HttpClient::new(String::from("https://jsonplaceholder.typicode.com"), None);
    let response = http_client.get("/posts/1", None).await?;
    println!("Response: {}", response.text().await?);
    Ok(())
}
