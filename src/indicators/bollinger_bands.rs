//! Bollinger Bands (BB / BBands)
//!
//! # Formula
//!
//! BBands_lower = SMA - (σ * d)
//!
//! BBands_upper = SMA + (σ * d)
//!
//! where:
//!
//! * `SMA` is the moving average of a period.
//! * `σ` is the standard deviation of the period.
//! * `d` is the distance from the SMA to calculate.
use super::SimpleMovingAverage;
use crate::traits::{Next, Period, Stats, Value};
use crate::{Num, TAError};

/// Bollinger Bands (BBands). More recent data is weighted heavier than older data.
///
/// # Formula
///
/// BBands_lower = SMA - (σ * d)
///
/// BBands_upper = SMA + (σ * d)
///
/// where:
///
/// * `SMA` is the moving average of a period.
/// * `σ` is the standard deviation of the period.
/// * `d` is the distance from the SMA to calculate.
#[derive(Debug)]
pub struct BollingerBands<L>
where
    L: Value + Period + Stats,
{
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// SMA for the same period
    line: L,
    /// Distance the Standard Deviation must be for the lower and upper bands.
    distance: Num,
    /// Lower bound for the Bollinger Bands.
    lower: Num,
    /// Upper bound for the Bollinger Bands.
    upper: Num,
}

impl BollingerBands<SimpleMovingAverage> {
    /// Creates a new Bollinger Band with the supplied period and initial data.
    ///
    /// ### Requirements:
    ///
    /// * Period must be greater than 0.
    /// * Data must have at least `period` elements.
    ///
    /// ## Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the BBands from.
    /// * `distance` - Distance the bands (in standard deviations) from the SMA. default 2.0
    pub fn new(period: usize, data: &[Num], distance: Num) -> Result<Self, TAError> {
        // SMA used for the Bollinger Band.
        let sma = match SimpleMovingAverage::new(period, data) {
            Ok(v) => v,
            Err(error) => return Err(error),
        };

        let distance = distance.abs();
        let stdev = sma.stdev(true);
        let lower = sma.value() - (stdev * distance);
        let upper = sma.value() + (stdev * distance);

        Ok(Self {
            period,
            line: sma,
            distance,
            lower,
            upper,
        })
    }
}

impl<L> BollingerBands<L>
where
    L: Value + Period + Stats,
{
    /// Creates Bollinger Bands using an alternative line, such as an EMA.
    ///
    /// # Arguments
    ///
    /// * `line` - `Line` to use as the middle value.
    /// * `distance` - Distance the bands (in standard deviations) from the SMA. default 2.0
    pub fn with_line(line: L, distance: Num) -> Result<BollingerBands<L>, TAError> {
        let distance = distance.abs();
        let stdev = line.stdev(true);
        let lower = line.value() - (stdev * distance);
        let upper = line.value() + (stdev * distance);

        Ok(Self {
            period: line.period(),
            line,
            distance,
            lower,
            upper,
        })
    }

    /// Distance the standard deviation must be for the lower and upper bands.
    pub fn distance(&self) -> Num {
        self.distance
    }

    /// Lower bound for the Bollinger Band.
    pub fn lower(&self) -> Num {
        self.lower
    }

    /// Upper bound for the Bollinger Band.
    pub fn upper(&self) -> Num {
        self.upper
    }
}

impl<L> Period for BollingerBands<L>
where
    L: Value + Period + Stats,
{
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl<L> Value for BollingerBands<L>
where
    L: Value + Period + Stats,
{
    /// Current and most recent SMA value calculated.
    fn value(&self) -> Num {
        self.line.value()
    }
}

impl<L> Next<Num> for BollingerBands<L>
where
    L: Value + Period + Stats + Next<Num>,
{
    /// Lower, Signal, Upper,
    type Output = (Num, <L as Next<Num>>::Output, Num);

    /// Supply an additional value to recalculate a new Bollinger Band, this returns the current
    /// SMA. Obtain the Upper and Lower bounds with `.upper()` and `.lower()`.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        // Progress the SMA by a value.
        let value = self.line.next(value);

        let stdev = self.line.stdev(true);
        self.lower = self.value() - (stdev * self.distance());
        self.upper = self.value() + (stdev * self.distance());
        (self.lower, value, self.upper)
    }
}
