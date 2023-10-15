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

use crate::traits::{Close, InternalValue, Next, Period, Stats, Volume};
use crate::{Buffer, Num, TAError};
use tatk_derive::{InternalValue, Period};

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
#[derive(Debug, InternalValue, Period)]
pub struct OnBalanceVolume {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// OBV's current value.
    value: Num,
    /// Lost close.
    last_close: Num,
    /// Holds all of the current period's values.
    buffer: Buffer,
}

impl OnBalanceVolume {
    /// Creates a new On-Balance Volume with the supplied period and initial data.
    ///
    /// ### Requirements:
    ///
    /// * Period must be greater than 1.
    /// * Data must have at least `period` elements.
    ///
    /// ## Arguments
    ///
    /// * `period` - History of values to keep.
    /// * `data` - Array of values to create the OBV from.
    pub fn new<T>(period: usize, data: &[T]) -> Result<Self, TAError>
    where
        T: Close + Volume,
    {
        // Check we can calculate On-Balance Volume.
        if period < 2 {
            return Err(TAError::InvalidSize(String::from(
                "period cannot be less than 2 to calculate on-balance volume",
            )));
        } else if data.len() < period {
            // Make sure we have enough data.
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

    /// Current and most recent value calculated.
    pub fn value(&self) -> Num {
        self.value
    }

    /// Calculates the On-Balance Value.
    ///
    /// # Arguments
    ///
    /// * `last_obv` - Last calculate on-balance volume.
    /// * `close` - Current close value.
    /// * `close_prev` - Previous close value.
    /// * `volume` - Current volume.
    fn calculate<T>(last_obv: Num, value: &T, close_prev: Num) -> Num
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

impl<T> Next<T> for OnBalanceVolume
where
    T: Close + Volume,
{
    /// Next value for the OBV.
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

impl Next<(Num, Num)> for OnBalanceVolume {
    /// Next value for the OBV.
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

impl Stats for OnBalanceVolume {
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
