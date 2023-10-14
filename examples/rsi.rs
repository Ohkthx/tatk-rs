//! Demonstrates how to initialize and use a Relative Strength Index.
use tatk::rsi;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the Relative Strength Index.
    let mut indicator = match rsi!(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Update the thresholds for both oversold and overbought.
    indicator.set_oversold(30.0);
    indicator.set_overbought(70.0);

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nRelative Strength Index: {}", indicator.value());
    println!(
        "Oversold?: {}, Overbought: {}",
        indicator.is_oversold(),
        indicator.is_overbought()
    );
    println!(
        "Adding {}. New RSI: {}",
        last_data,
        indicator.next(last_data)
    );
}
