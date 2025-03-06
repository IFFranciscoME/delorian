/// Experiment: Priority Fees Gap

use anyhow::Result;
use delorian_data::{clients, files};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {

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

    let v_dexes_raydium = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "addresses_dex",
        "raydium",
    );

    let v_dexes_orca = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "addresses_dex",
        "orca",
    );

    let array_raydium:Vec<String> = v_dexes_raydium.unwrap().clone();
    let pfr_response_raydium = s_client.get_priority_fee_recent(array_raydium.clone()).await?;
    let pfe_response_raydium = h_client.get_priority_fee_estimate(array_raydium.clone()).await?;

    println!("\n----\n WSOL_USDC DEX in raydium: \n {:?}", array_raydium[3]);
    println!("\nRecent Priority Fees: \n{:?}", pfr_response_raydium.fees.unwrap());
    println!("\nEstimate Priority Fees: \n{:?}\n----", pfe_response_raydium
        .result.unwrap().priority_fee_levels.unwrap());
    
    let array_orca:Vec<String> = v_dexes_orca.unwrap().clone();
    let pfr_response_orca = s_client.get_priority_fee_recent(array_orca.clone()).await?;
    let pfe_response_orca = h_client.get_priority_fee_estimate(array_orca.clone()).await?;

    println!("\n----\n WSOL_USDC DEX in orca: \n {:?}", array_orca[2]);
    println!("\nRecent Priority Fees: \n{:?}\n----", pfr_response_orca.fees.unwrap());
    println!("\nEstimate Priority Fees: \n{:?}\n----", pfe_response_orca
        .result.unwrap().priority_fee_levels.unwrap());
    
    Ok(())

}

