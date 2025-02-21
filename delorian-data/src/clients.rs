use crate::data::TransactionResponse;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

pub async fn get_transaction(tx_signature: &str) -> Result<TransactionResponse> {
    let client = Client::new();

    let tk1 = "0f34064d-5762-4b3f-af49-319920aa09b9";
    let url = "https://mainnet.helius-rpc.com/?api-key=".to_owned() + tk1;

    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": [
            tx_signature,
            {
                "maxSupportedTransactionVersion": 0,
            }
        ]
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .context("Failed to send RPC request")?;

    let tx_response: TransactionResponse = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    Ok(tx_response)
}
