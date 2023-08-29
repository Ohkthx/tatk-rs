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
use crate::buffer::Buffer;
use crate::{error::TAError, Num};

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
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the STDEV from.
    /// * `is_sample` - If the data is a Sample or Population, default should be True.
    pub fn new(period: usize, data: &[Num], is_sample: bool) -> Result<Self, TAError> {
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

    /// Period (window) for the samples.
    pub fn period(&self) -> usize {
        self.period
    }

    /// Current and most recent value calculated.
    pub fn value(&self) -> Num {
        self.value
    }

    /// Supply an additional value to recalculate a new standard deviation.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    pub fn next(&mut self, value: Num) -> Num {
        // Rotate the buffer.
        self.buffer.shift(value);

        // Calculate the new STDEV.
        self.value = self.buffer.stdev(self.is_sample);
        self.value
    }
}
