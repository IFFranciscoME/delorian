# examples

## Deserializing data

```shell
cargo run --example deserializing
```

and this will produce the following: 

```
----
Decoded and Deserialized Data: ExampleData { discriminator: 2, units: 644586 }
----
```

## Solana Transactions Parsing

```shell
cargo run --example solana_transactions
```

which should produce this output

```
-- tx_signature: "3mJCiys9UiSBMMJhntRAikEwRDDcno31MEzPHmAHBs89QH9i6axbxWZhkgQyB4mVcQ81tW8SpDgAADazvZ4jPtwi"

---- pfr_acc_metrics: PfrMetricsTable { first_slot: 365335292, last_slot: 365335441, count_slot: 150, min_fee: 0.0, mean_fee: 0.0, median_fee: 0.0, max_fee: 0.0 }

-- tx_signature: "47NRTuV5GTHyZQ4YCEREi5YoS3ysKdQF2Zav2xXHPp2uUQPRmFGaoCbowipSRCnm8Sqn7hrM2cvTMuox9dpVFMbq"

---- pfr_acc_metrics: PfrMetricsTable { first_slot: 365335293, last_slot: 365335442, count_slot: 150, min_fee: 0.0, mean_fee: 0.0, median_fee: 0.0, max_fee: 0.0 }
```
