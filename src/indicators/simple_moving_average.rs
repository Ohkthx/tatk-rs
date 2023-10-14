//! Simple Moving Average (SMA)
//!
//! Average moves within a period.
use crate::traits::{AsValue, Next, Period, Stats, Value};
use crate::{Buffer, Num, TAError};

/// Simple Moving Average (SMA), the average within a period that moves as data is added.
#[derive(Debug)]
pub struct SimpleMovingAverage {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// SMA's current value.
    value: Num,
    /// Holds all of the current period's values.
    buffer: Buffer,
}

impl SimpleMovingAverage {
    /// Creates a new SMA with the supplied period and initial data.
    ///
    /// ### Requirements:
    ///
    /// * Period must be greater than 0.
    /// * Data must have at least `period` elements.
    ///
    /// ## Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the SMA from.
    pub fn new(period: usize, data: &[Num]) -> Result<Self, TAError> {
        // Check we can calculate SMA.
        if period < 1 {
            return Err(TAError::InvalidSize(String::from(
                "period cannot be less than 1 to calculate simple moving average",
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
            value: buffer.mean(),
            buffer,
        })
    }
}

impl Period for SimpleMovingAverage {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for SimpleMovingAverage {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl Next<Num> for SimpleMovingAverage {
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

impl<T> Next<T> for SimpleMovingAverage
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

impl Stats for SimpleMovingAverage {
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
