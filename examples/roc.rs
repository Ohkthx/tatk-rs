//! Demonstrates how to initialize and use a Rate of Change.
use tatk::roc;
use tatk::test_data::TestData;
use tatk::traits::Next;

fn main() {
    let period: usize = 10;
    let data = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the Rate of Change.
    let mut indicator = match roc!(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last candle.
    let last_candle = data[data.len() - 1];

    println!("\nRate of Change: {}", indicator.value());
    println!(
        "Adding last candle. New ROC: {}",
        indicator.next(last_candle)
    );
}
