//! Demonstrates how to initialize and use a RSI.
use tatk::indicators::RSI;
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

fn main() {
    let period: usize = 14;
    const DATA: &[f64] = TestData::talib();

    println!("Data: {:?}", DATA);
    println!("Period: {}", period);

    let mut rsi = match RSI::new(period, DATA) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Update the thresholds for both oversold and overbought.
    rsi.set_oversold(30.0);
    rsi.set_overbought(70.0);

    println!("\nRSI: {}", rsi.value());
    println!(
        "Oversold?: {}, Overbought: {}",
        rsi.is_oversold(),
        rsi.is_overbought()
    );
    println!("Adding 107.00. New RSI: {}", rsi.next(107.0));
}
