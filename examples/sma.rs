//! Demonstrates how to initialize and use a SMA.
use tatk::indicators::Sma;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the SMA.
    let mut sma = match Sma::new(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nSMA: {}", sma.value());
    println!("Adding {}. New SMA: {}", last_data, sma.next(last_data));
}
