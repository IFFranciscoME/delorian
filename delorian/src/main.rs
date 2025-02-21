//! Main
//!

use anyhow::Result;
use delorian_data::{clients, files, processing};

#[tokio::main]
async fn main() -> Result<()> {
    let v_signatures = files::read_cases_json("./data/exp_1_cases.json");

    for i_signature in v_signatures.unwrap() {
        let tx_response = clients::get_transaction(&i_signature).await?;

        let compute_units = tx_response
            .result
            .as_ref()
            .unwrap()
            .meta
            .compute_units_consumed;

        let log_messages = tx_response
            .result
            .as_ref()
            .unwrap()
            .meta
            .log_messages
            .clone();

        let grep_msg = processing::grep_messages(&log_messages.as_ref().unwrap());

        let fees = tx_response.result.as_ref().unwrap().meta.fee;

        println!("\n -- tx_signature: {:?} -- \n", i_signature);
        println!(
            "Compute Units: {:?}\nFees: {:?}\ncompute_budget calls: {:?}",
            compute_units,
            fees,
            grep_msg.len()
        );

        let instruction_data = tx_response
            .result
            .unwrap()
            .transaction
            .message
            .unwrap()
            .instructions
            .unwrap();

        println!("\ninstructions: {:?}\n", instruction_data);
    }

    Ok(())
}
