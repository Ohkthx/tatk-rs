//! Exponential Moving Average (EMA)
//!
//! # Formula
//!
//! EMA = [(x - y) * k] + y
//!
//! where:
//!
//! * `x` = current value (most recent)
//! * `y` = last EMA
//! * `k` = 2 * (n + 1)
//! * `n` = period

use crate::traits::{AsValue, InternalValue, Next, Period, Stats};
use crate::{Buffer, Num, TAError};
use tatk_derive::{InternalValue, Period};

/// Exponential Moving Average (EMA). More recent data is weighted heavier than older data.
///
/// # Formula
///
/// EMA = [(x - y) * k] + y
///
/// where:
/// * `x` = current value (most recent)
/// * `y` = last EMA
/// * `k` = 2 * (n + 1)
/// * `n` = period
#[derive(Debug, InternalValue, Period)]
pub struct ExponentialMovingAverage {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// Current value for the EMA.
    value: Num,
    /// Holds `period` amount of generated EMAs.
    buffer: Buffer,
    /// Smoothing value.
    k: Num,
}

impl ExponentialMovingAverage {
    /// Creates a new EMA with the supplied period and initial data.
    ///
    /// ### Requirements:
    ///
    /// * Period must be greater than 0.
    /// * Data must have at least `period` elements.
    ///
    /// ## Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the EMA from.
    pub fn new(period: usize, data: &[Num]) -> Result<Self, TAError> {
        // Check we can calculate EMA.
        if period < 1 {
            return Err(TAError::InvalidSize(String::from(
                "period cannot be less than 1 to calculate exponential moving average",
            )));
        } else if data.len() < period {
            // Make sure we have enough data.
            return Err(TAError::InvalidData(String::from(
                "not enough data for period provided",
            )));
        }

        // Temporary buffer to get seed SMA for EMA.
        let mut last_ema = match Buffer::from_array(period, &data[..period]) {
            Ok(v) => v.mean(),
            Err(error) => return Err(error),
        };

        // Buffer will hold last `period` EMAs.
        let mut buffer = match Buffer::from_array(period, &[last_ema]) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Smoothing factor.
        let k: Num = 2.0 / (period + 1) as Num;

        // Calculate the remainder of the datas EMA, using the prior EMA.
        for value in data[period..].iter() {
            last_ema = Self::calculate(&k, &last_ema, value);
            buffer.shift(last_ema);
        }

        Ok(Self {
            period,
            value: last_ema,
            buffer,
            k,
        })
    }

    /// Current and most recent value calculated.
    pub fn value(&self) -> Num {
        self.value
    }

    /// Smoothing factor.
    fn k(&self) -> &Num {
        &self.k
    }

    /// Calculates an EMA with newly provided data and the last EMA.
    ///
    /// # Arguments
    ///
    /// * `k` - Smoothing value for the EMA.
    /// * `last_ema` - Last EMA calculated.
    /// * `value` - Most recent value.
    fn calculate(k: &Num, last_ema: &Num, value: &Num) -> Num {
        (value - last_ema) * k + last_ema
    }
}

impl Next<Num> for ExponentialMovingAverage {
    /// Next value for the EMA.
    type Output = Num;

    /// Supply an additional value to recalculate a new EMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        // Get the next EMA value.
        self.value = Self::calculate(self.k(), &self.value(), &value);
        self.buffer.shift(self.value());
        self.value
    }
}

impl<T> Next<T> for ExponentialMovingAverage
where
    T: AsValue,
{
    /// Next value for the EMA.
    type Output = Num;

    /// Supply an additional value to recalculate a new EMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.next(value.as_value())
    }
}

impl Stats for ExponentialMovingAverage {
    /// Obtains the total sum of the buffer for EMA.
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
