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
//! * `x` = [EMA(n)] Current EMA of period `n`
//! * `y` = [EMA(EMA(n))] EMA of EMA(n)
//! * `n` = period
use super::EMA;
use crate::error::TAError;

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
/// * `x` = [EMA(n)] Current EMA of period `n`
/// * `y` = [EMA(EMA(n))] EMA of EMA(n)
/// * `n` = period
#[derive(Debug)]
pub struct DEMA {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// DEMA's current value.
    value: f64,
    /// EMA(n), EMA of values / samples provided.
    ema_n: EMA,
    /// EMA(EMA(n)), EMA of EMA(n).
    ema_ema_n: EMA,
}

impl DEMA {
    /// Creates a new DEMA with the supplied period and initial data.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the DEMA from.
    pub fn new(period: usize, data: &[f64]) -> Result<Self, TAError> {
        if data.len() < (period * 2) - 1 {
            return Err(TAError::InvalidArray);
        }

        // Build EMA(n) from first 'n' samples.
        let mut ema1 = match EMA::new(period, &data[..period]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // EMA(n), build it manually because we need to catch the output.
        let mut emas: Vec<f64> = vec![ema1.value()];
        for n in period..data.len() {
            emas.push(ema1.next(data[n].clone()))
        }

        // EMA of EMA(n)
        let ema2 = match EMA::new(period, &emas) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        Ok(Self {
            period,
            value: (2.0 * ema1.value()) - ema2.value(),
            ema_n: ema1,
            ema_ema_n: ema2,
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

    /// Supply an additional value to recalculate a new DEMA.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    pub fn next(&mut self, value: f64) -> f64 {
        let ema1 = self.ema_n.next(value);
        let ema2 = self.ema_ema_n.next(ema1);

        // Calculate the new DEMA.
        self.value = (2.0 * ema1) - ema2;
        self.value
    }
}
