//! Files I/O

use delorian_results::errors::FileError;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use thiserror::Error;

// -------------------------------------------------------------------------------------------- //
// -------------------------------------------------------------------------------------------- //

pub fn read_json(json_file: &str, cases: &str, sub_case: &str) -> Result<Vec<String>, FileError> {
    // Json file
    let mut file = File::open(json_file)?;

    // Read the file's content into a single string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the Json string into a value
    let json_value: Value = serde_json::from_str(&contents)?;

    let mut v_entries = vec![];
    
    match cases {
        "tx_arbs_jito" => {
            // Get corresponding fields
            if let Some(tx_arbs_jito) = json_value["tx_arbs_jito"].as_object() {
                if let Some(tx_signature) = tx_arbs_jito["tx_signature"].as_array() {
                    for i_signature in tx_signature {
                        if let Some(signature) = i_signature.as_str() {
                            let cleaned_signature = signature.trim_matches('"').replace("\\", "");
                            v_entries.push(cleaned_signature);
                        }
                    }
                    Ok(v_entries)
                } else {
                    println!("tx_signature not present");
                    Ok(v_entries)
                }
            } else {
                println!("tx_arbs_jito not present");
                Ok(v_entries)
            }
        },
        "tx_arbs_suspected" => {
            if let Some(tx_arbs_suspected) = json_value["tx_arbs_suspected"].as_object() {
                if let Some(tx_signature) = tx_arbs_suspected["tx_signature"].as_array() {
                    for i_signature in tx_signature {
                        if let Some(signature) = i_signature.as_str() {
                            let cleaned_signature = signature.trim_matches('"').replace("\\", "");
                            v_entries.push(cleaned_signature);
                        }
                    }
                    Ok(v_entries)
                } else {
                    println!("tx_signature not present");
                    Ok(v_entries)
                }
            } else {
                println!("tx_arbs_suspected not present");
                Ok(v_entries)
            }
        },

        "addresses_dex" => {
            if let Some(addresses_dex) = json_value["addresses_dex"].as_object() {
                if let Some(sub_cases) = addresses_dex[sub_case].as_object() {
                    let sub_cases_values: Vec<String> = sub_cases
                        .values()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect();
                    v_entries.extend(sub_cases_values);
                    Ok(v_entries)
                } else {
                    println!("{:?} is not present", sub_case);
                    Ok(v_entries)
                }
            } else {
                println!("addresses_dex not present");
                Ok(v_entries)
            }
        }
        _ => {
            println!("Unknown case: {}", cases);
            Ok(v_entries)
        }
    }
}

