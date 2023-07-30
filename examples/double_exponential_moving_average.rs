//! Demonstrates how to initialize and use a DEMA.
use sigta::indicators::DEMA;
use sigta::test_data::TEST_DATA;

fn main() {
    let period: usize = 10;

    println!("Data: {:?}", TEST_DATA);
    println!("Period: {}", period);

    let mut dema = match DEMA::new(period, TEST_DATA) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!("\nDEMA: {}", dema.value());
    println!("Adding 107.00. New DEMA: {}", dema.next(107.000000));
}
