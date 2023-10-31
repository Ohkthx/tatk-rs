//! Technical Analysis Tools written in Rust.
//!
//! This crate is used to an analyze data from samples using common indicators to generate signals.

/// Represents the internally used numeric type for the crate.
#[cfg(not(feature = "f32"))]
pub type Num = f64;

/// Represents the internally used numeric type for the crate.
#[cfg(feature = "f32")]
pub type Num = f32;

#[cfg(feature = "test-data")]
pub mod test_data;

pub(crate) mod error;
pub use error::TAError;

pub(crate) mod buffer;
pub use buffer::Buffer;

pub mod indicators;
pub mod macros;
pub mod traits;
