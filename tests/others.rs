use tatk::indicators::{Variance, STDEV};
#[cfg(feature = "test-data")]
use tatk::test_data::TestData;
use tatk::traits::{Next, Value};

#[cfg(feature = "test-data")]
const DATA: &[f64] = TestData::talib_small();

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample variance using 20 data points with a period of 10.
fn create_variance_sample() {
    let variance = Variance::new(10, DATA, true).unwrap();
    assert_eq!(variance.value(), 12.190039166666669)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample variance using 20 data points with a period of 10, then adds an
/// additional data point.
fn next_variance_sample() {
    let mut variance = Variance::new(10, DATA, true).unwrap();
    assert_eq!(variance.next(107.0), 32.640476666666665)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population variance using 20 data points with a period of 10.
fn create_variance_population() {
    let variance = Variance::new(10, DATA, false).unwrap();
    assert_eq!(variance.value(), 10.971035250000002)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population variance using 20 data points with a period of 10, then adds an
/// additional data point.
fn next_variance_population() {
    let mut variance = Variance::new(10, DATA, false).unwrap();
    assert_eq!(variance.next(107.0), 29.376428999999995)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample standard deviation using 20 data points with a period of 10.
fn create_stdev_sample() {
    let stdev = STDEV::new(10, DATA, true).unwrap();
    assert_eq!(stdev.value(), 3.491423659005975)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the sample standard deviation using 20 data points with a period of 10, then adds an
/// additional data point.
fn next_stdev_sample() {
    let mut stdev = STDEV::new(10, DATA, true).unwrap();
    assert_eq!(stdev.next(107.0), 5.713184459359479)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population standard deviation using 20 data points with a period of 10.
fn create_stdev_population() {
    let stdev = STDEV::new(10, DATA, false).unwrap();
    assert_eq!(stdev.value(), 3.3122553117173807)
}

#[test]
#[cfg(feature = "test-data")]
/// Create and calculate the population standard deviation using 20 data points with a period of 10, then adds an
/// additional data point.
fn next_stdev_population() {
    let mut stdev = STDEV::new(10, DATA, false).unwrap();
    assert_eq!(stdev.next(107.0), 5.420002675276092)
}
