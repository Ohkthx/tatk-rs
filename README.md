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
<br>
    <a href="https://crates.io/crates/tatk" title="crates.io version.">
        <img src="https://img.shields.io/crates/v/tatk?style=for-the-badge&logoColor=89b4fa&labelColor=11111b&color=89b4fa"
            alt="crates.io version"></a>
    <a href="https://crates.io/crates/tatk" title="crates.io download counter.">
        <img src="https://img.shields.io/crates/d/tatk?style=for-the-badge&logoColor=89dceb&labelColor=11111b&color=89dceb"
            alt="crates.io downloads"></a>
    <a href="https://github.com/ohkthx/tatk-rs" title="Size of the repo!">
        <img src="https://img.shields.io/github/repo-size/Ohkthx/tatk-rs?style=for-the-badge&logoColor=a6e3a1&labelColor=11111b&color=a6e3a1"
</p>

# TATK, Technical Analysis Toolkit

The objective of this crate is analyzing signals and indicators of data and providing additional tools for accurate analysis. There are several other wonderful crates that implement similar algorithms, however they did not meet my personal needs or had minor errors in their calculations.

```toml
[dependencies]
tatk = { git = "https://github.com/ohkthx/tatk-rs" }
```

## Features
- **Momentum / Moving Averages**
  - Simple Moving Average (SMA)
  - Exponential Moving Average (EMA)
  - Double Exponential Moving Average (DEMA)
  - McGinley Dynamic Indicator (MD)
  - On-Balance Volume (OBV)
  - Rate of Change (ROC)
- **Oscillators**
  - Relative Strength Index (RSI)
  - Moving Average Convergence and Divergence (MACD)
  - Bollinger Bands (BBands)
  - True Range (TR)
  - Average True Range (ATR)
- **Others**
  - Linear Regression (LineReg)
  - Variance (Var(X))
  - Standard Deviation (SD/STDEV)
  - Cross (Cross), checks two lines for Golden or Death cross.

## Documentation

Most of the documentation can be accessed by clicking the following link: [docs.rs](https://docs.rs/tatk/latest/tatk/). That documentation is automatically generated and also accessible from [crates.io](https://crates.io/crates/tatk).

### Traits

The following traits are either used by the crate on indicators or to be defined by the user for additional functionality.

- **Indicator**
  - Stats - Basic statistics for the indicator such as: sum, mean, variance, and standard deviation.
  - Period - Period of window of the data for the indicator.
  - Value - Current value held by the indicator.
  - Next - Add a new data point to the indicator to recalculate value.
- **User Defined**
  - AsValue - Alternative value that can be passed to an Indicators `Next`.
  - Open - Opening value for the data type.
  - Close - Closing value for the data type.
  - Low - Lowest value for the data type.
  - High - Highest value for the data type.
  - Volume - Total volume for the data type.
- **Others**
  - HL2 - Average of the Highest and Lowest values, requires `High` and `Low` to be defined.
  - HLC3 - Average of the Highest, Lowest, and Close values, requires `High`, `Low`, and `Close` to be defined.
  - OHLC4 - Average of the Open, Highest, Lowest, and Close values, requires `Open`, `High`, `Low`, and `Close` to be defined.

## Examples

Following examples can be ran with:  `cargo run --example short_id`

- **Simple Moving Average (SMA)**: [sma.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/sma.rs)
- **Exponential Moving Average (EMA)**: [ema.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/ema.rs)
- **Double Exponential Moving Average (DEMA)**: [dema.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/dema.rs)
- **Moving Average Convergence Divergence (MACD)**: [macd.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/macd.rs)
- **Relative Strength Indicator (RSI)**: [rsi.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/rsi.rs)
- **Bollinger Bands (BBands)**: [bbands.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/bbands.rs)
- **Average True Range (ATR)**: [atr.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/atr.rs)
- **McGinley Dynamic Indicator (MD)**: [md.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/md.rs)
- **On-Balance Volume (OBV)**: [obv.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/obv.rs)
- **Rate of Change (ROC)**: [roc.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/roc.rs)
- **Linear Regression (LineReg)**: [linereg.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/linereg.rs)
- **Traits (Traits)**: [user_traits.rs](https://github.com/Ohkthx/tatk-rs/tree/main/examples/user_traits.rs)

## Tips Appreciated!

Wallet addresses are provided below, or click the badges above!
```
Ethereum (ETH): 0x7d75f6a9c021fcc70691fec73368198823fb0f60
Bitcoin (BTC):  bc1q75w3cgutug8qdxw3jlmqnkjlv9alt3jr7ftha0
Binance (BNB):  0x7d75f6a9c021fcc70691fec73368198823fb0f60
```
