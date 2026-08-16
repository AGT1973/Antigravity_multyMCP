use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

pub async fn execute_openai_compatible(
    url: &str,
    model: &str,
    api_key: &str,
    prompt: &str,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    
    let request_body = json!({
        "model": model,
        "messages": [
            {"role": "user", "content": prompt}
        ],
        "temperature": 0.0
    });

    let res = client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let status = res.status();
    
    // Si hay rate limit (429), lanzamos error explícito para que el circuit breaker lo cache.
    if status.as_u16() == 429 {
        return Err("HTTP 429: Too Many Requests / Quota Exhausted".into());
    }
    
    if !status.is_success() {
        let err_text = res.text().await?;
        return Err(format!("HTTP Error {}: {}", status, err_text).into());
    }

    let json_res: Value = res.json().await?;
    let content = json_res["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Empty response")
        .to_string();

    Ok(content)
}
