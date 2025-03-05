# delorian-analytics

Cases and explorations 

# EDA: Compute Units, Fees and Tips

- get compute units for every retrieved tx
- studies: descriptive stats, outlier detection
- plots: time series, boxplot

- get all the fees and tips (costs) for every retrieved tx
- studies: descriptive stats, outlier detection
- plots: time series, boxplot

# Experiment 1: Compute Units Optimization

Efforts in Compute Units Optimization is always present in an arbitrage transactions.

HO: The value of compute units in Arb tx are below the historical average of 
compute units in other transactions that involved transfers from and to same DEXs.

Data:

- dataset_a: 10 Arb transactions (Jito Explorer)
- dataset_b: 10 Non-arb transactions (pseudo-randomly picked)

# Classification of Arbitrage

1. By Timeframe
    - Atomic Arbitrage: Arbitrage executed within a single blockchain transaction or block.
    - Non-Atomic Arbitrage: Arbitrage spanning across multiple blocks or transactions.

2. By Market Type
    - DEX-to-DEX Arbitrage: Exploiting price differences for the same token across decentralized exchanges.
    - CEX-to-CEX Arbitrage: Arbitrage between centralized exchanges that operate with cryptocurrencies.
    - DEX-to-CEX Arbitrage: Arbitrage between decentralized and centralized exchanges.

3. By Asset Type
    - Single Asset Arbitrage.
    - Multi-Asset Arbitrage (Triangular).

4. By Financial Instrument
    - Spot Market Arbitrage.
    - Yield Arbitrage.
    - Flash Loan Arbitrage.

5. By Tokenized Asset Use Cases
    - Tokenized Real-World Assets (RWAs).
    - Stablecoin Arbitrage.

6. Specialized DeFi Strategies
    - Liquidity Pool Arbitrage.
    - Trade Batching with Flash Loans.


