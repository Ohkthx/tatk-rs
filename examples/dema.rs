//! Demonstrates how to initialize and use a DEMA.
use tatk::indicators::DEMA;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    const DATA: &[f64] = TestData::talib();

    println!("Data: {:?}", DATA);
    println!("Period: {}", period);

    let mut dema = match DEMA::new(period, DATA) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!("\nDEMA: {}", dema.value());
    println!("Adding 107.00. New DEMA: {}", dema.next(107.0));
}
