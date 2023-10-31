//! Shorthand macros used to create indicators.

/// Initialize an Average True Range (ATR) indicator.
///
/// ### Requirements:
///
/// * Period must be greater than 0.
/// * Data must have at least `period + 1` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the ATR from.
#[macro_export]
macro_rules! atr {
    ($period:expr, $data:expr) => {
        $crate::indicators::AverageTrueRange::new($period, $data)
    };
}

/// Initialize a Bollinger Bands (BB / BBands) indicator.
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
#[macro_export]
macro_rules! bb {
    ($period:expr, $data:expr, $distance:expr) => {
        $crate::indicators::BollingerBands::new($period, $data, $distance)
    };
}

/// Initialize a Cross indicator using two lines.
///
/// Death Cross: `short_line` (reactive) crosses below `long_line` (historic).
///
/// Golden Cross: `short_line` (reactive) crosses above `long_line` (historic).
///
/// ## Arguments
///
/// * `short_line` - Shorter or more reactive line.
/// * `long_line` - Longer or more historic line.
#[macro_export]
macro_rules! cross {
    ($short_line:expr, $long_line:expr) => {
        $crate::indicators::Cross::new($short_line, $long_line)
    };
}

/// Initialize a Double Exponential Moving Average (DEMA) indicator.
///
/// ### Requirements:
///
/// * Period must be greater than 0.
/// * Data must have at least `(period * 2) - 1` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the DEMA from.
#[macro_export]
macro_rules! dema {
    ($period:expr, $data:expr) => {
        $crate::indicators::DoubleExponentialMovingAverage::new($period, $data)
    };
}

/// Initialize an Exponential Moving Average (EMA) indicator.
///
/// ### Requirements:
///
/// * Period must be greater than 0.
/// * Data must have at least `period` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the EMA from.
#[macro_export]
macro_rules! ema {
    ($period:expr, $data:expr) => {
        $crate::indicators::ExponentialMovingAverage::new($period, $data)
    };
}

/// Initialize a Linear Regression (LR / LineReg) indicator.
///
/// ### Requirements:
///
/// * Period must be greater than 1.
/// * Data must have at least `period` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the LineReg from.
#[macro_export]
macro_rules! lr {
    ($period:expr, $data:expr) => {
        $crate::indicators::LinearRegression::new($period, $data)
    };
}

/// Initialize a McGinley Dynamic (MDI) indicator.
///
/// ### Requirements:
///
/// * Period must be greater than 1.
/// * Data must have at least `period + 1` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the MD from.
/// * `k` - Constant used to modify selected period. Default: 0.6
#[macro_export]
macro_rules! mdi {
    ($period:expr, $data:expr, $k:expr) => {
        $crate::indicators::McGinleyDynamic::new($period, $data, $k)
    };
}

/// Initialize a Moving Average Convergence and Divergence (MACD) indicator.
///
/// ### Requirements:
///
/// * Short, Signal, and Long must greater than 0.
/// * Short must be smaller than Long.
/// * Data must have at least `signal` elements.
/// * Data must have at least `long` elements.
///
/// ## Arguments
///
/// * `short` - Period of the short EMA.
/// * `long` - Period of the long EMA.
/// * `signal` - Period of the signal EMA.
/// * `data` - Array of values to create the MACD from.
#[macro_export]
macro_rules! macd {
    ($short:expr, $long:expr, $signal:expr, $data:expr) => {
        $crate::indicators::MovingAverageConvergenceDivergence::new($short, $long, $signal, $data)
    };
}

/// Initialize an On-Balance Volume (OBV) indicator.
///
/// ### Requirements:
///
/// * Period must be greater than 1.
/// * Data must have at least `period` elements.
///
/// ## Arguments
///
/// * `period` - History of values to keep.
/// * `data` - Array of values to create the OBV from.
#[macro_export]
macro_rules! obv {
    ($period:expr, $data:expr) => {
        $crate::indicators::OnBalanceVolume::new($period, $data)
    };
}

/// Initialize a Rate of Change (ROC) indicator.
///
/// ### Requirements:
///
/// * Period must be greater than 1.
/// * Data must have at least `period + 1` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the ROC from.
#[macro_export]
macro_rules! roc {
    ($period:expr, $data:expr) => {
        $crate::indicators::RateOfChange::new($period, $data)
    };
}

/// Initialize a Relative Strength Index (RSI) indicator.
///
/// ### Requirements:
///
/// * Period must be greater than 1.
/// * Data must have at least `period + 1` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the RSI from.
#[macro_export]
macro_rules! rsi {
    ($period:expr, $data:expr) => {
        $crate::indicators::RelativeStrengthIndex::new($period, $data)
    };
}

/// Initialize a Simple Moving Average (SMA) indicator.
///
/// ### Requirements:
///
/// * Period must be greater than 0.
/// * Data must have at least `period` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the SMA from.
#[macro_export]
macro_rules! sma {
    ($period:expr, $data:expr) => {
        $crate::indicators::SimpleMovingAverage::new($period, $data)
    };
}

/// Initialize a Standard Deviation (SD / Stdev) for a period of a buffer.
///
/// ### Requirements:
///
/// * Period must be greater than 0.
/// * Data must have at least `period` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the STDEV from.
/// * `is_sample` - If the data is a Sample or Population, default should be True.
#[macro_export]
macro_rules! sd {
    ($period:expr, $data:expr, $is_sample:expr) => {
        $crate::indicators::StandardDeviation::new($period, $data, $is_sample)
    };
}

/// Initialize a True Range (TR) indicator.
///
/// ### Requirements:
///
/// * Period must be greater than 0.
/// * Data must have at least `period + 1` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the TR from.
#[macro_export]
macro_rules! tr {
    ($period:expr, $data:expr) => {
        $crate::indicators::TrueRange::new($period, $data)
    };
}

/// Initialize a Variance (Var) for a period of a buffer.
///
/// ### Requirements:
///
/// * Period must be greater than 0.
/// * Data must have at least `period` elements.
///
/// ## Arguments
///
/// * `period` - Size of the period / window used.
/// * `data` - Array of values to create the Var(X) from.
/// * `is_sample` - If the data is a Sample or Population, default should be True.
#[macro_export]
macro_rules! var {
    ($period:expr, $data:expr, $is_sample:expr) => {
        $crate::indicators::Variance::new($period, $data, $is_sample)
    };
}
