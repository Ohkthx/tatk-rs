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
use crate::traits::{AsValue, Next, Period, Value};
use crate::{Buffer, Num, TAError};

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
#[derive(Debug)]
pub struct STDEV {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// STDEV's current value.
    value: Num,
    /// Holds all of the current period's values.
    buffer: Buffer,
    /// Labels it as sample or population.
    is_sample: bool,
}

impl STDEV {
    /// Creates a new standard deviation with the supplied period and initial data.
    ///
    /// Required: The initial data must be at least of equal size/length or greater than the period.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the STDEV from.
    /// * `is_sample` - If the data is a Sample or Population, default should be True.
    pub fn new(period: usize, data: &[Num], is_sample: bool) -> Result<Self, TAError> {
        // Make sure we have enough data.
        if data.len() < period {
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

    /// Indicates either sample or population being used.
    pub fn is_sample(&self) -> bool {
        self.is_sample
    }
}

impl Period for STDEV {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for STDEV {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl Next<Num> for STDEV {
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

impl<T> Next<T> for STDEV
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
