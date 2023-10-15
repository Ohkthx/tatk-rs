//! Standard Deviation (SD/STDEV)
//!
//! Measure of the amount of variation or dispersion between values. The lower the standard
//! deviation, the closer to the mean.
//!
//! # Formula
//!
//! SD = √( ∑\[(x - μ)^2\] )
//!
//! where:
//!
//! * `x` is the current value in a set.
//! * `μ` is the mean of the set.
//! * `∑` is the sum.

use crate::traits::{AsValue, InternalValue, Next, Period};
use crate::{Buffer, Num, TAError};
use tatk_derive::{InternalValue, Period};

/// Standard Deviation (SD/STDEV)
///
/// Measure of the amount of variation or dispersion between values. The lower the standard
/// deviation, the closer to the mean.
///
/// # Formula
///
/// SD = √( ∑\[(x - μ)^2\] )
///
/// where:
///
/// * `x` is the current value in a set.
/// * `μ` is the mean of the set.
/// * `∑` is the sum.
#[derive(Debug, InternalValue, Period)]
pub struct StandardDeviation {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// STDEV's current value.
    value: Num,
    /// Holds all of the current period's values.
    buffer: Buffer,
    /// Labels it as sample or population.
    is_sample: bool,
}

impl StandardDeviation {
    /// Creates a new standard deviation with the supplied period and initial data.
    ///
    /// ### Requirements:
    ///
    /// * Period must be greater than 0.
    /// * Data must have at least `period` elements.
    ///
    /// ## Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the STDEV from.
    /// * `is_sample` - If the data is a Sample or Population, default should be True.
    pub fn new(period: usize, data: &[Num], is_sample: bool) -> Result<Self, TAError> {
        // Check we can calculate Standard Deviation.
        if period < 1 {
            return Err(TAError::InvalidSize(String::from(
                "period cannot be less than 1 to calculate standard deviation",
            )));
        } else if data.len() < period {
            // Make sure we have enough data.
            return Err(TAError::InvalidData(String::from(
                "not enough data for period provided",
            )));
        }

        // Build the buffer from the data provided.
        let buffer: Buffer = match Buffer::from_array(period, data) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        Ok(Self {
            period,
            value: buffer.stdev(is_sample),
            buffer,
            is_sample,
        })
    }

    /// Current and most recent value calculated.
    pub fn value(&self) -> Num {
        self.value
    }

    /// Indicates either sample or population being used.
    pub fn is_sample(&self) -> bool {
        self.is_sample
    }
}

impl Next<Num> for StandardDeviation {
    /// Value for the next STDEV.
    type Output = Num;

    /// Supply an additional value to recalculate a new standard deviation.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        // Rotate the buffer.
        self.buffer.shift(value);

        // Calculate the new STDEV.
        self.value = self.buffer.stdev(self.is_sample());
        self.value
    }
}

impl<T> Next<T> for StandardDeviation
where
    T: AsValue,
{
    /// Value for the next STDEV.
    type Output = Num;

    /// Supply an additional value to recalculate a new STDEV.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.next(value.as_value())
    }
}
