//! Main

use anyhow::Result;
use delorian_analytics::{analyze_tx, analyze_pfe, TxAnalyticsTable, PfeAnalyticsTable};
use delorian_data::{clients, files};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {

    let url_env = "https://mainnet.helius-rpc.com/?api-key=";
    let tkn_env = match env::var("HELIUS_TOKEN") {
        Ok(h_tk) => h_tk.to_string(),
        Err(_e) => "".to_string(),
    };

    let h_client = clients::HeliusRpcBuilder::new()
        .url(url_env.to_string())
        .tkn(tkn_env.to_string())
        .build()
        .expect("Failed to build HeliusRpc");

    // -- Priority Fee Estimates -- //
    let v_accounts = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "addresses_dex",
        "raydium"
    );

    let i_account = &v_accounts?[0];
    let pfe_response = h_client.get_priority_fee_estimate(i_account).await?;
    let pfe_table: PfeAnalyticsTable = analyze_pfe(
        pfe_response.result.unwrap().clone(),
    );

    println!("\naccount: {:?}, \npfe_table: {:?}", i_account, pfe_table);
    
    // -- Transactions -- // 
    let v_signatures = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "tx_arbs_jito",
        ""
    );
    
    for i_signature in v_signatures.unwrap() {

        let tx_response = h_client.get_tx(&i_signature).await?;
        let analytics_table: TxAnalyticsTable = analyze_tx(
            tx_response.clone(),
        );

        println!("\n -- tx_signature: {:?} -- \n", i_signature);
        println!("Analytics Table: {:?}", analytics_table);
     
    }
    
    Ok(())

}
