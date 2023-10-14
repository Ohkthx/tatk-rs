//! Variance (Var(X))
//!
//! Measure of dispersion between values.
//!
//! # Formula
//!
//! Var(X) = ∑\[(x - μ)^2\]
//!
//! where:
//!
//! * `x` is the current value in a set.
//! * `μ` is the mean of the set.
//! * `∑` is the sum.
use crate::traits::{AsValue, Next, Period, Value};
use crate::{Buffer, Num, TAError};

/// Variance (Var(X))
///
/// Measure of dispersion between values.
///
/// # Formula
///
/// Var(X) = ∑\[(x - μ)^2\]
///
/// where:
///
/// * `x` is the current value in a set.
/// * `μ` is the mean of the set.
/// * `∑` is the sum.
#[derive(Debug)]
pub struct Variance {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// Var(X)'s current value.
    value: Num,
    /// Holds all of the current period's values.
    buffer: Buffer,
    /// Labels it as sample or population.
    is_sample: bool,
}

impl Variance {
    /// Creates a new Var(X) with the supplied period and initial data.
    ///
    /// ### Requirements:
    ///
    /// * Period must be greater than 0.
    /// * Data must have at least `period` elements.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the Var(X) from.
    /// * `is_sample` - If the data is a Sample or Population, default should be True.
    pub fn new(period: usize, data: &[Num], is_sample: bool) -> Result<Self, TAError> {
        // Check we can calculate Variance.
        if period < 1 {
            return Err(TAError::InvalidSize(String::from(
                "period cannot be less than 1 to calculate variance",
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
            value: buffer.variance(is_sample),
            buffer,
            is_sample,
        })
    }

    /// Indicates either sample or population being used.
    pub fn is_sample(&self) -> bool {
        self.is_sample
    }
}

impl Period for Variance {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for Variance {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl Next<Num> for Variance {
    /// Value for the next Variance.
    type Output = Num;

    /// Supply an additional value to recalculate a new Var(X).
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        // Rotate the buffer.
        self.buffer.shift(value);

        // Calculate the new Var(X).
        self.value = self.buffer.variance(self.is_sample());
        self.value
    }
}

impl<T> Next<T> for Variance
where
    T: AsValue,
{
    /// Value for the next Variance.
    type Output = Num;

    /// Supply an additional value to recalculate a new Variance.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.next(value.as_value())
    }
}
