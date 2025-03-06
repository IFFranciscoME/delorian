
use anyhow::Result;
use delorian_data::{clients, data};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResult {
    pub block_time: Option<i64>,
    pub meta: Option<data::TransactionMeta>,
    pub slot: Option<u64>,
    pub transaction: Option<data::Transaction>,
    pub version: Option<u8>,
}

#[tokio::main]
async fn main() -> Result<()> {

    let url_sol = "https://api.devnet.solana.com";

    let s_client = clients::SolanaRpcBuilder::new()
        .url(url_sol.to_string())
        .build()
        .expect("Failed to build SolanaRpc");
   
    let r_get_block = s_client.get_block(324824565).await; 
    let block_txs = r_get_block.unwrap().result.unwrap().transactions.unwrap();

    println!("Transactions in block: {:?}", block_txs.len());
    println!("block transaction: {:?}",block_txs[1].transaction);

    Ok(())

}
