//! Demonstrates how to initialize and use a Moving Average Convergence and Divergence.
use tatk::macd;
use tatk::test_data::TestData;
use tatk::traits::Next;

fn main() {
    let short: usize = 8;
    let long: usize = 10;
    let signal: usize = 6;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Periods:");
    println!("short: {}, long: {}, signal: {}", short, long, signal);

    // Create the Moving Average Convergence and Divergence.
    let mut indicator = match macd!(short, long, signal, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!(
        "\nMoving Average Convergence and Divergence: {}, signal: {}",
        indicator.value(),
        indicator.signal_value()
    );
    println!(
        "Adding {}. New MACD: {}",
        last_data,
        indicator.next(last_data).0
    );
}
