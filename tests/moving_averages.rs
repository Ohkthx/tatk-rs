#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a SMA using 252 data points with a period of 10.
fn create_sma() {
    use tatk::indicators::SMA;
    use tatk::test_data::TEST_DATA;

    let sma = SMA::new(10, TEST_DATA).unwrap();
    assert_eq!(sma.value(), 109.112);
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a SMA from 252 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_sma() {
    use tatk::indicators::SMA;
    use tatk::test_data::TEST_DATA;

    let mut sma = SMA::new(10, TEST_DATA).unwrap();
    assert_eq!(sma.next(107.0), 108.81199999999998);
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a EMA using 252 data points with a period of 10.
fn create_ema() {
    use tatk::indicators::EMA;
    use tatk::test_data::TEST_DATA;

    let ema = EMA::new(10, TEST_DATA).unwrap();
    assert_eq!(ema.value(), 108.97521174143839);
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a EMA from 252 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_ema() {
    use tatk::indicators::EMA;
    use tatk::test_data::TEST_DATA;

    let mut ema = EMA::new(10, TEST_DATA).unwrap();
    assert_eq!(ema.next(107.000000), 108.61608233390413);
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate a DEMA using 252 data points with a period of 10.
fn create_dema() {
    use tatk::indicators::DEMA;
    use tatk::test_data::TEST_DATA;

    let dema = DEMA::new(14, TEST_DATA).unwrap();
    assert_eq!(dema.value(), 109.46762588466589);
}

#[test]
#[cfg(feature = "test-data")]
/// Creates a DEMA from 252 data points and a period of 10, then adds an additional data point
/// to move the ensure the window of viewed is moving.
fn next_dema() {
    use tatk::indicators::DEMA;
    use tatk::test_data::TEST_DATA;

    let mut dema = DEMA::new(14, TEST_DATA).unwrap();
    assert_eq!(dema.next(107.000000), 108.91612556961066);
}

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
