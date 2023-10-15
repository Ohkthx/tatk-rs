//! Double Exponential Moving Average (DEMA)
//!
//! # Formula
//!
//! DEMA = (2 * EMA(n)) - EMA(EMA(n))
//!
//! DEMA = (2 * x) - y
//!
//! where:
//!
//! * `x` = \[EMA(n)\] Current EMA of period `n`
//! * `y` = \[EMA(EMA(n))\] EMA of EMA(n)
//! * `n` = period

use super::ExponentialMovingAverage;
use crate::traits::{AsValue, InternalValue, Next, Period, Stats};
use crate::{Buffer, Num, TAError};
use tatk_derive::{InternalValue, Period};

/// Double Exponential Moving Average (DEMA)
///
/// # Formula
///
/// DEMA = (2 * EMA(n)) - EMA(EMA(n))
///
/// DEMA = (2 * x) - y
///
/// where:
///
/// * `x` = \[EMA(n)\] Current EMA of period `n`
/// * `y` = \[EMA(EMA(n))\] EMA of EMA(n)
/// * `n` = period
#[derive(Debug, InternalValue, Period)]
pub struct DoubleExponentialMovingAverage {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// DEMA's current value.
    value: Num,
    /// EMA(n), EMA of values / samples provided.
    ema_n: ExponentialMovingAverage,
    /// EMA(EMA(n)), EMA of EMA(n).
    ema_ema_n: ExponentialMovingAverage,
    /// Holds `period` amount of generated DEMAs.
    buffer: Buffer,
}

impl DoubleExponentialMovingAverage {
    /// Creates a new Double Exponential Moving Average with the supplied period and initial data.
    ///
    /// ### Requirements:
    ///
    /// * Period must be greater than 0.
    /// * Data must have at least `(period * 2) - 1` elements.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the DEMA from.
    pub fn new(period: usize, data: &[Num]) -> Result<Self, TAError> {
        // Check we can calculate Double Exponential Moving Average.
        if period < 1 {
            return Err(TAError::InvalidSize(String::from(
                "period cannot be less than 1 to calculate double exponential moving average",
            )));
        } else if data.len() < (period * 2) - 1 {
            // Make sure we have enough data.
            return Err(TAError::InvalidData(String::from(
                "not enough data for period provided",
            )));
        }

        // Build EMA(n) from first 'n' samples (period amount).
        let mut ema_n = match ExponentialMovingAverage::new(period, &data[..period]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // n EMA(n), build it manually because we need to catch the output.
        let mut n_ema_n: Vec<Num> = vec![ema_n.value()];
        for v in data[period..((period * 2) - 1)].iter() {
            n_ema_n.push(ema_n.next(*v));
        }

        // EMA of EMA(n)
        let mut ema_ema_n = match ExponentialMovingAverage::new(period, &n_ema_n) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // Buffer will old processed DEMAs
        let mut value = (2.0 * ema_n.value()) - ema_ema_n.value();
        let mut buffer = match Buffer::from_array(period, &[value]) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Calculate the remainder data points.
        for v in data[((period * 2) - 1)..].iter() {
            let eman: Num = ema_n.next(*v);

            // Calculate the new DEMA.
            value = (2.0 * eman) - ema_ema_n.next(eman);
            buffer.shift(value);
        }

        Ok(Self {
            period,
            value,
            ema_n,
            ema_ema_n,
            buffer,
        })
    }

    /// Current and most recent value calculated.
    pub fn value(&self) -> Num {
        self.value
    }
}

impl Next<Num> for DoubleExponentialMovingAverage {
    /// Next value for the DEMA.
    type Output = Num;

    /// Supply an additional value to recalculate a new DEMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        let ema: Num = self.ema_n.next(value);

        // Calculate the new DEMA.
        self.value = (2.0 * ema) - self.ema_ema_n.next(ema);
        self.buffer.shift(self.value());
        self.value
    }
}

impl<T> Next<T> for DoubleExponentialMovingAverage
where
    T: AsValue,
{
    /// Next value for the DEMA.
    type Output = Num;

    /// Supply an additional value to recalculate a new DEMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.next(value.as_value())
    }
}

impl Stats for DoubleExponentialMovingAverage {
    /// Obtains the total sum of the buffer for DEMA.
    fn sum(&self) -> Num {
        self.buffer.sum()
    }

    /// Mean for the period of the EMA.
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
