//! Relative Strength Index (RSI)
//!
//! # Formula
//!
//! RSI (step1) = 100 - [100 / (1 + (x / y))]
//!
//! RSI (step2) = 100 - [100 / (1 + ((x * z + x1) / (y * z + y2)))]
//!
//! where:
//!
//! * `x` = Average gain over period.
//! * `y` = Average loss over period.
//! * `z` = Period - 1.
//! * `x1` = Most recent gain.
//! * `y1` = Most recent loss.
use crate::error::TAError;

/// Relative Strength Index (RSI)
///
/// # Formula
///
/// RSI (step1) = 100 - [100 / (1 + (x / y))]
///
/// RSI (step2) = 100 - [100 / (1 + ((  x * 13 + x1) / (y * 13 + y1)))]
///
/// where:
///
/// * `x` = Average gain over period.
/// * `y` = Average loss over period.
/// * `z` = Period - 1.
/// * `x1` = Most recent gain.
/// * `y1` = Most recent loss.
#[derive(Debug)]
pub struct RSI {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// RSI's current value.
    value: f64,
    /// Average gain percentage.
    gain_avg: f64,
    /// Average loss percentage.
    loss_avg: f64,
    /// Last value processed.
    last: f64,
    /// Oversold threshold.
    oversold: f64,
    /// Overbought threshold.
    overbought: f64,
}

impl RSI {
    /// Creates a new RSI with the supplied period and initial data.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the RSI from.
    pub fn new(period: usize, data: &[f64]) -> Result<Self, TAError> {
        if period + 1 > data.len() {
            return Err(TAError::InvalidArray);
        } else if period == 0 {
            return Err(TAError::InvalidPeriod);
        }

        let mut gains: f64 = 0.0;
        let mut losses: f64 = 0.0;
        let mut last: f64 = data[0].clone();

        // Generates the gains / losses for the first period of values. Unique and uses all gains /
        // losses for the first period as a seed value.
        for value in data[1..=period].iter() {
            let change = value - last;
            last = value.clone();
            if change > 0.0 {
                gains = gains + change;
            } else {
                losses = losses + change.abs();
            }
        }

        // These values will be updated by calculate, used to calculate period + 1.
        let mut last_gain: f64 = 0.0;
        let mut last_loss: f64 = 0.0;
        let mut value = Self::calculate(period, &mut last_gain, &mut last_loss, gains, losses);

        // Calculate remaining values. This uses the average + next value. It's a slightly
        // different calculation than the initial seed value for the RSI.
        if period < data.len() {
            for v in &data[(period + 1)..] {
                let change = v - last;
                let mut gain = 0.0;
                let mut loss = 0.0;

                if change > 0.0 {
                    gain = change;
                } else {
                    loss = change.abs();
                }

                value = Self::calculate(period, &mut last_gain, &mut last_loss, gain, loss);
                last = v.clone();
            }
        }

        Ok(Self {
            period,
            value,
            gain_avg: last_gain,
            loss_avg: last_loss,
            last,
            oversold: 20.0,
            overbought: 80.0,
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

    /// Changes the Oversold Threshold from the default (20.0)
    pub fn set_oversold(&mut self, oversold_value: f64) {
        self.oversold = oversold_value;
    }

    /// Changes the Overbought Threshold from the default (80.0)
    pub fn set_overbought(&mut self, overbought_value: f64) {
        self.overbought = overbought_value;
    }

    /// Checks if the RSI is currently within the oversold threshold (default 20.0)
    pub fn is_oversold(&self) -> bool {
        self.value < self.oversold
    }

    /// Checks if the RSI is currently within the overbought threshold (default 80.0)
    pub fn is_overbought(&self) -> bool {
        self.value > self.overbought
    }

    /// Supply an additional value to recalculate a new RSI.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    pub fn next(&mut self, value: f64) -> f64 {
        let mut gain = 0.0;
        let mut loss = 0.0;
        let change = value - self.last;

        if change > 0.0 {
            gain = change;
        } else {
            loss = change.abs();
        }

        // Calculate the new RSI.
        self.last = value;
        self.value = Self::calculate(
            self.period,
            &mut self.gain_avg,
            &mut self.loss_avg,
            gain,
            loss,
        );
        self.value
    }

    /// Calculates a RSI with the given data between two indexes.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `gain_avg` - Past calculation gain average.
    /// * `loss_avg` - Past calculation loss average.
    /// * `gain` - Most recent gain (>= 0).
    /// * `loss` - Most recent loss (>= 0).
    pub(crate) fn calculate(
        period: usize,
        gain_avg: &mut f64,
        loss_avg: &mut f64,
        gain: f64,
        loss: f64,
    ) -> f64 {
        let period_value = (period as f64) - 1.0;

        // Update the callers gain and loss averages.
        *gain_avg = (*gain_avg * period_value + gain) / period as f64;
        *loss_avg = (*loss_avg * period_value + loss) / period as f64;

        100.0 - (100.0 / (1.0 + (*gain_avg / *loss_avg)))
    }
}
