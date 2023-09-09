//! Demonstrates how to initialize and use a MD.
use tatk::indicators::MD;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let k: f64 = 0.6;
    const DATA: &[f64] = TestData::talib();

    println!("Data: {:?}", DATA);
    println!("Period: {}", period);

    let mut md = match MD::new(period, DATA, k) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!("\nMD: {}", md.value());
    println!("Adding 107.00. New MD: {}", md.next(107.0));
}
