use crate::data::{priorityFeeEstimateResponse, TransactionResponse};
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct HeliusRpc {
    pub url: String,
    pub tkn: String,
}

pub struct HeliusRpcBuilder {
    url: Option<String>,
    tkn: Option<String>,
}

impl HeliusRpcBuilder {
    pub fn new() -> Self {
        HeliusRpcBuilder {
            url: None,
            tkn: None,
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn tkn(mut self, tkn: String) -> Self {
        self.tkn = Some(tkn);
        self
    }

    pub fn build(self) -> Result<HeliusRpc, String> {
        match (self.url, self.tkn) {
            (Some(url), Some(tkn)) => Ok(HeliusRpc { url, tkn }),
            _ => Err("Both URL and token must be provided".to_string()),
        }
    }
}

impl HeliusRpc {
    pub async fn get_tx(&self, tx_signature: &str) -> Result<TransactionResponse> {
        let helius_client = Client::new();
        let url = format!("{}{}", self.url, self.tkn);

        let tx_request = json!({
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

        let helius_response = helius_client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&tx_request)
            .send()
            .await
            .context("Failed to send RPC request to Helius")?;

        let tx_response: TransactionResponse = helius_response
            .json()
            .await
            .context("Failed to parse Helius response JSON data")?;

        Ok(tx_response)
    }
}

// -------------------------------------------------------------------------------------------- //
// -------------------------------------------------------------------------------------------- //

// -------------------------------------------------------------------------------------------- //
// -------------------------------------------------------------------------------------------- //

pub async fn get_priority_fee_estimate(tx_hash: &str) -> Result<priorityFeeEstimateResponse> {
    let client = Client::new();
    let tk1 = "0f34064d-5762-4b3f-af49-319920aa09b9";
    let url = "https://mainnet.helius-rpc.com/?api-key=".to_owned() + tk1;

    let request = json!({
        "jsonrpc": "2.0",
        "id": "1",
        "method": "getPriorityFeeEstimate",
        "params": [
            {
                "accountKeys": ["So11111111111111111111111111111111111111112"],
                "options": { "includeAllPriorityFeeLevels": true }
            },
        ],
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .context("Failed to send RPC request")?;

    // println!("response: {:?}", response);

    let priority_fee_response: priorityFeeEstimateResponse = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    Ok(priority_fee_response)
}

pub fn get_accouts() {
    println!("placeholder")
}
