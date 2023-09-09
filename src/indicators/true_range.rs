//! True Range (TR), Maximum of three price ranges.
//!
//! # Formula
//!
//! TR = MAX[ |H - L|, |H - C|, |L - C| ]
//!
//! where:
//!
//! * `H` = highest value for the data point / candle.
//! * `L` = lowest value for the data point / candle.
//! * `C` = last close prior to this data point.
use crate::traits::{Close, High, Low, Next, Period, Stats, Value};
use crate::{Buffer, Num, TAError};

/// Used for conversions. Holds High (0), Low (1), and Close (2) values.
#[derive(Copy, Clone)]
pub(crate) struct TRData(pub Num, pub Num, pub Num);

// Highest value.
impl High for TRData {
    fn high(&self) -> Num {
        self.0
    }
}

// Lowest value.
impl Low for TRData {
    fn low(&self) -> Num {
        self.1
    }
}

// Closing value.
impl Close for TRData {
    fn close(&self) -> Num {
        self.2
    }
}

/// True Range (TR), Maximum of three price ranges.
///
/// # Formula
///
/// TR = MAX[ |H - L|, |H - C|, |L - C| ]
///
/// where:
///
/// * `H` = highest value for the data point / candle.
/// * `L` = lowest value for the data point / candle.
/// * `C` = last close prior to this data point.
#[derive(Debug)]
pub struct TR {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// TR's current value.
    value: Num,
    /// Last close used not used for calculation.
    last_close: Num,
    /// Holds `period` amount of generated TRs.
    buffer: Buffer,
}

impl TR {
    /// Creates a new TR with the supplied period and initial data.
    ///
    /// Required: The initial data must contain at least 2 data points.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the TR from.
    pub fn new<T>(period: usize, data: &[T]) -> Result<Self, TAError>
    where
        T: Close + Low + High,
    {
        // Make sure we have enough data. Requires additional data point for `last_close`
        if data.len() < period + 1 {
            return Err(TAError::InvalidData(String::from(
                "not enough data to calculate true range",
            )));
        }

        // First close and TR to use.
        let mut last_close = data[0].close();
        let mut last_tr = Self::calculate(&data[1], &mut last_close);

        // Buffer will hold last `period` of TRs.
        let mut buffer = match Buffer::from_array(period, &[last_tr]) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Calculate the remainder of TRs.
        if data.len() > 2 {
            for v in data[2..].iter() {
                last_tr = Self::calculate(v, &mut last_close);
                buffer.shift(last_tr);
            }
        }

        Ok(Self {
            period,
            last_close,
            value: last_tr,
            buffer,
        })
    }

    /// Calculates a new TR, requring a prior close.
    /// * 0 = High
    /// * 1 = Low
    /// * 2 = Close
    fn calculate<T>(value: &T, last_close: &mut Num) -> Num
    where
        T: High + Low + Close,
    {
        let hl = (value.high() - value.low()).abs();
        let hc = (value.high() - *last_close).abs();
        let lc = (value.low() - *last_close).abs();

        *last_close = value.close();
        hl.max(hc.max(lc))
    }
}

impl Period for TR {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for TR {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl<T> Next<&T> for TR
where
    T: High + Low + Close,
{
    /// Next Value for the TR.
    type Output = Num;

    /// Supply an additional value to recalculate a new TR.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: &T) -> Self::Output {
        self.value = Self::calculate(value, &mut self.last_close);

        // Rotate the buffer.
        self.buffer.shift(self.value());
        self.value
    }
}

impl Next<(Num, Num, Num)> for TR {
    /// Next Value for the TR.
    type Output = Num;

    /// Supply an additional value to recalculate a new TR.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    ///     * 0 = High
    ///     * 1 = Low
    ///     * 2 = Close
    fn next(&mut self, value: (Num, Num, Num)) -> Self::Output {
        let v = TRData {
            0: value.0, // High
            1: value.1, // Low
            2: value.2, // Close
        };

        self.next(&v)
    }
}

impl Stats for TR {
    /// Obtains the total sum of the buffer for TR.
    fn sum(&self) -> Num {
        self.buffer.sum()
    }

    /// Mean for the period of the TR.
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
