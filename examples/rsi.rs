//! Demonstrates how to initialize and use a RSI.
use tatk::indicators::RSI;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the RSI.
    let mut rsi = match RSI::new(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Update the thresholds for both oversold and overbought.
    rsi.set_oversold(30.0);
    rsi.set_overbought(70.0);

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nRSI: {}", rsi.value());
    println!(
        "Oversold?: {}, Overbought: {}",
        rsi.is_oversold(),
        rsi.is_overbought()
    );
    println!("Adding {}. New RSI: {}", last_data, rsi.next(last_data));
}
