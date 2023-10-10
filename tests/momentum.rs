#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a SMA using 19 data points with a period of 10.
fn create_sma() {
    use tatk::indicators::Sma;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = Sma::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 92.816)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a SMA from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_sma() {
    use tatk::indicators::Sma;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = Sma::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 92.5565)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a EMA using 19 data points with a period of 10.
fn create_ema() {
    use tatk::indicators::Ema;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = Ema::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 91.98938928832645)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates an EMA from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_ema() {
    use tatk::indicators::Ema;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = Ema::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 91.6049548722671)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a DEMA using 19 data points with a period of 10.
fn create_dema() {
    use tatk::indicators::Dema;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = Dema::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 90.5309787563998)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a DEMA from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_dema() {
    use tatk::indicators::Dema;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = Dema::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 90.09717264209674)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a MD using 19 data points with a period of 10.
fn create_md() {
    use tatk::indicators::Mdi;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = Mdi::new(10, &DATA[..DATA.len() - 1], 0.6).unwrap();
    assert_eq!(indicator.value(), 91.6428518997655)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a MD from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_md() {
    use tatk::indicators::Mdi;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = Mdi::new(10, &DATA[..DATA.len() - 1], 0.6).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 91.32433432593635)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a OBV using 364 data points with a period of 10.
fn create_obv() {
    use tatk::indicators::Obv;
    use tatk::test_data::{Candle, TestData};
    use tatk::traits::Value;

    let candles: Vec<Candle> = TestData::candles();
    let indicator = Obv::new(10, &candles[..candles.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 201742.77812596984)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a OBV from 364 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_obv() {
    use tatk::indicators::Obv;
    use tatk::test_data::{Candle, TestData};
    use tatk::traits::Next;

    let candles: Vec<Candle> = TestData::candles();
    let mut indicator = Obv::new(10, &candles[..candles.len() - 1]).unwrap();
    assert_eq!(
        indicator.next(candles[candles.len() - 1]),
        210525.39734986983
    )
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a ROC using 251 data points with a period of 10.
fn create_roc() {
    use tatk::indicators::Roc;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = Roc::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 1.4504788794773873)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a ROC from 251 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_roc() {
    use tatk::indicators::Roc;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = Roc::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), -2.806315561803827)
}
