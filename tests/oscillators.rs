#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a Moving Average Convergence and Divergence using 19 data points with a short of 8, long of 10, and signal of 6.
fn create_macd() {
    use tatk::indicators::MovingAverageConvergenceDivergence;
    use tatk::test_data::TestData;
    const DATA: &[f64] = TestData::talib_small();

    let indicator =
        MovingAverageConvergenceDivergence::new(8, 10, 6, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), -0.3145389483187415)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a Moving Average Convergence and Divergence from 19 data points with short of 8, long of 10, and signal of 6, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_macd() {
    use tatk::indicators::MovingAverageConvergenceDivergence;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator =
        MovingAverageConvergenceDivergence::new(8, 10, 6, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]).0, -0.3300712744833305)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a Relative Strength Index using 19 data points with a period of 10.
fn create_rsi() {
    use tatk::indicators::RelativeStrengthIndex;
    use tatk::test_data::TestData;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = RelativeStrengthIndex::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 49.16871847490771)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a Relative Strength Index from 19 data points with period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_rsi() {
    use tatk::indicators::RelativeStrengthIndex;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = RelativeStrengthIndex::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 45.033256056615095)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate Bollinger Bands using 19 data points with a period of 20.
fn create_bbands() {
    use tatk::indicators::BollingerBands;
    use tatk::test_data::TestData;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = BollingerBands::new(10, &DATA[..DATA.len() - 1], 2.0).unwrap();
    assert_eq!(indicator.value(), 92.816)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates Bollinger Bands from 19 data points with period of 20, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_bbands() {
    use tatk::indicators::BollingerBands;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = BollingerBands::new(10, &DATA[..DATA.len() - 1], 2.0).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]).1, 92.5565)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate Average True Range using 364 data points with a period of 14.
fn create_atr() {
    use tatk::indicators::AverageTrueRange;
    use tatk::test_data::{Candle, TestData};
    let candles: Vec<Candle> = TestData::candles();

    let indicator = AverageTrueRange::new(10, &candles[..candles.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 839.944706407304)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates Average True Range from 364 data points with period of 14, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_atr() {
    use tatk::indicators::AverageTrueRange;
    use tatk::test_data::{Candle, TestData};
    use tatk::traits::Next;
    let candles: Vec<Candle> = TestData::candles();

    let mut indicator = AverageTrueRange::new(10, &candles[..candles.len() - 1]).unwrap();
    assert_eq!(
        indicator.next(candles[candles.len() - 1]),
        854.3072357665736
    )
}
