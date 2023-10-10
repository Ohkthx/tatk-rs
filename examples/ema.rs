//! Demonstrates how to initialize and use a EMA.
use tatk::indicators::Ema;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the EMA.
    let mut ema = match Ema::new(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nEMA: {}", ema.value());
    println!("Adding {}. New EMA: {}", last_data, ema.next(last_data));
}
