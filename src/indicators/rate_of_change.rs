//! Rate of Change (ROC), Measures percentage change in value.
//!
//! # Formula
//!
//! ROC = ((x - y) / y) * 100
//!
//! where:
//!
//! * `x` = current value (most recent)
//! * `y` = value `n` periods prior.
use crate::traits::{AsValue, Next, Period, Stats, Value};
use crate::{Buffer, Num, TAError};

/// Rate of Change (ROC), Measures percentage change in value.
///
/// # Formula
///
/// ROC = ((x - y) / y) * 100
///
/// where:
///
/// * `x` = current value (most recent)
/// * `y` = value `n` periods prior.
#[derive(Debug)]
pub struct Roc {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// ROC's current value.
    value: Num,
    /// Stasis values.
    values: Buffer,
    /// Holds all of the current period's values.
    buffer: Buffer,
}

impl Roc {
    /// Creates a new ROC with the supplied period and initial data.
    ///
    /// Required: The initial data must be at least of equal size/length or greater than the period.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the ROC from.
    pub fn new(period: usize, data: &[Num]) -> Result<Self, TAError> {
        // Make sure we have enough data.
        if data.len() < period + 1 {
            return Err(TAError::InvalidData(String::from(
                "not enough data for period provided",
            )));
        }

        // Stores previous closes / data points.
        let mut values: Buffer = match Buffer::from_array(period, &data[..period]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // Create the first value.
        let recent_value = data[period];
        let mut last_roc = Self::calculate(&recent_value, values.oldest());
        values.shift(recent_value);

        // Build the buffer from the data provided.
        let mut buffer: Buffer = match Buffer::from_array(period, &[last_roc]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // Process the remaining values.
        for v in data[period + 1..].iter() {
            last_roc = Self::calculate(v, values.oldest());
            buffer.shift(last_roc);
            values.shift(*v);
        }

        Ok(Self {
            period,
            value: last_roc,
            values,
            buffer,
        })
    }

    /// Calculates an ROC with newly provided datal.
    ///
    /// # Arguments
    ///
    /// * `value` - Current value / close.
    /// * `last` - Last value / close from 'n' periods.
    fn calculate(value: &Num, last: Num) -> Num {
        ((value - last) / last) * 100.0
    }
}

impl Period for Roc {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for Roc {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl Next<Num> for Roc {
    /// Next Value for the ROC.
    type Output = Num;

    /// Supply an additional value to recalculate a new ROC.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        self.value = Self::calculate(&value, self.values.oldest());

        self.buffer.shift(self.value);
        self.values.shift(value);
        self.value
    }
}

impl<T> Next<T> for Roc
where
    T: AsValue,
{
    /// Next Value for the ROC.
    type Output = Num;

    /// Supply an additional value to recalculate a new ROC.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.next(value.as_value())
    }
}

impl Stats for Roc {
    /// Obtains the total sum of the buffer for ROC.
    fn sum(&self) -> Num {
        self.buffer.sum()
    }

    /// Mean for the period of the ROC.
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
