//! Demonstrates how to initialize and use a Exponential Moving Average.
use tatk::ema;
use tatk::test_data::TestData;
use tatk::traits::Next;

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the Exponential Moving Average.
    let mut indicator = match ema!(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nExponential Moving Average: {}", indicator.value());
    println!(
        "Adding {}. New EMA: {}",
        last_data,
        indicator.next(last_data)
    );
}
