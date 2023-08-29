#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample variance using 252 data points with a period of 12.
fn create_variance_sample() {
    use tatk::indicators::Variance;
    use tatk::test_data::TEST_DATA;

    let variance = Variance::new(12, TEST_DATA, true).unwrap();
    assert_eq!(variance.value(), 0.9084750000000004)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample variance using 252 data points with a period of 12, then adds an
/// additional data point.
fn next_variance_sample() {
    use tatk::indicators::Variance;
    use tatk::test_data::TEST_DATA;

    let mut variance = Variance::new(12, TEST_DATA, true).unwrap();
    assert_eq!(variance.next(107.000000), 0.9084750000000003)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population variance using 252 data points with a period of 12.
fn create_variance_population() {
    use tatk::indicators::Variance;
    use tatk::test_data::TEST_DATA;

    let variance = Variance::new(12, TEST_DATA, false).unwrap();
    assert_eq!(variance.value(), 0.8327687500000004)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population variance using 252 data points with a period of 12, then adds an
/// additional data point.
fn next_variance_population() {
    use tatk::indicators::Variance;
    use tatk::test_data::TEST_DATA;

    let mut variance = Variance::new(12, TEST_DATA, false).unwrap();
    assert_eq!(variance.next(107.000000), 0.8327687500000002)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample standard deviation using 252 data points with a period of 12.
fn create_stdev_sample() {
    use tatk::indicators::STDEV;
    use tatk::test_data::TEST_DATA;

    let stdev = STDEV::new(12, TEST_DATA, true).unwrap();
    assert_eq!(stdev.value(), 0.9531395490692852)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample standard deviation using 252 data points with a period of 12, then adds an
/// additional data point.
fn next_stdev_sample() {
    use tatk::indicators::STDEV;
    use tatk::test_data::TEST_DATA;

    let mut stdev = STDEV::new(12, TEST_DATA, true).unwrap();
    assert_eq!(stdev.next(107.000000), 0.9531395490692851)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population standard deviation using 252 data points with a period of 12.
fn create_stdev_population() {
    use tatk::indicators::STDEV;
    use tatk::test_data::TEST_DATA;

    let stdev = STDEV::new(12, TEST_DATA, false).unwrap();
    assert_eq!(stdev.value(), 0.9125616417535861)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population standard deviation using 252 data points with a period of 12, then adds an
/// additional data point.
fn next_stdev_population() {
    use tatk::indicators::STDEV;
    use tatk::test_data::TEST_DATA;

    let mut stdev = STDEV::new(12, TEST_DATA, false).unwrap();
    assert_eq!(stdev.next(107.000000), 0.912561641753586)
}
