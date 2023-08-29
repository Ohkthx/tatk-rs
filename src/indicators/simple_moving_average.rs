//! Simple Moving Average (SMA)
//!
//! Average moves within a period.
use crate::buffer::Buffer;
use crate::{error::TAError, Num};

/// Simple Moving Average (SMA), the average within a period that moves as data is added.
#[derive(Debug)]
pub struct SMA {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// SMA's current value.
    value: Num,
    /// Holds all of the current period's values.
    buffer: Buffer,
}

impl SMA {
    /// Creates a new SMA with the supplied period and initial data.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the SMA from.
    pub fn new(period: usize, data: &[Num]) -> Result<Self, TAError> {
        let start_idx = data.len() - period;
        let end_idx = data.len() - 1;

        // Calculate the SMA of the last period of the data.
        let sma = match Self::calculate(period, start_idx, end_idx, data) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Build the buffer from the data provided.
        let buffer: Buffer = match Buffer::from_array(period, data) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        Ok(Self {
            period,
            value: sma,
            buffer,
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

    /// Obtains the total sum of the buffer for SMA.
    pub(crate) fn sum(&self) -> Num {
        self.buffer.sum()
    }

    /// Supply an additional value to recalculate a new SMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    pub fn next(&mut self, value: Num) -> Num {
        // Rotate the buffer.
        self.buffer.shift(value);

        // Calculate the new SMA.
        self.value = self.sum() / self.period as Num;
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
        data: &[Num],
    ) -> Result<Num, TAError> {
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

        let values_sum: Num = data[start_idx..=end_idx].iter().sum();
        Ok(values_sum / period as Num)
    }
}
