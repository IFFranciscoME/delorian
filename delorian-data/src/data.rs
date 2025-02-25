//! Data Structures

#![allow(warnings)]

use serde::{Deserialize, Serialize};
use serde_json::json;

// ----------------------------------------------------------------------------- Transaction -- //
// ----------------------------------------------------------------------------- ----------- -- //

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub jsonrpc: String,
    pub result: Option<TransactionResult>,
    pub id: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResult {
    pub blockTime: Option<i64>,
    pub meta: TransactionMeta,
    pub slot: Option<u64>,
    pub transaction: Transaction,
    pub version: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionStatus {
    pub ok: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMeta {
    pub err: Option<serde_json::Value>,
    pub fee: Option<u64>,
    pub pre_balances: Option<Vec<u64>>,
    pub post_balances: Option<Vec<u64>>,
    pub inner_instructions: Option<Vec<InnerInstruction>>,
    pub log_messages: Option<Vec<String>>,
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    pub post_token_balances: Option<Vec<TokenBalance>>,
    pub rewards: Option<Vec<Reward>>,
    pub status: Option<TransactionStatus>,
    pub compute_units_consumed: Option<u64>,
    pub loaded_addresses: Option<LoadedAddresses>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub message: Option<Message>,
    pub signatures: Vec<String>,
}

// --------------------------------------------------------------------------------- Message -- //
// --------------------------------------------------------------------------------- ------- -- //

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub account_keys: Option<Vec<String>>,
    pub header: MessageHeader,
    pub instructions: Option<Vec<Instruction>>,
    pub recent_blockhash: Option<String>,
    pub address_table_lookups: Vec<AddressTableLookup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeader {
    pub num_required_signatures: Option<u8>,
    pub num_readonly_signed_accounts: Option<u8>,
    pub num_readonly_unsigned_accounts: Option<u8>,
}

// ----------------------------------------------------------------------------- Instruction -- //
// ----------------------------------------------------------------------------- ----------- -- //

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub program_id_index: Option<u64>,
    pub accounts: Option<Vec<u64>>,
    pub data: Option<String>,
    pub stack_height: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerInstruction {
    pub index: u8,
    pub instructions: Vec<Instruction>,
}

// ------------------------------------------------------------------------------- Addresses -- //
// ------------------------------------------------------------------------------- --------- -- //

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressTableLookup {
    pub account_key: Option<String>,
    pub writable_indexes: Option<Vec<u8>>,
    pub readonly_indexes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedAddresses {
    pub writable: Vec<String>,
    pub readonly: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub account_index: u8,
    pub mint: String,
    pub owner: String,
    pub program_id: String,
    pub ui_token_amount: UiTokenAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTokenAmount {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<f64>,
    pub ui_amount_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    // Add fields if rewards are used
}

// --------------------------------------------------------------------------- Priority Fees -- //
// --------------------------------------------------------------------------- ------------- -- //

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct priorityFeeEstimateResponse {
    pub jsonrpc: String,
    pub result: Option<priorityFeeEstimateResult>,
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct priorityFeeEstimateResult {
    pub priority_fee_estimate: Option<f64>,
    pub priority_fee_levels: Option<priorityFeeLevels>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct priorityFeeLevels {
    pub min: Option<f64>,
    pub low: Option<f64>,
    pub medium: Option<f64>,
    pub high: Option<f64>,
    pub very_high: Option<f64>,
    pub unsafe_max: Option<f64>,
}
