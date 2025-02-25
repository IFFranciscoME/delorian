//! delorian-analytics
//!
//! Structures and tools for exploratory analytics
//!

/// Data processing methods
pub mod processing;

use delorian_data::data::{TransactionResponse, priorityFeeEstimateResult};

#[derive(Debug, Clone)]
pub struct PfeAnalyticsTable {
    pub min: f64,
    pub low: f64,
    pub medium: f64,
    pub high: f64,
    pub very_high: f64,
    pub unsafe_max: f64,
}

#[derive(Debug, Clone)]
pub struct TxAnalyticsTable {
    pub compute_units: u64,
    pub fee: u64,
    pub compute_budget_call: bool,
    pub net_profit: f64,
}

pub fn analyze_tx(
    tx_response: TransactionResponse, 
    ) -> TxAnalyticsTable {

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

    TxAnalyticsTable {
        compute_units,
        fee,
        compute_budget_call: grep_message,
        net_profit: 1.0,
    }

}

pub fn analyze_pfe(
    pfe_response: priorityFeeEstimateResult
    ) -> PfeAnalyticsTable {

    let pfe = pfe_response.priority_fee_levels.unwrap();

    PfeAnalyticsTable {
        min: pfe.min.unwrap(),
        low: pfe.low.unwrap(),
        medium: pfe.medium.unwrap(),
        high: pfe.high.unwrap(),
        very_high: pfe.very_high.unwrap(),
        unsafe_max: pfe.unsafe_max.unwrap(),
    }

}

