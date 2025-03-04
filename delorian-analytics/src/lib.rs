//! delorian-analytics
//!
//! Structures and tools for exploratory analytics
//!

use delorian_data::data::TransactionResponse;

/// Data processing methods
pub mod processing;

/// Metrics from Blockchain Data
pub mod metrics;

/// Response Tables
pub mod tables;

pub fn analyze_tx(tx_response: TransactionResponse) -> tables::TxAnalyticsTable {
    let compute_units = tx_response
        .result
        .as_ref()
        .unwrap()
        .meta
        .compute_units_consumed
        .unwrap();

    let fee = tx_response.result.as_ref().unwrap().meta.fee.unwrap();

    let compute_budget_call = tx_response
        .result
        .clone()
        .unwrap()
        .transaction
        .message
        .unwrap()
        .account_keys
        .unwrap()
        .clone();

    let cb_label = String::from("ComputeBudget111111111111111111111111111111");
    let grep_message = processing::grep_messages(&compute_budget_call, &[cb_label]);

    println!("\ngrep_message: {:?}", grep_message);

    let account_keys = tx_response
        .result
        .clone()
        .unwrap()
        .transaction
        .message
        .unwrap()
        .account_keys
        .unwrap();

    let jito_tip_accounts = vec![
        "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5".to_string(),
        "HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe".to_string(),
        "Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY".to_string(),
        "ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49".to_string(),
        "DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh".to_string(),
        "ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt".to_string(),
        "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL".to_string(),
        "T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt".to_string(),
    ];

    let grep_jito_tips = processing::grep_messages(&account_keys, &jito_tip_accounts);
    let jito_tip = if grep_jito_tips.len() != 0 {
        true
    } else {
        false
    };

    println!("\njito tip present: {:?}", jito_tip);

    println!("\ngrep_jito_tips: {:?}", grep_jito_tips);

    // Get the amount of a present Jito Tip Sent
    let tx_jito_tip = tx_response
        .result
        .clone()
        .unwrap()
        .transaction
        .message
        .unwrap()
        .instructions
        .unwrap();

    println!("\ntx_jito_tip: {:?}", tx_jito_tip);

    tables::TxAnalyticsTable {
        compute_units,
        fee,
        compute_budget_call: false,
        net_profit: 1.0,
    }
}
