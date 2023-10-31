//! Errors that can occur while processing data.
use std::fmt;

/// Errors that can occur within the library.
#[derive(Debug)]
pub enum TAError {
    /// Invalid size for capacity, length, period, etc.
    InvalidSize(String),
    /// Data provided is invalid.
    InvalidData(String),
    /// Indexes provided are not valid.
    InvalidIndex(usize, usize),
    /// Line length is not valid.
    InvalidLine(String),
}

impl fmt::Display for TAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TAError::InvalidSize(text) => {
                write!(f, "invalid size, {}", text)
            }
            TAError::InvalidData(text) => {
                write!(f, "invalid data, {}", text)
            }
            TAError::InvalidIndex(start_idx, end_idx) => {
                write!(
                    f,
                    "invalid indexes, start: {} and end: {}",
                    start_idx, end_idx
                )
            }
            TAError::InvalidLine(text) => {
                write!(f, "invalid line, {}", text)
            }
        }
    }
}
