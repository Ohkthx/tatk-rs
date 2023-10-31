//! McGinley Dynamic (MD)
//!
//! # Formula
//!
//! MD = MD_prev + \[ (x - MD_prev) / (k * n * (x / MD_prev) ^ 4) \]
//!
//! where:
//!
//! * `x` = current close (most recent)
//! * `k` = modifies the period, normally 0.6
//! * `n` = period

use crate::traits::{AsValue, InternalValue, Next, Period, Stats};
use crate::{Buffer, Num, TAError};
use tatk_derive::{InternalValue, Period};

/// McGinley Dynamic (MD)
///
/// # Formula
///
/// MD = MD_prev + \[ (x - MD_prev) / (k * n * (x / MD_prev) ^ 4) \]
///
/// where:
///
/// * `x` = current close (most recent)
/// * `k` = modifies the period, normally 0.6
/// * `n` = period
#[derive(Debug, InternalValue, Period)]
pub struct McGinleyDynamic {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// Constant for period modification.
    k: Num,
    /// MD's current value.
    value: Num,
    /// Holds all of the current period's values.
    buffer: Buffer,
}

impl McGinleyDynamic {
    /// Creates a new McGinley Dynamic with the supplied period and initial data.
    ///
    /// ### Requirements:
    ///
    /// * Period must be greater than 1.
    /// * Data must have at least `period + 1` elements.
    ///
    /// ## Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the MD from.
    /// * `k` - Constant used to modify selected period. Default: 0.6
    pub fn new(period: usize, data: &[Num], k: Num) -> Result<Self, TAError> {
        // Check we can calculate McGinley Dynamic Indicator.
        if period < 2 {
            return Err(TAError::InvalidSize(String::from(
                "period cannot be less than 2 to calculate McGinely dynamic",
            )));
        } else if data.len() < period + 1 {
            // Make sure we have enough data.
            return Err(TAError::InvalidData(String::from(
                "not enough data for period provided",
            )));
        }

        // First MD value.
        let mut last_md = data[0];

        // Buffer will hold last `period` MDs.
        let mut buffer = match Buffer::from_array(period, &[last_md]) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Calculate the remainder of the data set.
        for v in data[1..].iter() {
            last_md = Self::calculate(k, last_md, *v, period);
            buffer.shift(last_md);
        }

        Ok(Self {
            period,
            k,
            value: last_md,
            buffer,
        })
    }

    /// Current and most recent value calculated.
    pub fn value(&self) -> Num {
        self.value
    }

    /// Calculates an MD with newly provided data and the last MD.
    ///
    /// # Arguments
    ///
    /// * `k` - Used to modify the period.
    /// * `last_md` - Last MD calculated.
    /// * `value` - Most recent value.
    /// * `period` - Size of the period / window used.
    fn calculate(k: Num, last_md: Num, value: Num, period: usize) -> Num {
        let numerator = value - last_md;
        let denominator = k * (period as Num) * Num::powi(value / last_md, 4);
        last_md + (numerator / denominator)
    }
}

impl Next<Num> for McGinleyDynamic {
    /// Next value for the McGinley Dynamic.
    type Output = Num;

    /// Supply an additional value to recalculate a new McGinley Dynamic.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        self.value = Self::calculate(self.k, self.value(), value, self.period());

        // Rotate the buffer.
        self.buffer.shift(self.value);
        self.value
    }
}

impl<T> Next<T> for McGinleyDynamic
where
    T: AsValue,
{
    /// Next value for the McGinley Dynamic.
    type Output = Num;

    /// Supply an additional value to recalculate a new MD.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.next(value.as_value())
    }
}

impl Stats for McGinleyDynamic {
    /// Obtains the total sum of the buffer for MD.
    fn sum(&self) -> Num {
        self.buffer.sum()
    }

    /// Mean for the period of the MD.
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
