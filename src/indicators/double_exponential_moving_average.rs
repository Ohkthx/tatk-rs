//! Double Exponential Moving Average (DEMA)
//!
//! # Formula
//!
//! DEMA = (2 * EMA(n)) - EMA(EMA(n))
//!
//! DEMA = (2 * x) - y
//!
//! where:
//!
//! * `x` = \[EMA(n)\] Current EMA of period `n`
//! * `y` = \[EMA(EMA(n))\] EMA of EMA(n)
//! * `n` = period
use super::EMA;
use crate::traits::{Line, Stats};
use crate::{Buffer, Num, TAError};

/// Double Exponential Moving Average (DEMA)
///
/// # Formula
///
/// DEMA = (2 * EMA(n)) - EMA(EMA(n))
///
/// DEMA = (2 * x) - y
///
/// where:
///
/// * `x` = \[EMA(n)\] Current EMA of period `n`
/// * `y` = \[EMA(EMA(n))\] EMA of EMA(n)
/// * `n` = period
#[derive(Debug)]
pub struct DEMA {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// DEMA's current value.
    value: Num,
    /// EMA(n), EMA of values / samples provided.
    ema_n: EMA,
    /// EMA(EMA(n)), EMA of EMA(n).
    ema_ema_n: EMA,
    /// Holds `period` amount of generated DEMAs.
    buffer: Buffer,
}

impl DEMA {
    /// Creates a new DEMA with the supplied period and initial data.
    ///
    /// Required: The initial data must be at least of equal size/length or greater than the period.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the DEMA from.
    pub fn new(period: usize, data: &[Num]) -> Result<Self, TAError> {
        // Make sure we have enough data.
        if data.len() < (period * 2) - 1 {
            return Err(TAError::InvalidData(String::from(
                "not enough data for period provided",
            )));
        }

        // Build EMA(n) from first 'n' samples (period amount).
        let mut ema_n = match EMA::new(period, &data[..period]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // n EMA(n), build it manually because we need to catch the output.
        let mut n_ema_n: Vec<Num> = vec![ema_n.value()];
        for n in period..((period * 2) - 1) {
            n_ema_n.push(ema_n.next(data[n].clone()))
        }

        // EMA of EMA(n)
        let mut ema_ema_n = match EMA::new(period, &n_ema_n) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // Buffer will old processed DEMAs
        let mut value = (2.0 * ema_n.value()) - ema_ema_n.value();
        let mut buffer = match Buffer::from_array(period, &[value]) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Calculate the remainder data points.
        for n in ((period * 2) - 1)..data.len() {
            let eman: Num = ema_n.next(data[n].clone());

            // Calculate the new DEMA.
            value = (2.0 * eman) - ema_ema_n.next(eman);
            buffer.shift(value);
        }

        Ok(Self {
            period,
            value,
            ema_n,
            ema_ema_n,
            buffer,
        })
    }
}

impl Line for DEMA {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }

    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }

    /// Supply an additional value to recalculate a new DEMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Num {
        let ema: Num = self.ema_n.next(value);

        // Calculate the new DEMA.
        self.value = (2.0 * ema) - self.ema_ema_n.next(ema);
        self.buffer.shift(self.value());
        self.value
    }
}

impl Stats for DEMA {
    /// Obtains the total sum of the buffer for DEMA.
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
