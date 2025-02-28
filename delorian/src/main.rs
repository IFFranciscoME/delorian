//! Main

use anyhow::Result;
use delorian_analytics::metrics;
use delorian_data::{clients, files};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    
    // ---------------------------------------------------------------------- Initialization -- //
    // ---------------------------------------------------------------------- -------------- -- //
    
    let url_env = "https://mainnet.helius-rpc.com/?api-key=";
    let url_sol = "https://api.devnet.solana.com";

    let tkn_env = match env::var("HELIUS_TOKEN") {
        Ok(h_tk) => h_tk.to_string(),
        Err(_e) => "".to_string(),
    };

    let h_client = clients::HeliusRpcBuilder::new()
        .url(url_env.to_string())
        .tkn(tkn_env.to_string())
        .build()
        .expect("Failed to build HeliusRpc");

    let s_client = clients::SolanaRpcBuilder::new()
        .url(url_sol.to_string())
        .build()
        .expect("Failed to build SolanaRpc");

    // ---------------------------------------------------------------------------- Accounts -- //
    // ---------------------------------------------------------------------------- -------- -- //
    
    let v_jito_tx = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "tx_arbs_jito",
        "tx_signature",
    );

    let v_jito_addresses = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "addresses_jito",
        "wallets",
    );
        
    // --------------------------------------------------------- Recent Prioritization Fees -- //
    // --------------------------------------------------------- -------------------------- -- //
    
    let generic_address = v_jito_addresses.as_ref().unwrap()[6].clone();
    let v_accounts = vec![generic_address];
    let solana_response = s_client.get_rpf(v_accounts).await?;

    println!("---- rpf_metrics: {:?}", solana_response.result.unwrap());
    
    // ----------------------------------------------------------------- Transaction's Data -- //
    // ----------------------------------------------------------------- ------------------ -- //
   
    for i_signature in v_jito_tx.unwrap() {

        println!("\n-- tx_signature: {:?} --- \n", &i_signature);

        // -------------------------------------------------------------- Transaction's Data -- //
        // -------------------------------------------------------------- ------------------ -- //
       
        // Each Arb positive transaction from the Jito Arb explorer
        let tx_response = h_client.get_tx(&i_signature).await?;
        let tx_jito_metrics = metrics::jito_metrics(tx_response.clone());
        let tx_co_metrics = metrics::co_metrics(tx_response.clone());

        // ---------------------------------------------------------- Priority Fee Estimates -- //
        // ---------------------------------------------------------- ---------------------- -- //

        // the jito_tip_7 address as an example
        let generic_address = v_jito_addresses.as_ref().unwrap()[6].as_ref();
        let pfe_response = h_client.get_priority_fee_estimate(&generic_address).await?;
        let pfe_acc_metrics = metrics::pfe_metrics(pfe_response);

        println!("---- tx_jito_metrics: {:?}", tx_jito_metrics);
        println!("---- tx_co_metrics: {:?}", tx_co_metrics);
        println!("---- pfe_acc_metrics: {:?}", pfe_acc_metrics);
     
    }

    Ok(())
    

}

