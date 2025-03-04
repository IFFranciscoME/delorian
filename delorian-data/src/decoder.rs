use hex::encode as hex_encode;
use hex::FromHexError;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedInstruction {
    pub discriminator: u32,
    pub lamports: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferInfo {
    pub source: String,
    pub destination: String,
    pub lamports: u64,
}

pub fn decode_instruction_data(
    encoded_data: &str,
) -> Result<TransferInfo, Box<dyn std::error::Error>> {
    // Decode Base58 string to bytes
    let decoded_data = bs58::decode(encoded_data).into_vec()?;

    let source = "";
    let destination = "";

    // Parse the first 4 bytes as the discriminator (u32)
    let discriminator = u32::from_le_bytes(decoded_data[0..4].try_into()?);

    // Parse the next 8 bytes as lamports (u64)
    let lamports = u64::from_le_bytes(decoded_data[4..12].try_into()?);

    Ok(TransferInfo {
        source: source.to_string(),
        destination: destination.to_string(),
        lamports,
    })
}
