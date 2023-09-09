//! Demonstrates how to initialize and use Bollinger Bands (BB).
use tatk::indicators::{BBands, EMA};
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 20;
    const DATA: &[f64] = TestData::talib();

    println!("Data: {:?}", DATA);
    println!("Period: {}", period);

    // Use EMA as the line instead of the default SMA.
    let ema = match EMA::new(period, DATA) {
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

    let next = bbands.next(107.0);
    println!(
        "Adding 107.00. New BBands: {}, lower: {}, upper: {}",
        next.1, next.0, next.2
    );
}
