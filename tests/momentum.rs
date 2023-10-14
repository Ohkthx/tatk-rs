#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a Simple Moving Average using 19 data points with a period of 10.
fn create_sma() {
    use tatk::indicators::SimpleMovingAverage;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = SimpleMovingAverage::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 92.816)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a Simple Moving Average from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_sma() {
    use tatk::indicators::SimpleMovingAverage;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = SimpleMovingAverage::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 92.5565)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate an Exponential Moving Average using 19 data points with a period of 10.
fn create_ema() {
    use tatk::indicators::ExponentialMovingAverage;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = ExponentialMovingAverage::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 91.98938928832645)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates an Exponential Moving Average from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_ema() {
    use tatk::indicators::ExponentialMovingAverage;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = ExponentialMovingAverage::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 91.6049548722671)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a Double Exponential Moving Average using 19 data points with a period of 10.
fn create_dema() {
    use tatk::indicators::DoubleExponentialMovingAverage;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = DoubleExponentialMovingAverage::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 90.5309787563998)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a Double Exponential Moving Average from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_dema() {
    use tatk::indicators::DoubleExponentialMovingAverage;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = DoubleExponentialMovingAverage::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 90.09717264209674)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a McGinley Dynamic Indicator using 19 data points with a period of 10.
fn create_md() {
    use tatk::indicators::McGinleyDynamic;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = McGinleyDynamic::new(10, &DATA[..DATA.len() - 1], 0.6).unwrap();
    assert_eq!(indicator.value(), 91.6428518997655)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a McGinley Dynamic Indicator from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_md() {
    use tatk::indicators::McGinleyDynamic;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = McGinleyDynamic::new(10, &DATA[..DATA.len() - 1], 0.6).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 91.32433432593635)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate an On-Balance Volume using 364 data points with a period of 10.
fn create_obv() {
    use tatk::indicators::OnBalanceVolume;
    use tatk::test_data::{Candle, TestData};
    use tatk::traits::Value;

    let candles: Vec<Candle> = TestData::candles();
    let indicator = OnBalanceVolume::new(10, &candles[..candles.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 201742.77812596984)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates an On-Balance Volume from 364 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_obv() {
    use tatk::indicators::OnBalanceVolume;
    use tatk::test_data::{Candle, TestData};
    use tatk::traits::Next;

    let candles: Vec<Candle> = TestData::candles();
    let mut indicator = OnBalanceVolume::new(10, &candles[..candles.len() - 1]).unwrap();
    assert_eq!(
        indicator.next(candles[candles.len() - 1]),
        210525.39734986983
    )
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a Rate of Change using 251 data points with a period of 10.
fn create_roc() {
    use tatk::indicators::RateOfChange;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = RateOfChange::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 1.4504788794773873)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a Rate of Change from 251 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_roc() {
    use tatk::indicators::RateOfChange;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = RateOfChange::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), -2.806315561803827)
}
