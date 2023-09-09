//! Traits for both indicators and user-defined data types.
//!
use crate::Num;

/// Indicator: Statistics for the indicator.
pub trait Stats {
    /// Sum for the period of the line.
    fn sum(&self) -> Num;
    /// Mean for the period of the line.
    fn mean(&self) -> Num;
    /// Variance for the period of the line.
    fn variance(&self, is_sample: bool) -> Num;
    /// Standard deviation for the period of the line.
    fn stdev(&self, is_sample: bool) -> Num;
}

/// Indicator: Window or capacity for an indicator.
pub trait Period {
    /// Window or capacity for an indicator.
    fn period(&self) -> usize;
}

/// Indicator: Current alue for an indicator.
pub trait Value {
    /// Current alue for an indicator.
    fn value(&self) -> Num;
}

/// Indicator: Add new data to an indicator.
pub trait Next<T> {
    /// Output from the function.
    type Output;

    /// Supply additional data to calculate the next value for the indicator.
    ///
    /// # Return depends on indicator, all values are in the form of `Num`.
    ///
    /// * Default: `value`
    /// * MACD:    (`signal`, `short`, `long`)
    /// * BBands:  (`lower`, `value`, `upper`)
    ///
    /// # Arguments
    ///
    /// * `value` - New data to add to the indicator.
    fn next(&mut self, value: T) -> Self::Output;
}

/// User Defined: Specialized value to pass to indicators. Values such as HL, HLC, OHLC. etc
pub trait AsValue {
    /// User defined value to pass to indicators. Values such as HL, HLC, OHLC. etc
    fn as_value(&self) -> Num;
}

/// User Defined: Opening value for the data type.
pub trait Open {
    /// Opening value for the data type.
    fn open(&self) -> Num;
}

/// User Defined: Closing value for the data type.
pub trait Close {
    /// Closing value for the data type.
    fn close(&self) -> Num;
}

/// User Defined: Highest value for the data type.
pub trait High {
    /// Highest value for the data type.
    fn high(&self) -> Num;
}

/// User Defined: Lowest value for the data type.
pub trait Low {
    /// Lowest value for the data type.
    fn low(&self) -> Num;
}

/// User Defined: Total volume for the data type.
pub trait Volume {
    /// Total volume for the data type.
    fn volume(&self) -> Num;
}

/// Average between High and Low traits.
pub trait HL2: High + Low {
    /// Average between High and Low traits.
    fn hl2(&self) -> Num {
        (self.high() + self.low()) / 2.0 as Num
    }
}

/// Average between High, Low, and Close traits.
pub trait HLC3: High + Low + Close {
    /// Average between High, Low, and Close traits.
    fn hlc3(&self) -> Num {
        (self.high() + self.low() + self.close()) / 3.0 as Num
    }
}

/// Average between Open, High, Low, and Close traits.
pub trait OHLC4: Open + High + Low + Close {
    /// Average between Open, High, Low, and Close traits.
    fn ohlc4(&self) -> Num {
        (self.open() + self.high() + self.low() + self.close()) / 4.0 as Num
    }
}
