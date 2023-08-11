//! Errors that can occur while processing data.
use std::fmt;

/// Errors that can occur within the library.
#[derive(Debug)]
pub enum TAError {
    /// Period or capacity provided is not valid, normally due to being 0 or less than an arrays
    /// length.
    InvalidPeriod,
    /// Array provided is not valid, normally due to be less than the period or capacity.
    InvalidArray,
    /// Indexes provided are not valid.
    InvalidIndex(usize, usize),
    /// Line length is not valid.
    InvalidLine(String),
}

impl fmt::Display for TAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // TAError::InvalidPeriod(value) => write!(f, "bad status: {}", value),
            TAError::InvalidPeriod => f.write_str("period cannot be 0"),
            TAError::InvalidArray => f.write_str("array length cannot be less than the period"),
            TAError::InvalidIndex(start_idx, end_idx) => {
                write!(
                    f,
                    "invalid indexes, start: {} and end: {}",
                    start_idx, end_idx
                )
            }
            TAError::InvalidLine(line) => {
                write!(f, "invalid line, {} is too small", line)
            }
        }
    }
}
