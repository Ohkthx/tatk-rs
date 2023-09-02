//! Demonstrates how to initialize and use Bollinger Bands (BB).
use tatk::indicators::{BBands, EMA};
use tatk::test_data::TEST_DATA;
use tatk::traits::Line;

fn main() {
    let period: usize = 20;

    println!("Data: {:?}", TEST_DATA);
    println!("Period: {}", period);

    // Use EMA as the line instead of the default SMA.
    let ema = match EMA::new(period, TEST_DATA) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    let mut bbands = match BBands::with_line(ema, 2.0) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!(
        "\nBollinger Band (BBands): {}, lower: {}, upper: {}",
        bbands.value(),
        bbands.lower(),
        bbands.upper()
    );
    println!(
        "Adding 107.00. New BBands: {}, lower: {}, upper: {}",
        bbands.next(107.000000),
        bbands.lower(),
        bbands.upper()
    );
}
