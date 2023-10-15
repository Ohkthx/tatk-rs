//! Demonstrates how to initialize and use a Simple Moving Average.
use tatk::sma;
use tatk::test_data::TestData;
use tatk::traits::Next;

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the Simple Moving Average.
    let mut indicator = match sma!(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nSimple Moving Average: {}", indicator.value());
    println!(
        "Adding {}. New SMA: {}",
        last_data,
        indicator.next(last_data)
    );
}
