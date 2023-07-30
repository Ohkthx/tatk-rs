//! Demonstrates how to initialize and use a SMA.
use sigta::indicators::SMA;
use sigta::test_data::TEST_DATA;

fn main() {
    let period: usize = 10;

    println!("Data: {:?}", TEST_DATA);
    println!("Period: {}", period);

    let mut sma = match SMA::new(period, TEST_DATA) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!("\nSMA: {}", sma.value());
    println!("Adding 107.00. New SMA: {}", sma.next(107.000000));
}
