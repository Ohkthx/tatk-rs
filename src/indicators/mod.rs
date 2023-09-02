//! Indicators generated from samples used for signals.
mod bollinger_bands;
mod cross;
mod double_exponential_moving_average;
mod exponential_moving_average;
mod moving_average_convergence_divergence;
mod relative_strength_index;
mod simple_moving_average;
mod standard_deviation;
mod variance;

pub use bollinger_bands::BBands;
pub use cross::Cross;
pub use double_exponential_moving_average::DEMA;
pub use exponential_moving_average::EMA;
pub use moving_average_convergence_divergence::MACD;
pub use relative_strength_index::RSI;
pub use simple_moving_average::SMA;
pub use standard_deviation::STDEV;
pub use variance::Variance;
