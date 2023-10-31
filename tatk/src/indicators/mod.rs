//! Indicators generated from samples used for signals.
mod average_true_range;
mod bollinger_bands;
mod cross;
mod double_exponential_moving_average;
mod exponential_moving_average;
mod linear_regression;
mod mcginley_dynamic;
mod moving_average_convergence_divergence;
mod on_balance_volume;
mod rate_of_change;
mod relative_strength_index;
mod simple_moving_average;
mod standard_deviation;
mod true_range;
mod variance;

pub use average_true_range::AverageTrueRange;
pub use bollinger_bands::BollingerBands;
pub use cross::Cross;
pub use double_exponential_moving_average::DoubleExponentialMovingAverage;
pub use exponential_moving_average::ExponentialMovingAverage;
pub use linear_regression::LinearRegression;
pub use mcginley_dynamic::McGinleyDynamic;
pub use moving_average_convergence_divergence::MovingAverageConvergenceDivergence;
pub use on_balance_volume::OnBalanceVolume;
pub use rate_of_change::RateOfChange;
pub use relative_strength_index::RelativeStrengthIndex;
pub use simple_moving_average::SimpleMovingAverage;
pub use standard_deviation::StandardDeviation;
pub use true_range::TrueRange;
pub use variance::Variance;