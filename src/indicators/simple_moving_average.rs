//! Simple Moving Average (SMA)
//!
//! Average moves within a period.
use crate::traits::{AsValue, Next, Period, Stats, Value};
use crate::{Buffer, Num, TAError};

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
    /// Required: The initial data must be at least of equal size/length or greater than the period.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the SMA from.
    pub fn new(period: usize, data: &[Num]) -> Result<Self, TAError> {
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
            value: buffer.mean(),
            buffer,
        })
    }
}

impl Period for SMA {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for SMA {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl Next<Num> for SMA {
    /// Next Value for the SMA.
    type Output = Num;

    /// Supply an additional value to recalculate a new SMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        // Rotate the buffer.
        self.buffer.shift(value);

        // Calculate the new SMA.
        self.value = self.sum() / self.period() as Num;
        self.value
    }
}

impl<T> Next<T> for SMA
where
    T: AsValue,
{
    /// Next Value for the SMA.
    type Output = Num;

    /// Supply an additional value to recalculate a new SMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.next(value.as_value())
    }
}

impl Stats for SMA {
    /// Obtains the total sum of the buffer for SMA.
    fn sum(&self) -> Num {
        self.buffer.sum()
    }

    /// Mean for the period of the SMA.
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
