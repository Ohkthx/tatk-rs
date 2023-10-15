//! Demonstrates how to initialize and use an Average True Range.
use tatk::atr;
use tatk::test_data::TestData;
use tatk::traits::Next;

fn main() {
    let period: usize = 10;
    let candles = TestData::candles();

    println!("Data (total): {:?}", candles.len());
    println!("Period: {}", period);

    // Create the Average True Range.
    let mut indicator = match atr!(period, &candles[..candles.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last candle.
    let last_candle = candles[candles.len() - 1];

    println!("\nAverage True Range: {}", indicator.value());
    println!(
        "Adding last candle. New ATR: {}",
        indicator.next(last_candle)
    );
}
