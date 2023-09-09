use tatk::indicators::{BBands, ATR, MACD, RSI};
#[cfg(feature = "test-data")]
use tatk::test_data::{Candle, TestData};
use tatk::traits::{Next, Value};

#[cfg(feature = "test-data")]
const DATA: &[f64] = TestData::talib();

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a MACD using 252 data points with a short of 12, long of 26, and signal of 9.
fn create_macd() {
    let macd = MACD::new(12, 26, 9, DATA).unwrap();
    assert_eq!(macd.value(), 0.9040092995013111)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a MACD from 252 data points with short of 12, long of 26, and signal of 9, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_macd() {
    let mut macd = MACD::new(12, 26, 9, DATA).unwrap();
    assert_eq!(macd.next(107.0).0, 0.6789823967962718)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a RSI using 252 data points with a short of 12, long of 26, and signal of 9.
fn create_rsi() {
    let rsi = RSI::new(14, DATA).unwrap();
    assert_eq!(rsi.value(), 49.63210207086755)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a RSI from 252 data points with short of 12, long of 26, and signal of 9, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_rsi() {
    let mut rsi = RSI::new(14, DATA).unwrap();
    assert_eq!(rsi.next(107.0), 47.53209455563524)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate BBands using 252 data points with a period of 20.
fn create_bbands() {
    let bbands = BBands::new(20, DATA, 2.0).unwrap();
    assert_eq!(bbands.lower(), 104.38335904421554)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates BBands from 252 data points with period of 20, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_bbands() {
    let mut bbands = BBands::new(20, DATA, 2.0).unwrap();
    bbands.next(107.0);
    assert_eq!(bbands.upper(), 116.6788398921392)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate ATR using 364 data points with a period of 14.
fn create_atr() {
    let candles: Vec<Candle> = TestData::candles();
    let atr = ATR::new(14, &candles[..candles.len() - 2]).unwrap();
    assert_eq!(atr.value(), 754.436206059607)
}

#[test]
#[cfg(feature = "test-data")]
/// Creates ATR from 364 data points with period of 14, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_atr() {
    let candles: Vec<Candle> = TestData::candles();
    let mut atr = ATR::new(14, &candles[..candles.len() - 2]).unwrap();
    assert_eq!(atr.next(candles[candles.len() - 1]), 750.075762769635)
}
