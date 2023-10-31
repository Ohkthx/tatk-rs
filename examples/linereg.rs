//! Demonstrates how to initialize and use a Linear Regression.
use tatk::lr;
use tatk::test_data::TestData;
use tatk::traits::Next;

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the Linear Regression indicator.
    let mut indicator = match lr!(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nbest fit: {}", indicator.value());
    println!(
        "Adding {}. New best fit: {}, r_squared: {:.4}",
        last_data,
        indicator.next(last_data),
        indicator.r_sq()
    );

    println!("Forecasted next: {}", indicator.forecast(1));
}
