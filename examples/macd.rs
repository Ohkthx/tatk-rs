//! Demonstrates how to initialize and use a MACD.
use tatk::indicators::MACD;
use tatk::test_data::TEST_DATA;
use tatk::traits::Line;

// const TEST_DATA: &[f64] = &[
//     11.13, 11.3, 11.59, 11.71, 11.8, 11.8, 12.07, 12.14, 12.04, 12.02, 12.34, 12.61, 12.59, 12.66,
//     12.82, 12.93, 12.79, 12.21,
// ];

fn main() {
    let period: usize = 10;

    println!("Data: {:?}", TEST_DATA);
    println!("Period: {}", period);

    let mut macd = match MACD::new(12, 26, 9, TEST_DATA) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!("\nMACD: {}, signal: {}", macd.value(), macd.signal_value());
    println!("Adding 107.00. New MACD: {}", macd.next(107.000000));
}
