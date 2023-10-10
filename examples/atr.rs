//! Demonstrates how to initialize and use an ATR.
use tatk::indicators::Atr;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let candles = TestData::candles();

    println!("Data (total): {:?}", candles.len());
    println!("Period: {}", period);

    // Create the ATR.
    let mut atr = match Atr::new(period, &candles[..candles.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last candle.
    let last_candle = candles[candles.len() - 1];

    println!("\nATR: {}", atr.value());
    println!("Adding last candle. New ATR: {}", atr.next(last_candle));
}
