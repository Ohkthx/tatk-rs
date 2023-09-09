use tatk::indicators::{DEMA, EMA, MD, OBV, SMA};
#[cfg(feature = "test-data")]
use tatk::test_data::{Candle, TestData};
use tatk::traits::{Next, Value};

#[cfg(feature = "test-data")]
const DATA: &[f64] = TestData::talib();

#[cfg(feature = "test-data")]
#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a SMA using 252 data points with a period of 10.
fn create_sma() {
    let sma = SMA::new(10, DATA).unwrap();
    assert_eq!(sma.value(), 109.112);
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a SMA from 252 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_sma() {
    let mut sma = SMA::new(10, DATA).unwrap();
    assert_eq!(sma.next(107.0), 108.81199999999998);
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a EMA using 252 data points with a period of 10.
fn create_ema() {
    let ema = EMA::new(10, DATA).unwrap();
    assert_eq!(ema.value(), 108.97521174143839)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a EMA from 252 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_ema() {
    let mut ema = EMA::new(10, DATA).unwrap();
    assert_eq!(ema.next(107.0), 108.61608233390413)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a DEMA using 252 data points with a period of 10.
fn create_dema() {
    let dema = DEMA::new(14, DATA).unwrap();
    assert_eq!(dema.value(), 109.46762588466589)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a DEMA from 252 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_dema() {
    let mut dema = DEMA::new(14, DATA).unwrap();
    assert_eq!(dema.next(107.0), 108.91612556961066)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a MD using 252 data points with a period of 10.
fn create_md() {
    let md = MD::new(10, DATA, 0.6).unwrap();
    assert_eq!(md.value(), 108.79792924004384)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a MD from 252 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_md() {
    let mut md = MD::new(10, DATA, 0.6).unwrap();
    assert_eq!(md.next(107.0), 108.47762052717786)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a OBV using 364 data points with a period of 14.
fn create_obv() {
    let candles: Vec<Candle> = TestData::candles();
    let obv = OBV::new(14, &candles[..candles.len() - 2]).unwrap();
    assert_eq!(obv.value(), 218972.70043628983)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a OBV from 364 data points and a period of 14, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_obv() {
    let candles: Vec<Candle> = TestData::candles();
    let mut obv = OBV::new(14, &candles[..candles.len() - 2]).unwrap();
    assert_eq!(obv.next(candles[candles.len() - 1]), 227755.31966018982)
}
