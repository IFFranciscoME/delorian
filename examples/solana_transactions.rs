use anyhow::Result;
use delorian_analytics::metrics;
use delorian_data::{clients, files};

#[tokio::main]
async fn main() -> Result<()> {
    // ---------------------------------------------------------------------- Initialization -- //
    // ---------------------------------------------------------------------- -------------- -- //

    let url_sol = "https://api.devnet.solana.com";

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

    // ------------------------------------------------------------ Transaction's Processing -- //
    // ------------------------------------------------------------ ------------------------ -- //

    for i_signature in &v_jito_tx.unwrap()[3..5] {
        println!("\n-- tx_signature: {:?}\n", &i_signature);

        // ---------------------------------------------------------- Priority Fee Estimates -- //
        // ---------------------------------------------------------- ---------------------- -- //

        // the jito_tip_7 address as an example
        let generic_address: &str = v_jito_addresses.as_ref().unwrap()[6].as_ref();

        // ----------------------------------------------------------- Priority Fee Historic -- //
        // ----------------------------------------------------------- --------------------- -- //

        let v_accounts = vec![generic_address.to_string()];
        let pfr_response = s_client.get_priority_fee_recent(v_accounts).await?;
        let pfr_acc_metrics = metrics::pfr_metrics(pfr_response.clone());

        println!("---- pfr_acc_metrics: {:?}", pfr_acc_metrics);
    }

    Ok(())
}
