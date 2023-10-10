//! Demonstrates how to initialize and use a MACD.
use tatk::indicators::Macd;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let short: usize = 8;
    let long: usize = 10;
    let signal: usize = 6;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Periods:");
    println!("short: {}, long: {}, signal: {}", short, long, signal);

    // Create the MACD.
    let mut macd = match Macd::new(short, long, signal, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nMACD: {}, signal: {}", macd.value(), macd.signal_value());
    println!("Adding {}. New MACD: {}", last_data, macd.next(last_data).0);
}
