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
use super::SMA;
use crate::error::TAError;

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
pub struct EMA {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// Current value for the EMA.
    value: f64,
    /// SMA for the same period
    sma: SMA,
}

impl EMA {
    /// Creates a new EMA with the supplied period and initial data.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the EMA from.
    pub fn new(period: usize, data: &[f64]) -> Result<Self, TAError> {
        // Get the starting seed of data (SMA for the period at the beginning.)
        let end_idx = period - 1;
        let mut last_ema: f64 = match SMA::calculate(period, 0, end_idx, data) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        // Calculate the remainder of the datas EMA, using the prior EMA.
        for value in data[end_idx + 1..].iter() {
            last_ema = Self::calculate(period, &last_ema, value);
        }

        let sma: SMA = match SMA::new(period, data) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        Ok(Self {
            period,
            value: last_ema,
            sma,
        })
    }

    /// Period (window) for the samples.
    pub fn period(&self) -> usize {
        self.period
    }

    /// Current and most recent value calculated.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns true if the EMA is above an SMA of the same period.
    pub fn is_above(&self) -> bool {
        self.value > self.sma.value()
    }

    /// Returns true if the EMA is below an SMA of the same period.
    pub fn is_below(&self) -> bool {
        self.value < self.sma.value()
    }

    /// Supply an additional value to recalculate a new EMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    pub fn next(&mut self, value: f64) -> f64 {
        // Progress the SMA by a value.
        self.sma.next(value);

        // Get the next EMA value.
        self.value = Self::calculate(self.period, &self.value, &value);
        self.value
    }

    /// Calculates an EMA with newly provided data and the last EMA..
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `last_ema` - Last EMA calculated.
    /// * `value` - Most recent value.
    pub(crate) fn calculate(period: usize, last_ema: &f64, value: &f64) -> f64 {
        let k = 2.0 / (period + 1) as f64;
        (value - last_ema) * k + last_ema
    }
}
