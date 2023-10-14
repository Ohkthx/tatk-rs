#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample variance using 20 data points with a period of 10.
fn create_variance_sample() {
    use tatk::indicators::Variance;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = Variance::new(10, &DATA[..DATA.len() - 1], true).unwrap();
    assert_eq!(indicator.value(), 11.317109999999998)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample variance using 20 data points with a period of 10, then adds an
/// additional data point.
fn next_variance_sample() {
    use tatk::indicators::Variance;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = Variance::new(10, &DATA[..DATA.len() - 1], true).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 12.190039166666669)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population variance using 20 data points with a period of 10.
fn create_variance_population() {
    use tatk::indicators::Variance;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = Variance::new(10, &DATA[..DATA.len() - 1], false).unwrap();
    assert_eq!(indicator.value(), 10.185398999999999)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population variance using 20 data points with a period of 10, then adds an
/// additional data point.
fn next_variance_population() {
    use tatk::indicators::Variance;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = Variance::new(10, &DATA[..DATA.len() - 1], false).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 10.971035250000002)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample standard deviation using 20 data points with a period of 10.
fn create_stdev_sample() {
    use tatk::indicators::StandardDeviation;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = StandardDeviation::new(10, &DATA[..DATA.len() - 1], true).unwrap();
    assert_eq!(indicator.value(), 3.3640912591664334)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample standard deviation using 20 data points with a period of 10, then adds an
/// additional data point.
fn next_stdev_sample() {
    use tatk::indicators::StandardDeviation;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = StandardDeviation::new(10, &DATA[..DATA.len() - 1], true).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 3.491423659005975)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population standard deviation using 20 data points with a period of 10.
fn create_stdev_population() {
    use tatk::indicators::StandardDeviation;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = StandardDeviation::new(10, &DATA[..DATA.len() - 1], false).unwrap();
    assert_eq!(indicator.value(), 3.1914571906889178)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population standard deviation using 20 data points with a period of 10, then adds an
/// additional data point.
fn next_stdev_population() {
    use tatk::indicators::StandardDeviation;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = StandardDeviation::new(10, &DATA[..DATA.len() - 1], false).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 3.3122553117173807)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the best fit line using 20 data points with a period of 10.
fn create_linereg() {
    use tatk::indicators::LinearRegression;
    use tatk::test_data::TestData;
    use tatk::traits::Value;
    const DATA: &[f64] = TestData::talib_small();

    let indicator = LinearRegression::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.value(), 89.77590909090917)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the best fit line using 20 data points with a period of 10, then adds an
/// additional data point.
fn next_linereg() {
    use tatk::indicators::LinearRegression;
    use tatk::test_data::TestData;
    use tatk::traits::Next;
    const DATA: &[f64] = TestData::talib_small();

    let mut indicator = LinearRegression::new(10, &DATA[..DATA.len() - 1]).unwrap();
    assert_eq!(indicator.next(DATA[DATA.len() - 1]), 88.69072727272732)
}
