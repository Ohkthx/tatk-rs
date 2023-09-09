//! Demonstrates how to initialize and use an OBV.
use tatk::indicators::OBV;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 14;
    let candles = TestData::candles();

    println!("Data (total): {:?}", candles.len());
    println!("Period: {}", period);

    // Create the OBV.
    let mut atr = match OBV::new(period, &candles[..candles.len() - 2]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last candle.
    let last_candle = candles[candles.len() - 1];

    println!("\nOBV: {}", atr.value());
    println!("Adding last candle. New OBV: {}", atr.next(last_candle));
}
