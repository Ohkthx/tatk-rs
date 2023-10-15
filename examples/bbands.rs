//! Demonstrates how to initialize and use Bollinger Bands (BB).
use tatk::ema;
use tatk::indicators::BollingerBands;
use tatk::test_data::TestData;
use tatk::traits::Next;

fn main() {
    let period: usize = 10;
    let data: &[f64] = TestData::talib_small();

    println!("Data (total): {:?}", data.len());
    println!("Period: {}", period);

    // Use EMA as the line instead of the default SMA.
    let ema_indicator = match ema!(period, &data[..data.len() - 1]) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    let mut indicator = match BollingerBands::with_line(ema_indicator, 2.0) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    // Extract last data point.
    let last_data = data[data.len() - 1];

    println!(
        "\nBollinger Band (BB): {}, lower: {}, upper: {}",
        indicator.value(),
        indicator.lower(),
        indicator.upper()
    );

    let next = indicator.next(last_data);
    println!(
        "Adding {}. New BBands value: {}, lower: {}, upper: {}",
        last_data, next.1, next.0, next.2
    );
}
