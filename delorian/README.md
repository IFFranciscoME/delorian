# delorian

A series of data definitions, tools, results, stats and notes: 

1. Experiments Data: Curated from Jito Arbs Explorer (Accounts, Transactions)
2. Data I/O, Types: Methods for JSON-RPC (Helius, Solana), Extensive Types Mapping.
3. Jito Activity Detection: Tips, Accounts.
4. Cost Optimization Detection: Compute Budget (Fees, Units and Price).
5. Priority Fees Historical and Estimates: from Solana and Helius Stats
6. Arbitrage Conceptual definitions. 

# Experiment: PriorityFees 

## Data sources 

1. Solana [getRecentPrioritizationFees](https://solana.com/docs/rpc/http/getrecentprioritizationfees) 
2. Helius [getPriorityFeeEstimate](https://docs.helius.dev/solana-apis/priority-fee-api)

The Solana RPC method returns a list of prioritization fees from recent blocks. Currently, a node's prioritization-fee cache stores
data up to 150 blocks.

When an array of account addresses is provided, which supports up to 128 addresses, the response will reflect a fee needed to have a 
transaction locking all of the provided accounts as if they were writable.

The Heliyus RPC method returns a recommendation on priority fee levels. Methodology is documented. 

```Rust
use anyhow::Result;
use delorian_data::{clients, files};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {

    let url_env = "https://mainnet.helius-rpc.com/?api-key=";
    let url_sol = "https://api.devnet.solana.com";

    let tkn_env = match env::var("HELIUS_TOKEN") {
        Ok(h_tk) => h_tk.to_string(),
        Err(_e) => "".to_string(),
    };

    let h_client = clients::HeliusRpcBuilder::new()
        .url(url_env.to_string())
        .tkn(tkn_env.to_string())
        .build()
        .expect("Failed to build HeliusRpc");

    let s_client = clients::SolanaRpcBuilder::new()
        .url(url_sol.to_string())
        .build()
        .expect("Failed to build SolanaRpc");

    // ---------------------------------------------------------------------------- Accounts -- //
    // ---------------------------------------------------------------------------- -------- -- //

    let v_jito_addresses = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "addresses_jito",
        "wallets",
    );

    let v_dexes_raydium = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "addresses_dex",
        "raydium",
    );

    let v_dexes_orca = files::read_json(
        "./assets/datasets/exp_1_cases.json",
        "addresses_dex",
        "orca",
    );

    let array_raydium:Vec<String> = v_dexes_raydium.unwrap().clone();
    let pfr_response_raydium = s_client.get_priority_fee_recent(array_raydium.clone()).await?;
    let pfe_response_raydium = h_client.get_priority_fee_estimate(array_raydium.clone()).await?;

    println!("\n----\n WSOL_USDC DEX in raydium: \n {:?}", array_raydium[3]);
    println!("\nRecent Priority Fees: \n{:?}", pfr_response_raydium.fees.unwrap());
    println!("\nEstimate Priority Fees: \n{:?}\n----", pfe_response_raydium
        .result.unwrap().priority_fee_levels.unwrap());
    
    let array_orca:Vec<String> = v_dexes_orca.unwrap().clone();
    let pfr_response_orca = s_client.get_priority_fee_recent(array_orca.clone()).await?;
    let pfe_response_orca = h_client.get_priority_fee_estimate(array_orca.clone()).await?;

    println!("\n----\n WSOL_USDC DEX in orca: \n {:?}", array_orca[2]);
    println!("\nRecent Priority Fees: \n{:?}\n----", pfr_response_orca.fees.unwrap());
    println!("\nEstimate Priority Fees: \n{:?}\n----", pfe_response_orca
        .result.unwrap().priority_fee_levels.unwrap());
    
    Ok(())

}
```

The results show some differences in the Priority Fee Estimate, provided by Helius, and the Priority Fee Recent, provided by Solana. Which in a way makes sense, the most recent 150 blocks has not shown any priority fees included in transactions that involved the DEX, which in this example is `WSOL_USDC`. 

```shell
----
 WSOL_USDC DEX in raydium: 
 "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2"

Recent Priority Fees: 
[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

Estimate Priority Fees: 
priorityFeeLevels { min: Some(0.0), low: Some(0.0), medium: Some(0.0), high: Some(21001.0), very_high: Some(2000000.0), unsafe_max: Some(666666666.0) }
----

----
 WSOL_USDC DEX in orca: 
 "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE"

Recent Priority Fees: 
[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
----

Estimate Priority Fees: 
priorityFeeLevels { min: Some(0.0), low: Some(0.0), medium: Some(0.0), high: Some(21001.0), very_high: Some(7248426.0), unsafe_max: Some(666666666.0) }
----

```

From this we can derive some further ideas: 

1. Calculating an estimate of the priority fees definitel involve more than the last 150 blocks.
2. Given that more history is used for the estimate, we need to account for time series statistics, e.g. Mean reversion, Heteroskedasticity, Stationarity, etc. 
3. Priority Fees are included per transaction, and, are determined in a "DAG" structure given the venues (node) and transfers/actions (edges) and amounts (weights on edges)

## Further work 

- Use a DAG with weights approach: Programs (Venues, systems program, etc) are the nodes, computations are the edges, amounts in tokens or compute units (weights on edges)
- Create a data pipeline to fetch priority fees as they are submitted, with high priority on completness. 
