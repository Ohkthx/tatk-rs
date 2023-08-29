<p align="center">
    <a href="https://github.com/Ohkthx/tatk-rs#tips-appreciated" title="Donate with Bitcoin!">
        <img src="https://img.shields.io/badge/donate-black?style=for-the-badge&logo=bitcoin&logoColor=f38ba8&label=BITCOIN&labelColor=11111b&color=f38ba8"
            alt="Donate with Bitcoin!"></a>
    <a href="https://github.com/Ohkthx/tatk-rs#tips-appreciated" title="Donate with Ethereum!">
        <img src="https://img.shields.io/badge/donate-black?style=for-the-badge&logo=ethereum&logoColor=fab387&label=ETHEREUM&labelColor=11111b&color=fab387"
            alt="Donate with Ethereum!"></a>
    <a href="https://github.com/Ohkthx/tatk-rs#tips-appreciated" title="Donate with BNB (Binance)!">
        <img src="https://img.shields.io/badge/donate-black?style=for-the-badge&logo=binance&logoColor=f9e2af&label=BINANCE&labelColor=11111b&color=f9e2af"
            alt="Donate with BNB!"></a>
</p>

# TATK, Technical Analysis Toolkit

The objective of this crate is analyzing signals and indicators of data and providing additional tools for accurate analysis. There are several other wonderful crates that implement similar algorithms, however they did not meet my personal needs or had minor errors in their calculations.

```toml
[dependencies]
tatk = { git = "https://github.com/ohkthx/tatk-rs" }
```

## Features
- **Moving Averages**
  - Simple Moving Average (SMA)
  - Exponential Moving Average (EMA)
  - Double Exponential Moving Average (DEMA)
- **Oscillators**
  - Relative Strength Index (RSI)
  - Moving Average Convergence and Divergence (MACD)
- **Others**
  - Variance (Var(X))
  - Standard Deviation (SD/STDEV)

## Documentation

- None.

### TODO

- Add additional indicators and test.

## Examples

- **Simple Moving Average (SMA)**: [sma.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/sma.rs)
  - `cargo run --example sma --all-features`
- **Exponential Moving Average (EMA)**: [ema.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/ema.rs)
  - `cargo run --example ema --all-features`
- **Double Exponential Moving Average (DEMA)**: [dema.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/dema.rs)
  - `cargo run --example dema --all-features`
- **Moving Average Convergence Divergence (MACD)**: [macd.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/macd.rs)
  - `cargo run --example macd --all-features`

## Tips Appreciated!

Wallet addresses are provided below, or click the badges above!
```
Ethereum (ETH): 0x7d75f6a9c021fcc70691fec73368198823fb0f60
Bitcoin (BTC):  bc1q75w3cgutug8qdxw3jlmqnkjlv9alt3jr7ftha0
Binance (BNB):  0x7d75f6a9c021fcc70691fec73368198823fb0f60
```
