//! Average True Range (ATR), for a `n` true ranges.
//!
//! # Formula
//!
//! ATR = [ ATR_prev + TR ] / n
//!
//! where:
//!
//! * `TR` = true range
//! * `n` = period
use super::true_range::TrueRangeData;
use super::TrueRange;
use crate::traits::{Close, High, Low, Next, Period, Stats, Value};
use crate::{Buffer, Num, TAError};

/// Average True Range (ATR), for a `n` true ranges.
///
/// # Formula
///
/// ATR = [ ATR_prev + TR ] / n
///
/// where:
///
/// * `TR` = true range
/// * `n` = period
#[derive(Debug)]
pub struct AverageTrueRange {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// ATR's current value.
    value: Num,
    /// True Range used for calculations.
    true_range: TrueRange,
    /// Holds `period` amount of generated ATRs.
    buffer: Buffer,
}

impl AverageTrueRange {
    /// Creates a new ATR with the supplied period and initial data.
    ///
    /// ### Requirements:
    ///
    /// * Period must be greater than 0.
    /// * Data must have at least `period + 1` elements.
    ///
    /// ## Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the ATR from.
    pub fn new<T>(period: usize, data: &[T]) -> Result<Self, TAError>
    where
        T: High + Low + Close,
    {
        // Check we can calculate ATR.
        if period < 1 {
            return Err(TAError::InvalidSize(String::from(
                "period cannot be less than 1 to calculate average true range",
            )));
        } else if data.len() < period + 1 {
            // Make sure we have enough data.
            return Err(TAError::InvalidData(String::from(
                "not enough data to calculate average true range",
            )));
        }

        // Create the first `n` true ranges.
        let mut tr = match TrueRange::new(period, &data[..(period + 1)]) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Initial value.
        let mut atr_value = tr.mean();

        // Buffer will hold last `n` ATRs.
        let mut buffer = match Buffer::from_array(period, &[atr_value]) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Calculate the remainder of ATRs.
        for value in data[(period + 1)..].iter() {
            let tr_next = tr.next(value);
            atr_value = Self::calculate(tr_next, period, atr_value);
            buffer.shift(atr_value);
        }

        Ok(Self {
            period,
            true_range: tr,
            value: atr_value,
            buffer,
        })
    }

    /// Caclulates a new ATR, requring a prior close.
    fn calculate(tr_value: Num, period: usize, last_atr: Num) -> Num {
        let top = (last_atr * (period as Num - 1.0)) + tr_value;
        top / period as Num
    }
}

impl Period for AverageTrueRange {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for AverageTrueRange {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl<T> Next<T> for AverageTrueRange
where
    T: High + Low + Close,
{
    /// Next Value for the ATR.
    type Output = Num;

    /// Supply an additional value to recalculate a new ATR.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        let period = self.period();
        let last = self.value();
        let tr_value = self.true_range.next(&value);
        self.value = Self::calculate(tr_value, period, last);

        // Rotate the buffer.
        self.buffer.shift(self.value());
        self.value
    }
}

impl Next<(Num, Num, Num)> for AverageTrueRange {
    /// Next Value for the ATR.
    type Output = Num;

    /// Supply an additional value to recalculate a new ATR.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    ///     * 0 = High
    ///     * 1 = Low
    ///     * 2 = Close
    fn next(&mut self, value: (Num, Num, Num)) -> Self::Output {
        let v = TrueRangeData {
            0: value.0, // High
            1: value.1, // Low
            2: value.2, // Close
        };

        self.next(v)
    }
}

impl Stats for AverageTrueRange {
    /// Obtains the total sum of the buffer for ATR.
    fn sum(&self) -> Num {
        self.buffer.sum()
    }

    /// Mean for the period of the ATR.
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
