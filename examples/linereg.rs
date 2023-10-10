//! Demonstrates how to initialize and use a LineReg.
use tatk::indicators::Linereg;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Create the LineReg.
    let mut linereg = match Linereg::new(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!("\nbest fit: {}", linereg.value());
    println!(
        "Adding {}. New best fit: {}, r_squared: {:.4}",
        last_data,
        linereg.next(last_data),
        linereg.r_sq()
    );

    println!("Forecasted next: {}", linereg.forecast(1));
}
