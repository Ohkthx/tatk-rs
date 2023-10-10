//! Demonstrates how to initialize and use a ROC.
use tatk::indicators::Roc;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let data = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the ROC.
    let mut roc = match Roc::new(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last candle.
    let last_candle = data[data.len() - 1];

    println!("\nROC: {}", roc.value());
    println!("Adding last candle. New ROC: {}", roc.next(last_candle));
}
