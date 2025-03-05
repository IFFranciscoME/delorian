# delorian

The overall elements of the project are the following: 

## Tooling

- Repo is a Package with Modular crates, so it can be expanded simultaneously and as needed by two or more devs.
- Docs, Tests, Benches for better dev practices and maintainability.
- Error definition and handling.

## Data: 

- Curated dataset, from Jito Arb Explorer, ancillary addresses and cases for experiment design.
- Helius RPC client for PriorityFee estimated values.
- Solana RPC client for PriorityFee historical values.
- Wide reach of types definition (close to exhaustive).
- Decoding-Deserialization for the encoded & serialized data on the instructions.

## Process:

- Async client connection for both sources.

## Analytics: 

- Data parsing for metrics calculations.
- Silver tables to build timeseries and/or storage in FileSystems or DBs
- Jito Tips Activity Detection
- Costs Optimization Activity Detection
- Priority Fees (for a particular account) Estimate Stats
- Priority Fees (for a particular account) Historical Values

