//! Main
//!

use anyhow::Result;
use delorian_data::{clients, files};
use delorian_analytics::{AnalyticsTable, analyze_tx};

#[tokio::main]
async fn main() -> Result<()> {
    let v_signatures = files::read_cases_json("./data/exp_1_cases.json");

    let tkn_env = "0f34064d-5762-4b3f-af49-319920aa09b9";
    let url_env = "https://mainnet.helius-rpc.com/?api-key=";

    println!("url_env: {:?}", url_env);

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

