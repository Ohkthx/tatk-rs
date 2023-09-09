//! Demonstrates how to initialize and use a MACD.
use tatk::indicators::MACD;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    const DATA: &[f64] = TestData::talib();

    println!("Data: {:?}", DATA);
    println!("Period: {}", period);

    let mut macd = match MACD::new(12, 26, 9, DATA) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!("\nMACD: {}, signal: {}", macd.value(), macd.signal_value());
    println!("Adding 107.00. New MACD: {}", macd.next(107.0).1);
}
