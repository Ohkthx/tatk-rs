//! Technical Analysus Tools written in Rust.
//!
//! This crate is used to an analyze data from samples using common indicators to generate signals.
pub mod buffer;
pub mod error;
pub mod indicators;

#[cfg(not(feature = "f32"))]
pub(crate) type Num = f64;
#[cfg(feature = "f32")]
pub(crate) type Num = f32;

#[cfg(feature = "test-data")]
pub mod test_data;
