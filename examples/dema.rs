//! Demonstrates how to initialize and use a DEMA.
use tatk::indicators::Dema;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the DEMA.
    let mut dema = match Dema::new(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nDEMA: {}", dema.value());
    println!("Adding {}. New DEMA: {}", last_data, dema.next(last_data));
}
