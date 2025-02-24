//! Files I/O

use delorian_results::errors::FileError;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use thiserror::Error;

// -------------------------------------------------------------------------------------------- //
// -------------------------------------------------------------------------------------------- //

pub fn read_cases_json(json_file: &str) -> Result<Vec<String>, FileError> {
    // Json file
    let mut file = File::open(json_file)?;

    // Read the file's content into a single string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the Json string into a value
    let json_value: Value = serde_json::from_str(&contents)?;

    // Get corresponding fields
    let mut v_signatures = vec![];
    if let Some(jito_arbs) = json_value["jito_arbs"].as_object() {
        if let Some(tx_signatures) = jito_arbs["tx_signature"].as_array() {
            for i_signature in tx_signatures {
                if let Some(signature) = i_signature.as_str() {
                    let cleaned_signature = signature.trim_matches('"').replace("\\", "");
                    v_signatures.push(cleaned_signature);
                }
            }
        }
        //  TODO: parse logic for other field types
    }
    Ok(v_signatures)
}
