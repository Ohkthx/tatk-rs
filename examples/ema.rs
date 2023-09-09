//! Demonstrates how to initialize and use a EMA.
use tatk::indicators::EMA;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    const DATA: &[f64] = TestData::talib();

    println!("Data: {:?}", DATA);
    println!("Period: {}", period);

    let mut ema = match EMA::new(period, DATA) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!("\nEMA: {}", ema.value());
    println!("Adding 107.00. New EMA: {}", ema.next(107.0));
}
