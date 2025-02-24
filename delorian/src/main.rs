//! Main
//!

use anyhow::Result;
use delorian_analytics::{analyze_tx, AnalyticsTable};
use delorian_data::{clients, files};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let tkn_env = match env::var("HELIUS_TOKEN") {
        Ok(h_tk) => h_tk.to_string(),
        Err(_e) => "".to_string(),
    };

    let url_env = "https://mainnet.helius-rpc.com/?api-key=";

    let v_signatures = files::read_cases_json("./data/exp_1_cases.json");

    let h_client = clients::HeliusRpcBuilder::new()
        .url(url_env.to_string())
        .tkn(tkn_env.to_string())
        .build()
        .expect("Failed to build HeliusRpc");

    for i_signature in v_signatures.unwrap() {
        let tx_response = h_client.get_tx(&i_signature).await?;
        let analytics_table: AnalyticsTable = analyze_tx(tx_response.clone());

        println!("\n -- tx_signature: {:?} -- \n", i_signature);
        println!("Analytics Table: {:?}", analytics_table);
    }

    Ok(())
}
