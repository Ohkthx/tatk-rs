//! Demonstrates how to initialize and use a EMA.
use sigta::indicators::EMA;
use sigta::test_data::TEST_DATA;

fn main() {
    let period: usize = 10;

    println!("Data: {:?}", TEST_DATA);
    println!("Period: {}", period);

    let mut ema = match EMA::new(period, TEST_DATA) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!("\nEMA: {}", ema.value());
    println!("Adding 107.00. New EMA: {}", ema.next(107.000000));
}
