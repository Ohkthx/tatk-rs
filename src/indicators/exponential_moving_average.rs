//! Exponential Moving Average (EMA)
//!
//! # Formula
//!
//! EMA = [(x - y) * k] + y
//!
//! where:
//!
//! * `x` = current value (most recent)
//! * `y` = last EMA
//! * `k` = 2 * (n + 1)
//! * `n` = period
use crate::traits::{AsValue, Next, Period, Stats, Value};
use crate::{Buffer, Num, TAError};

/// Initializes an EMA.
///
/// # Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the EMA from.
#[macro_export]
macro_rules! ema {
    ($period:expr, $data:expr) => {
        $crate::indicators::Ema::new($period, $data)
    };
}

/// Exponential Moving Average (EMA). More recent data is weighted heavier than older data.
///
/// # Formula
///
/// EMA = [(x - y) * k] + y
///
/// where:
/// * `x` = current value (most recent)
/// * `y` = last EMA
/// * `k` = 2 * (n + 1)
/// * `n` = period
#[derive(Debug)]
pub struct Ema {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// Current value for the EMA.
    value: Num,
    /// Holds `period` amount of generated EMAs.
    buffer: Buffer,
    /// Smoothing value.
    k: Num,
}

impl Ema {
    /// Creates a new EMA with the supplied period and initial data.
    ///
    /// Required: The initial data must be at least of equal size/length or greater than the period.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the EMA from.
    pub fn new(period: usize, data: &[Num]) -> Result<Self, TAError> {
        // Make sure we have enough data.
        if data.len() < period {
            return Err(TAError::InvalidData(String::from(
                "not enough data for period provided",
            )));
        }

        // Temporary buffer to get seed SMA for EMA.
        let mut last_ema = match Buffer::from_array(period, &data[..period]) {
            Ok(v) => v.mean(),
            Err(error) => return Err(error),
        };

        // Buffer will hold last `period` EMAs.
        let mut buffer = match Buffer::from_array(period, &[last_ema]) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Smoothing factor.
        let k: Num = 2.0 / (period + 1) as Num;

        // Calculate the remainder of the datas EMA, using the prior EMA.
        for value in data[period..].iter() {
            last_ema = Self::calculate(&k, &last_ema, value);
            buffer.shift(last_ema);
        }

        Ok(Self {
            period,
            value: last_ema,
            buffer,
            k,
        })
    }

    /// Smoothing factor.
    fn k(&self) -> &Num {
        &self.k
    }

    /// Calculates an EMA with newly provided data and the last EMA.
    ///
    /// # Arguments
    ///
    /// * `k` - Smoothing value for the EMA.
    /// * `last_ema` - Last EMA calculated.
    /// * `value` - Most recent value.
    fn calculate(k: &Num, last_ema: &Num, value: &Num) -> Num {
        (value - last_ema) * k + last_ema
    }
}

impl Period for Ema {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for Ema {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl Next<Num> for Ema {
    /// Next Value for the EMA.
    type Output = Num;

    /// Supply an additional value to recalculate a new EMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        // Get the next EMA value.
        self.value = Self::calculate(self.k(), &self.value(), &value);
        self.buffer.shift(self.value());
        self.value
    }
}

impl<T> Next<T> for Ema
where
    T: AsValue,
{
    /// Next Value for the EMA.
    type Output = Num;

    /// Supply an additional value to recalculate a new EMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.next(value.as_value())
    }
}

impl Stats for Ema {
    /// Obtains the total sum of the buffer for EMA.
    fn sum(&self) -> Num {
        self.buffer.sum()
    }

    /// Mean for the period of the EMA.
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
