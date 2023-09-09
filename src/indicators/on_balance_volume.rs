//! On-Balance Volume (OBV), volume flow to predict price changes.
//!
//! # Formula
//!
//! vol = x == y ? 0.0 : x > y ? z : -z
//!
//! OBV = OBV_prev + vol
//!
//! where:
//!
//! * `x` = current close (most recent)
//! * `y` = last close
//! * `z` = current volume
use crate::traits::{Close, Next, Period, Stats, Value, Volume};
use crate::{Buffer, Num, TAError};

/// Used for conversions. Holds Close (0), and Volume (1) values.
#[derive(Copy, Clone)]
struct Data(Num, Num);

// Closing value.
impl Close for Data {
    fn close(&self) -> Num {
        self.0
    }
}

// Volume value.
impl Volume for Data {
    fn volume(&self) -> Num {
        self.1
    }
}

/// On-Balance Volume (OBV), volume flow to predict price changes.
///
/// # Formula
///
/// vol = x == y ? 0.0 : x > y ? z : -z
///
/// OBV = OBV_prev + vol
///
/// where:
///
/// * `x` = current close (most recent)
/// * `y` = last close
/// * `z` = current volume
#[derive(Debug)]
pub struct OBV {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// OBV's current value.
    value: Num,
    /// Lost close.
    last_close: Num,
    /// Holds all of the current period's values.
    buffer: Buffer,
}

impl OBV {
    /// Creates a new OBV with the supplied period and initial data.
    ///
    /// Required: The initial data must be at least of equal size/length or greater than the period.
    ///
    /// # Arguments
    ///
    /// * `period` - History of values to keep.
    /// * `data` - Array of values to create the OBV from.
    pub fn new<T>(period: usize, data: &[T]) -> Result<Self, TAError>
    where
        T: Close + Volume,
    {
        // Make sure we have enough data.
        if data.len() < period {
            return Err(TAError::InvalidData(String::from(
                "not enough data for history period provided",
            )));
        }

        let mut last_close = data[0].close();
        let mut last_obv = 0.0;

        // Build the buffer from the data provided.
        let mut buffer: Buffer = match Buffer::from_array(period, &[last_obv]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // Calculate the remaining values.
        for v in data[1..].iter() {
            last_obv = Self::calculate(last_obv, v, last_close);
            last_close = v.close();
            buffer.shift(last_obv);
        }

        Ok(Self {
            period,
            last_close,
            value: last_obv,
            buffer,
        })
    }

    /// Calculates the On-Balance Value.
    ///
    /// # Arguments
    ///
    /// * `last_obv` - Last calculate on-balance volume.
    /// * `close` - Current close value.
    /// * `close_prev` - Previous close value.
    /// * `volume` - Current volume.
    pub fn calculate<T>(last_obv: Num, value: &T, close_prev: Num) -> Num
    where
        T: Close + Volume,
    {
        let vol = if value.close() > close_prev {
            value.volume()
        } else if value.close() < close_prev {
            -value.volume()
        } else {
            0.0
        };

        last_obv + vol
    }
}

impl Period for OBV {
    /// Period (window) for the history.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for OBV {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl<T> Next<T> for OBV
where
    T: Close + Volume,
{
    /// Next Value for the OBV.
    type Output = Num;

    /// Supply an additional value to recalculate a new OBV.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.value = Self::calculate(self.value(), &value, self.last_close);
        self.last_close = value.close();

        // Rotate the buffer.
        self.buffer.shift(self.value());
        self.value
    }
}

impl Next<(Num, Num)> for OBV {
    /// Next Value for the OBV.
    type Output = Num;

    /// Supply an additional value to recalculate a new OBV.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to calculate.
    ///     * 0 = Close
    ///     * 1 = Volume
    fn next(&mut self, value: (Num, Num)) -> Self::Output {
        let v = Data {
            0: value.0,
            1: value.1,
        };

        self.next(v)
    }
}

impl Stats for OBV {
    /// Obtains the total sum of the buffer for OBV.
    fn sum(&self) -> Num {
        self.buffer.sum()
    }

    /// Mean for the period of the OBV.
    fn mean(&self) -> Num {
        self.buffer.mean()
    }

    /// Current variance for the period.
    ///
    /// # Arguments
    ///
    /// * `is_sample` - If the data is a Sample or Population, default should be True.
    fn variance(&self, is_sample: bool) -> Num {
        self.buffer.variance(is_sample)
    }

    /// Current standard deviation for the period.
    ///
    /// # Arguments
    ///
    /// * `is_sample` - If the data is a Sample or Population, default should be True.
    fn stdev(&self, is_sample: bool) -> Num {
        self.buffer.stdev(is_sample)
    }
}
