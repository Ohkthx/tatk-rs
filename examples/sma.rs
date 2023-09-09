//! Demonstrates how to initialize and use a SMA.
use tatk::indicators::SMA;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    const DATA: &[f64] = TestData::talib();

    println!("Data: {:?}", DATA);
    println!("Period: {}", period);

    let mut sma = match SMA::new(period, DATA) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!("\nSMA: {}", sma.value());
    println!("Adding 107.00. New SMA: {}", sma.next(107.0));
}
