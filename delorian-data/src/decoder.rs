use serde::{Serialize, Deserialize};
use serde_json::json;
use hex::FromHexError;
use borsh::{BorshDeserialize, BorshSerialize};
use bs58::{decode, encode};
use hex::encode as hex_encode;

#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize)]
pub struct DataInfo {
    #[serde(rename = "type")]
    pub data_type: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize)]
pub struct InstructionType {
    pub discriminator: DataInfo,
    pub lamports: DataInfo,
}

pub fn decode_instruction_data(encoded_data: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Decode Base58 string to bytes
    let decoded_data = bs58::decode(encoded_data).into_vec()?;

    // Attempt Borsh deserialization
    let instruction: InstructionType = BorshDeserialize::try_from_slice(&decoded_data[..])?;

    // Serialize to JSON
    let json_output = serde_json::to_string(&instruction)?;
    Ok(json_output)
}
