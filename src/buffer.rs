//! Buffer with maximum capacity that rotates itself.
//!
//! Removes oldest values when a newer value is added. The oldest value is returned.
use crate::error::TAError;
use crate::Num;

/// Buffer with maximum capacity that rotates itself.
#[derive(Debug)]
pub struct Buffer {
    /// Maximum capacity of the buffer.
    capacity: usize,
    /// Data the buffer current holds.
    data: Vec<Num>,
    /// 0 if the buffer is full, >0 otherwise.
    full: usize,
    /// Sum of the buffer
    sum: Num,
}

impl Buffer {
    /// Creates a new buffer from the data provided. If the data's length is less than the capacity
    /// provided, the oldest values will be padded with the default value of T and and `is_ready()`
    /// will be `false`. If the data's length is >= capacity, it takes the last values of data and
    /// `is_ready()` will be `true`.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Total size of the buffer, must be > 0.
    /// * `data` - Array of data to fill with. Newest -> Oldest.
    pub fn from_array(capacity: usize, data: &[Num]) -> Result<Self, TAError> {
        if capacity == 0 {
            return Err(TAError::InvalidPeriod);
        }

        let mut vec: Vec<Num>;
        let full: usize;

        if data.len() >= capacity {
            // Full vector, mark as full (0).
            vec = data[(data.len() - capacity)..].to_vec();
            full = 0;
        } else {
            // Pad out the vector and mark it as not full.
            vec = vec![Default::default(); capacity - data.len()];
            vec.extend_from_slice(data);
            full = capacity - data.len();
        }

        let sum = vec.iter().sum();

        Ok(Self {
            capacity,
            data: vec,
            full,
            sum,
        })
    }

    /// Maximum capacity the buffer can hold.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Checks if the buffer is ready meaning there is full data and no remaining padding.
    pub fn is_ready(&self) -> bool {
        self.full == 0
    }

    /// Gets the oldest value in the buffer, this is the next value that will be removed.
    pub fn oldest(&self) -> Num {
        self.data.first().unwrap().clone()
    }

    /// Gets the newest value in the buffer, this value will current live the longest in the
    /// buffer.
    pub fn newest(&self) -> Num {
        self.data.last().unwrap().clone()
    }

    /// Returns the data held by the buffer from Oldest -> Newest. Index 0 being the oldest and
    /// next value to be removed. Index (len-1) being the newest data.
    pub fn queue(&self) -> &[Num] {
        &self.data[..]
    }

    /// Adds a new (newest) value to the buffer. Oldest value is removed and returned.
    ///
    /// # Arguments
    ///
    /// * `value` - New (newest) value to add to the buffer.
    pub fn shift(&mut self, value: Num) -> Num {
        if self.full > 0 {
            self.full = self.full - 1;
        }
        let oldest = self.data.remove(0usize);
        self.sum = self.sum - oldest + value;
        self.data.push(value);
        oldest
    }

    /// Obtain the sum of the buffer.
    pub fn sum(&self) -> Num {
        self.sum
    }

    /// Calculates the mean of the buffer.
    pub fn mean(&self) -> Num {
        self.sum() / self.data.len() as Num
    }

    /// Calculates the variance of the buffer.
    ///
    /// # Arguments
    ///
    /// * `is_sample` - If the data is a Sample or Population, default should be True.
    pub fn variance(&self, is_sample: bool) -> Num {
        let mean = self.mean();
        let divisor: Num = if is_sample {
            (self.data.len() - 1) as Num
        } else {
            self.data.len() as Num
        };

        self.data
            .iter()
            .map(|x| Num::powi(x - mean, 2))
            .sum::<Num>()
            / divisor
    }

    /// Calculates the standard deviation of the buffer.
    ///
    /// # Arguments
    ///
    /// * `is_sample` - If the data is a Sample or Population, default should be True.
    pub fn stdev(&self, is_sample: bool) -> Num {
        self.variance(is_sample).sqrt()
    }
}
