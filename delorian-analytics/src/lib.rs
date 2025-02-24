//! delorian-analytics
//!
//! Structures and tools for exploratory analytics
//!

/// Data processing methods
pub mod processing;

use delorian_data::data::TransactionResponse;

#[derive(Debug, Clone)]
pub struct AnalyticsTable {
    pub compute_units: u64,
    pub fee: u64,
    pub compute_budget_call: bool,
    pub net_profit: f64,
}

pub fn analyze_tx(tx_response: TransactionResponse) -> AnalyticsTable {
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

    let grep_message = processing::grep_messages(&compute_budget_call);

    AnalyticsTable {
        compute_units,
        fee,
        compute_budget_call: grep_message,
        net_profit: 1.0,
    }
}
