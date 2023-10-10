//! Demonstrates how to initialize and use a MD.
use tatk::indicators::Mdi;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the MD.
    let mut md = match Mdi::new(period, &data[..data.len() - 1], 0.6) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nMD: {}", md.value());
    println!("Adding {}. New MD: {}", last_data, md.next(last_data));
}
