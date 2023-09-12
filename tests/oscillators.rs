use tatk::indicators::{BBands, ATR, MACD, RSI};
#[cfg(feature = "test-data")]
use tatk::test_data::{Candle, TestData};
use tatk::traits::{Next, Value};

#[cfg(feature = "test-data")]
const DATA: &[f64] = TestData::talib_small();

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a MACD using 19 data points with a short of 8, long of 10, and signal of 6.
fn create_macd() {
    let indicator = MACD::new(8, 10, 6, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), -0.3145389483187415)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a MACD from 19 data points with short of 8, long of 10, and signal of 6, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_macd() {
    let mut indicator = MACD::new(8, 10, 6, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]).0, -0.3300712744833305)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a RSI using 19 data points with a period of 10.
fn create_rsi() {
    let indicator = RSI::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 49.16871847490771)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a RSI from 19 data points with period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_rsi() {
    let mut indicator = RSI::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 45.033256056615095)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate BBands using 19 data points with a period of 20.
fn create_bbands() {
    let indicator = BBands::new(10, &DATA[..DATA.len() - 1], 2.0).unwrap();
    assert_eq!(indicator.value(), 92.816)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates BBands from 19 data points with period of 20, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_bbands() {
    let mut indicator = BBands::new(10, &DATA[..DATA.len() - 1], 2.0).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]).1, 92.5565)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate ATR using 364 data points with a period of 14.
fn create_atr() {
    let candles: Vec<Candle> = TestData::candles();
    let atr = ATR::new(10, &candles[..candles.len() - 1]).unwrap();
    assert_eq!(atr.value(), 839.944706407304)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates ATR from 364 data points with period of 14, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_atr() {
    let candles: Vec<Candle> = TestData::candles();
    let mut atr = ATR::new(10, &candles[..candles.len() - 1]).unwrap();
    assert_eq!(atr.next(candles[candles.len() - 1]), 854.3072357665736)
}
