# Tests


## Solana RPC calls

The Solana RPC related tests use a utility function for convenience: 

```rust
mod test_client_utils {
    
    use delorian_data::clients::{SolanaRpc, SolanaRpcBuilder};

    pub fn new_solana_client() -> Result<SolanaRpc, Box<dyn std::error::Error>> {

    let url_sol = "https://api.devnet.solana.com";
    let s_client = SolanaRpcBuilder::new()
        .url(url_sol.to_string())
        .build()
        .expect("Failed to build SolanaRpc client");
    
    Ok(s_client)
    }

}
```

### getRecentPrioritizationFees

Since the documentation has a hardcoded value of providing `150` block of historical data, when calling the getRecentPrioritizationFees, this is 
a test precisely to validate this functionality does not change since it might be a breaking change. 

```shell
cargo test --test test_solana_client -- --show-output
```

Should produce the following: 

```shell
---- tests::test_solana_grpf stdout ----

Test:
Solana RPC call to getRecentPrioritizationFees

According to the Docs:
The getRecentPrioritizationFees stores up to 150 blocks


successes:
    tests::test_solana_grpf
```
