//! Simple Moving Average
//!
//! Average moves within a period.
use crate::buffer::Buffer;
use crate::error::TAError;

/// Simple Moving Average (SMA), the average within a period that moves as data is added.
#[derive(Debug)]
pub struct SMA {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// SMA's current value.
    value: f64,
    /// Total sum of the buffer.
    sum: f64,
    /// Holds all of the current period's values.
    buffer: Buffer<f64>,
}

impl SMA {
    /// Creates a new SMA with the supplied period and initial data.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the SMA from.
    pub fn new(period: usize, data: &[f64]) -> Result<Self, TAError> {
        let start_idx = data.len() - period;
        let end_idx = data.len() - 1;

        // Calculate the SMA of the last period of the data.
        let sma = match Self::calculate(period, start_idx, end_idx, data) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Build the buffer from the data provided.
        let buffer: Buffer<f64> = match Buffer::from_array(period, data) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        Ok(Self {
            period,
            value: sma,
            sum: buffer.queue().iter().sum::<f64>(),
            buffer,
        })
    }

    /// Period (window) for the samples.
    pub fn period(&self) -> usize {
        self.period
    }

    /// Current and most recent value calculated.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Supply an additional value to recalculate a new SMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    pub fn next(&mut self, value: f64) -> f64 {
        // Rotate the buffer.
        let last_value = self.buffer.shift(value);

        // Calculate the new SMA.
        self.value = (self.sum - last_value + value) / self.period as f64;
        self.value
    }

    /// Calculates a SMA with the given data between two indexes.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `start_idx` - Starting index to begin calculations.
    /// * `end_idx` - Ending index to stop calculations.
    /// * `data` - Array of values to create the SMA from.
    pub(crate) fn calculate(
        period: usize,
        start_idx: usize,
        end_idx: usize,
        data: &[f64],
    ) -> Result<f64, TAError> {
        if period == 0 {
            // Period cannot be 0.
            return Err(TAError::InvalidPeriod);
        } else if start_idx > end_idx {
            // Starting index must be less than ending index.
            return Err(TAError::InvalidIndex(start_idx, end_idx));
        } else if data.len() < end_idx {
            // Prevents out-of-bounds access.
            return Err(TAError::InvalidArray);
        } else if end_idx - start_idx != period - 1 {
            // Ensures the correct period is being used with the indexes.
            return Err(TAError::InvalidPeriod);
        }

        let values_sum: f64 = data[start_idx..=end_idx].iter().sum();
        Ok(values_sum / period as f64)
    }
}
