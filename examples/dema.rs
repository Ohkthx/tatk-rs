//! Demonstrates how to initialize and use a Double Exponential Moving Average.
use tatk::dema;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the Double Exponential Moving Average.
    let mut indicator = match dema!(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nDouble Exponential Moving Average: {}", indicator.value());
    println!(
        "Adding {}. New DEMA: {}",
        last_data,
        indicator.next(last_data)
    );
}
