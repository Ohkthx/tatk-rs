//! Demonstrates how to initialize and use an On-Balance Volume.
use tatk::obv;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let candles = TestData::candles();

    println!("Data (total): {:?}", candles.len());
    println!("Period: {}", period);

    // Create the On-Balance Volume.
    let mut indicator = match obv!(period, &candles[..candles.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last candle.
    let last_candle = candles[candles.len() - 1];

    println!("\nOn-Balance Volume: {}", indicator.value());
    println!(
        "Adding last candle. New OBV: {}",
        indicator.next(last_candle)
    );
}
