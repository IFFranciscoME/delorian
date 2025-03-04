use crate::{processing, tables};
use delorian_data::{
    data::priorityFeeEstimateResponse, data::priorityFeeRecentResponse, data::TransactionResponse,
    decoder, files,
};

pub fn pfr_metrics(pfr_response: priorityFeeRecentResponse) -> tables::PfrMetricsTable {
    // slots: count, min, max
    let slots = pfr_response.slots.unwrap().clone();

    // fees: count, min, max, mean, median
    let mut fees: Vec<u64> = pfr_response.fees.unwrap().clone();
    fees.sort_unstable();

    let count = fees.len();
    let sum: f64 = fees.iter().map(|value| *value as f64).sum();
    let mean = sum / count as f64;
    let median = if count % 2 == 0 {
        (fees[count / 2 - 1] + fees[count / 2]) as f64 / 2.0
    } else {
        fees[count / 2] as f64
    };

    tables::PfrMetricsTable {
        first_slot: *slots.first().unwrap_or(&0) as u64,
        last_slot: *slots.last().unwrap_or(&0) as u64,
        count_slot: count as i64,
        min_fee: *fees.first().unwrap_or(&0) as f64,
        mean_fee: mean,
        median_fee: median,
        max_fee: *fees.last().unwrap_or(&0) as f64,
    }
}

pub fn pfe_metrics(pfe_response: priorityFeeEstimateResponse) -> tables::PfeMetricsTable {
    let pfe = pfe_response.result.unwrap().priority_fee_levels.unwrap();

    tables::PfeMetricsTable {
        min: pfe.min.unwrap_or(0.0),
        low: pfe.low.unwrap_or(0.0),
        medium: pfe.medium.unwrap_or(0.0),
        high: pfe.high.unwrap_or(0.0),
        very_high: pfe.very_high.unwrap_or(0.0),
        unsafe_max: pfe.unsafe_max.unwrap_or(0.0),
    }
}

pub fn co_metrics(tx_response: TransactionResponse) -> tables::CostsMetricsTable {
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

    let cu_data = if grep_cu_call.len() != 0 {
        let cu_call = true;
        let cu_address_dir = 1;
        // println!("cu_address_dir: {:?}", &cu_address_dir);

        (cu_call, cu_address_dir)
    } else {
        let cu_call = false;
        let cu_address_dir = 0;
        // println!("cu_address_dir: {:?}", &cu_address_dir);

        (cu_call, cu_address_dir)
    };

    // Parse indexes from ComputeBudget calls (SetComputeUnitPrice, SetComputeUnitLimit)

    // Get compute unit limit
    let _tx_instructions_data = tx_response
        .result
        .clone()
        .unwrap()
        .transaction
        .message
        .unwrap()
        .instructions
        .unwrap();

    //let hex_data = grep_cu_call[1];
    //let json_result = decoder::decode_base_to_json(hex_data.1);
    //println!("\n-- grep_cu_call: {:?}", grep_cu_call);
    //let json_result = decoder::decode_hex_to_json(hex_data);

    let cu_limit = 0;
    let cu_price = 0;

    // Get compute unit consumed
    let cu_consumed = tx_response
        .result
        .as_ref()
        .unwrap()
        .meta
        .compute_units_consumed
        .unwrap_or_else(|| 0) as u32;

    tables::CostsMetricsTable {
        total_fee: fee,
        compute_budget_call: cu_data.0,
        compute_unit_limit: cu_limit,
        compute_unit_price: cu_price,
        compute_unit_consumed: cu_consumed,
    }
}

pub fn jito_metrics(tx_response: TransactionResponse) -> tables::JitoMetricsTable {
    let tx_account_keys = tx_response
        .result
        .clone()
        .unwrap()
        .transaction
        .message
        .unwrap()
        .account_keys
        .unwrap();

    let tx_message = tx_response
        .result
        .clone()
        .unwrap()
        .transaction
        .message
        .unwrap();

    // Get all Jito Tips Addresses
    let jito_tips_addresses = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "addresses_jito",
        "wallets",
    )
    .unwrap_or_default();

    // Grep from transaction's account keys, if present, the Jito Tips Addresses
    let grep_jito = processing::grep_messages(&tx_account_keys, &jito_tips_addresses);

    // Get the Address of a present Jito Tip Sent
    let jito_address = grep_jito.clone().into_iter().map(|value| value.0).collect();
    let mut v_tip_amounts: Vec<u64> = Vec::new();

    if jito_address != "" {
        println!(
            "\n**** Call to Jito related Address Detected: {:?} ****\n ",
            jito_address
        );

        let jito_call: Vec<usize> = grep_jito.clone()[0].1.clone();

        // println!("---\n tx_message : {:?}\n", tx_message);
        // println!("--- jito_call : {:?}\n", jito_call);

        let _account_call = &tx_message.instructions.as_ref().unwrap()[jito_call[0]]
            .data
            .clone()
            .unwrap();

        for i_instruction in &tx_message.instructions.as_ref().unwrap().clone() {
            if i_instruction
                .clone()
                .accounts
                .unwrap()
                .contains(&(jito_call[0] as u64))
            {
                let base58_data = i_instruction.data.as_ref().unwrap();
                let json_result = decoder::decode_instruction_data(&base58_data);

                v_tip_amounts.push(json_result.unwrap().lamports);
            } else {
            }
        }

        tables::JitoMetricsTable {
            tip_found: if grep_jito.len() != 0 { true } else { false },
            tip_amount: v_tip_amounts[0],
            tip_account: jito_address,
        }
    } else {
        tables::JitoMetricsTable {
            tip_found: false,
            tip_amount: 0,
            tip_account: jito_address,
        }
    }
}
