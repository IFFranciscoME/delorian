
use delorian_data::{files, data::TransactionResponse, data::priorityFeeEstimateResponse};
use crate::processing;
use crate::{analyze_pfe, PfeAnalyticsTable};

#[derive(Debug, Clone)]
pub struct JitoMetricsTable {
    pub tip_found: bool,
    pub tip_amount: u64,
    pub tip_account: String,
}

#[derive(Debug, Clone)]
pub struct CostsMetricsTable {
    pub total_fee: u64,
    pub compute_budget_call: bool,
    pub compute_unit_limit: u32,
    pub compute_unit_consumed: u32,
}

pub fn pfe_metrics(pfe_response: priorityFeeEstimateResponse) -> PfeAnalyticsTable {

    analyze_pfe(pfe_response.result.unwrap().clone())

}

pub fn co_metrics(tx_response: TransactionResponse) -> CostsMetricsTable {

    // Get Total Fee
    let fee = tx_response.result.as_ref().unwrap().meta.fee.unwrap();
    
    // Get compute budget call
    let tx_account_keys = tx_response
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
    let grep_cu_call = processing::grep_messages(&tx_account_keys, &[cb_label]);

    let cu_call = if grep_cu_call.len() != 0 {
        true
    } else { false };
    
    // Get compute unit limit
    let _tx_account_data = tx_response
        .result
        .clone()
        .unwrap()
        .transaction
        .message
        .unwrap()
        .instructions
        .unwrap();

    let cu_limit = 0;
    
    // Get compute unit consumed
    let cu_consumed = tx_response
        .result
        .as_ref()
        .unwrap()
        .meta
        .compute_units_consumed
        .unwrap_or_else(|| 0) as u32;

    CostsMetricsTable {
        total_fee: fee,
        compute_budget_call: cu_call,
        compute_unit_limit: cu_limit,
        compute_unit_consumed: cu_consumed,
    }
}

pub fn jito_metrics(tx_response: TransactionResponse) -> JitoMetricsTable {

    let tx_account_keys = tx_response
        .result
        .clone()
        .unwrap()
        .transaction
        .message
        .unwrap()
        .account_keys
        .unwrap();
    
    // Get all Jito Tips Addresses
    let jito_tips_addresses = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "addresses_jito",
        "wallets",
    ).unwrap_or_default();

    // Grep from transaction's account keys, if present, the Jito Tips Addresses
    let grep_jito = processing::grep_messages(
        &tx_account_keys,
        &jito_tips_addresses);
   
    // Get the Address of a present Jito Tip Sent
    let jito_address = grep_jito.clone().into_iter().map(|value| value.0).collect();

    // Get the amount of a present Jito Tip Sent
    //  TODO: Parse Jito Tip when present

    JitoMetricsTable {
        tip_found: if grep_jito.len() != 0 { true } else { false },
        tip_amount: 0,
        tip_account: jito_address,
    }

}

