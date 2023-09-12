use tatk::indicators::{DEMA, EMA, MD, OBV, ROC, SMA};
#[cfg(feature = "test-data")]
use tatk::test_data::{Candle, TestData};
use tatk::traits::{Next, Value};

#[cfg(feature = "test-data")]
const DATA: &[f64] = TestData::talib_small();

#[cfg(feature = "test-data")]
#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a SMA using 19 data points with a period of 10.
fn create_sma() {
    let indicator = SMA::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 92.816)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a SMA from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_sma() {
    let mut indicator = SMA::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 92.5565)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a EMA using 19 data points with a period of 10.
fn create_ema() {
    let indicator = EMA::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 91.98938928832645)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates an EMA from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_ema() {
    let mut indicator = EMA::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 91.6049548722671)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a DEMA using 19 data points with a period of 10.
fn create_dema() {
    let indicator = DEMA::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 90.5309787563998)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a DEMA from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_dema() {
    let mut indicator = DEMA::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 90.09717264209674)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a MD using 19 data points with a period of 10.
fn create_md() {
    let indicator = MD::new(10, &DATA[..DATA.len() - 1], 0.6).unwrap();
    assert_eq!(indicator.value(), 91.6428518997655)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a MD from 19 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_md() {
    let mut indicator = MD::new(10, &DATA[..DATA.len() - 1], 0.6).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 91.32433432593635)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a OBV using 364 data points with a period of 10.
fn create_obv() {
    let candles: Vec<Candle> = TestData::candles();
    let obv = OBV::new(10, &candles[..candles.len() - 1]).unwrap();
    assert_eq!(obv.value(), 201742.77812596984)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a OBV from 364 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_obv() {
    let candles: Vec<Candle> = TestData::candles();
    let mut obv = OBV::new(10, &candles[..candles.len() - 1]).unwrap();
    assert_eq!(obv.next(candles[candles.len() - 1]), 210525.39734986983)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a ROC using 251 data points with a period of 10.
fn create_roc() {
    let roc = ROC::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(roc.value(), 1.4504788794773873)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a ROC from 251 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_roc() {
    let mut roc = ROC::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(roc.next(DATA[DATA.len() - 1]), -2.806315561803827)
}
