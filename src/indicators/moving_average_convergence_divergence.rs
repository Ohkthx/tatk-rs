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
use crate::traits::{AsValue, Next, Period, Value};
use crate::{Num, TAError};

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
    value: Num,
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
    /// Required: The initial data must be at least of equal size/length or greater than the long
    /// size.
    ///
    /// # Arguments
    ///
    /// * `short` - Period of the short EMA.
    /// * `long` - Period of the long EMA.
    /// * `signal` - Period of the signal EMA.
    /// * `data` - Array of values to create the MACD from.
    pub fn new(short: usize, long: usize, signal: usize, data: &[Num]) -> Result<Self, TAError> {
        if short > long {
            return Err(TAError::InvalidSize(String::from(
                "short period longer than long period provided",
            )));
        } else if data.len() < signal {
            return Err(TAError::InvalidSize(String::from(
                "amount of data less than signal period provided",
            )));
        } else if data.len() < long {
            return Err(TAError::InvalidSize(String::from(
                "amount of data less than long period provided",
            )));
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
        let mut signals: Vec<Num> = vec![ema_short.value() - ema_long.value()];

        // Process the remainder of the data, building a signal line.
        for v in data[long..].iter() {
            let short_value = ema_short.next(*v);
            let long_value = ema_long.next(*v);

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

    /// Current and most recent signal value calculated.
    pub fn signal_value(&self) -> Num {
        self.ema_signal.value()
    }

    /// Check if the value crossed the signal.
    pub fn crossed(&self) -> bool {
        self.crossed
    }

    /// Returns true if the value is above the signal.
    pub fn is_above(&self) -> bool {
        self.value() > self.signal_value()
    }

    /// Returns true if the value is below the signal.
    pub fn is_below(&self) -> bool {
        self.value() < self.signal_value()
    }
}

impl Period for MACD {
    /// Period (window) for the signal.
    fn period(&self) -> usize {
        self.ema_signal.period()
    }
}

impl Value for MACD {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl Next<Num> for MACD {
    /// Signal, Short, and Long values,
    type Output = (Num, Num, Num);

    /// Supply an additional value to recalculate a new MACD.
    ///
    /// # Returns
    ///
    /// * (`Signal`, `Short`, `Long`)
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        let was_below: bool = self.is_below();

        let short_value = self.ema_short.next(value);
        let long_value = self.ema_long.next(value);

        // Calculate the new MACD and signal.
        self.value = short_value - long_value;
        self.ema_signal.next(self.value());

        // Update if it crossed the signal or not.
        if was_below && self.is_below() {
            self.crossed = false;
        } else if !was_below && self.is_above() {
            self.crossed = false;
        } else {
            self.crossed = true;
        }

        (self.value, short_value, long_value)
    }
}

impl<T> Next<T> for MACD
where
    T: AsValue,
{
    /// Signal, Short, and Long values,
    type Output = (Num, Num, Num);

    /// Supply an additional value to recalculate a new MACD.
    ///
    /// # Returns
    ///
    /// * (`Signal`, `Short`, `Long`)
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.next(value.as_value())
    }
}
