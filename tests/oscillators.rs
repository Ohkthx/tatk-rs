#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a MACD using 252 data points with a short of 12, long of 26, and signal of 9.
fn create_macd() {
    use tatk::indicators::MACD;
    use tatk::test_data::TEST_DATA;

    let macd = MACD::new(12, 26, 9, TEST_DATA).unwrap();
    assert_eq!(macd.value(), 0.9040092995013111);
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a MACD from 252 data points with short of 12, long of 26, and signal of 9, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_macd() {
    use tatk::indicators::MACD;
    use tatk::test_data::TEST_DATA;

    let mut macd = MACD::new(12, 26, 9, TEST_DATA).unwrap();
    assert_eq!(macd.next(107.000000), 0.6789823967962718);
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a RSI using 252 data points with a short of 12, long of 26, and signal of 9.
fn create_rsi() {
    use tatk::indicators::RSI;
    use tatk::test_data::TEST_DATA;

    let rsi = RSI::new(14, TEST_DATA).unwrap();
    assert_eq!(rsi.value(), 49.63210207086755);
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a RSI from 252 data points with short of 12, long of 26, and signal of 9, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_rsi() {
    use tatk::indicators::RSI;
    use tatk::test_data::TEST_DATA;

    let mut rsi = RSI::new(14, TEST_DATA).unwrap();
    assert_eq!(rsi.next(107.000000), 47.53209455563524);
}
