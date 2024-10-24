use anyhow::{Context, Result};

pub async fn fetch_url(url: &str) ->Result<String> {
    let response = reqwest::get(url).await.with_context(|| format!("Failed to reach {}", url))?;

    // ensure we got a succesfull response
    if response.status().is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        Err(anyhow::anyhow!("Failed to fetch URL {}: Status {}", url, response.status())) // Example with predefined status
    }
}