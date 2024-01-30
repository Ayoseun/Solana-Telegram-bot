use reqwest::Client;
use serde::Deserialize;
use std::env;
// Define a struct to represent the Solana JSON response
#[derive(Debug, Deserialize)]
struct SolanaResponse {
    jsonrpc: String,
    result: SolanaResult,
    id: i64,
}

#[derive(Debug, Deserialize)]
struct SolanaResult {
    context: SolanaContext,
    value: u64, // Assuming the "value" field is of type u64
}

#[derive(Debug, Deserialize)]
struct SolanaContext {
    apiVersion: String,
    slot: u64,
}


pub async  fn get_solana_balance(wallet: &str) -> Result<f64, reqwest::Error> {


    let api_url = env::var("SOLANA_API_TOKEN")
        .expect("SOLANA_API_TOKEN not found in environment variables");

  
    let payload = format!(
        r#"
        {{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBalance",
            "params": [
                "{}"
            ]
        }}
        "#,
        wallet
    );

    let client = Client::new();
    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .body(payload)
        .send()
        .await?;

    let solana_response: SolanaResponse = response.json().await?;
    let value = solana_response.result.value as f64 / 1000000000.0;

    Ok(value)
}