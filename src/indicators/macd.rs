//! Moving Average Convergence and Divergence (MACD)
//!
//! # Formula
//!
//! MACD = SHORT_EMA - LONG_EMA
//!
//! MACD = x - y
//!
//! where:
//!
//! * `x` = Short EMA of period `n`
//! * `y` = Long EMA of period `n`
use super::EMA;
use crate::error::TAError;

/// Moving Average Convergence and Divergence (MACD)
///
/// # Formula
///
/// MACD = SHORT_EMA - LONG_EMA
///
/// MACD = x - y
///
/// where:
///
/// * `x` = Short EMA of period `n`
/// * `y` = Long EMA of period `n`
#[derive(Debug)]
pub struct MACD {
    /// MACD's current value.
    value: f64,
    /// Short EMA
    ema_short: EMA,
    /// Long EMA
    ema_long: EMA,
    /// Signal Line, EMA of MACD values.
    ema_signal: EMA,
    /// If the MACD crossed the signal.
    crossed: bool,
}

impl MACD {
    /// Creates a new MACD with the supplied period and initial data. Often the short line is
    /// period of 12, long is a period of 26, and signal is period of 9.
    ///
    /// # Arguments
    ///
    /// * `short` - Period of the short EMA.
    /// * `long` - Period of the long EMA.
    /// * `signal` - Period of the signal EMA.
    /// * `data` - Array of values to create the MACD from.
    pub fn new(short: usize, long: usize, signal: usize, data: &[f64]) -> Result<Self, TAError> {
        if short > long {
            return Err(TAError::InvalidLine("long".to_string()));
        } else if data.len() < short {
            return Err(TAError::InvalidLine("data".to_string()));
        } else if data.len() < signal {
            return Err(TAError::InvalidLine("data".to_string()));
        }

        // Build short EMA up to the long.
        let mut ema_short = match EMA::new(short, &data[..long]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // Build long EMA.
        let mut ema_long = match EMA::new(long, &data[..long]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // Add the first value.
        let mut signals: Vec<f64> = vec![ema_short.value() - ema_long.value()];

        // Process the remainder of the data, building a signal line.
        for n in long..data.len() {
            let short_value = ema_short.next(data[n].clone());
            let long_value = ema_long.next(data[n].clone());

            signals.push(short_value - long_value);
        }

        // Build signal EMA of MACDs.
        let ema_signal = match EMA::new(signal, &signals) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        Ok(Self {
            value: ema_short.value() - ema_long.value(),
            ema_short,
            ema_long,
            ema_signal,
            crossed: false,
        })
    }

    /// Current and most recent value calculated.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Current and most recent signal value calculated.
    pub fn signal_value(&self) -> f64 {
        self.ema_signal.value()
    }

    /// Check if the value crossed the signal.
    pub fn crossed(&self) -> bool {
        self.crossed
    }

    /// Returns true if the value is above the signal.
    pub fn is_above(&self) -> bool {
        self.value > self.signal_value()
    }

    /// Returns true if the value is below the signal.
    pub fn is_below(&self) -> bool {
        self.value < self.signal_value()
    }

    /// Supply an additional value to recalculate a new MACD.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    pub fn next(&mut self, value: f64) -> f64 {
        let was_below: bool = self.is_below();

        let short_value = self.ema_short.next(value);
        let long_value = self.ema_long.next(value);

        // Calculate the new MACD and signal.
        self.value = short_value - long_value;
        self.ema_signal.next(self.value);

        // Update if it crossed the signal or not.
        if was_below && self.is_below() {
            self.crossed = false;
        } else if !was_below && self.is_above() {
            self.crossed = false;
        } else {
            self.crossed = true;
        }

        self.value
    }
}
