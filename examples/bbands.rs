//! Demonstrates how to initialize and use Bollinger Bands (BB).
use tatk::indicators::{BBands, EMA};
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Use EMA as the line instead of the default EMA.
    let ema = match EMA::new(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    let mut bbands = match BBands::with_line(ema, 2.0) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!(
        "\nBollinger Band (BBands): {}, lower: {}, upper: {}",
        bbands.value(),
        bbands.lower(),
        bbands.upper()
    );

    let next = bbands.next(last_data);
    println!(
        "Adding {}. New BBands: {}, lower: {}, upper: {}",
        last_data, next.1, next.0, next.2
    );
}
