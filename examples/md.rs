//! Demonstrates how to initialize and use a McGinley Dynamic Indicator.
use tatk::mdi;
use tatk::test_data::TestData;
use tatk::traits::Next;

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the McGinley Dynamic Indicator.
    let mut indicator = match mdi!(period, &data[..data.len() - 1], 0.6) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nMcGinley Dynamic Indicator: {}", indicator.value());
    println!(
        "Adding {}. New MDI: {}",
        last_data,
        indicator.next(last_data)
    );
}
