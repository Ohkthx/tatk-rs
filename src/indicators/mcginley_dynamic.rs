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
use crate::traits::{AsValue, Next, Period, Stats, Value};
use crate::{Buffer, Num, TAError};

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
#[derive(Debug)]
pub struct MD {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// Constant for period modification.
    k: Num,
    /// MD's current value.
    value: Num,
    /// Holds all of the current period's values.
    buffer: Buffer,
}

impl MD {
    /// Creates a new MD with the supplied period and initial data.
    ///
    /// Required: The initial data must be at least of equal size/length or greater than the period.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the MD from.
    /// * `k` - Constant used to modify selected period. Default: 0.6
    pub fn new(period: usize, data: &[Num], k: Num) -> Result<Self, TAError> {
        // Make sure we have enough data.
        if data.len() < period + 1 {
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

impl Period for MD {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for MD {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl Next<Num> for MD {
    /// Next Value for the MD.
    type Output = Num;

    /// Supply an additional value to recalculate a new MD.
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

impl<T> Next<T> for MD
where
    T: AsValue,
{
    /// Next Value for the MD.
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

impl Stats for MD {
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
