//! Linear Regression (LineReg), creates a best fit line.
//!
//! Creates a line that best fits a period of data using the least squares approach.
use crate::traits::{AsValue, Next, Period, Stats, Value};
use crate::{Buffer, Num, TAError};

/// Linear Regression (LineReg), creates a best fit line.
///
/// Creates a line that best fits a period of data using the least squares approach.
#[derive(Debug)]
pub struct LineReg {
    /// Size of the period (window) in which data is looked at.
    period: usize,
    /// LineReg's current value.
    value: Num,
    /// Stasis values.
    values: Buffer,
    /// Holds all of the current period's values.
    buffer: Buffer,
    /// Sum of the X.
    sum_x: Num,
    /// Sum of the X, squared.
    sum_x_sq: Num,
    /// Intercept of the line.
    intercept: Num,
    /// Slope of the line.
    slope: Num,
}

impl LineReg {
    /// Creates a new LineReg with the supplied period and initial data.
    ///
    /// Required: The initial data must be at least of equal size/length or greater than the period.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `data` - Array of values to create the LineReg from.
    pub fn new(period: usize, data: &[Num]) -> Result<Self, TAError> {
        // Make sure we have enough data.
        if data.len() < period {
            return Err(TAError::InvalidData(String::from(
                "not enough data for period provided",
            )));
        } else if period < 2 {
            return Err(TAError::InvalidSize(String::from(
                "period must be 2 or more",
            )));
        }

        // Constants
        let sum_x: Num = (period * (period + 1)) as Num * 0.5;
        let sum_x_sq: Num = (period * (period + 1) * (2 * period + 1)) as Num / 6.0;

        // Build the buffer containing the `period` of y values.
        let mut values: Buffer = match Buffer::from_array(period, &data[..period]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // Calculate the first value to seed the buffer.
        let (mut intercept, mut slope) = Self::calculate(period, &values, sum_x, sum_x_sq);
        let mut value: Num = intercept + (slope * period as Num);

        // Build the buffer to hold old best fit values.
        let mut buffer: Buffer = match Buffer::from_array(period, &[value]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        // Calculate the remaining best fit values.
        for y in data[period..].iter() {
            values.shift(*y);
            (intercept, slope) = Self::calculate(period, &values, sum_x, sum_x_sq);

            // Calculate new value.
            value = intercept + (slope * period as Num);
            buffer.shift(value);
        }

        Ok(Self {
            period,
            value,
            values,
            buffer,
            sum_x,
            sum_x_sq,
            intercept,
            slope,
        })
    }

    /// Calculates the intercept and slope for the line.
    ///
    /// # Arguments
    ///
    /// * `period` - Size of the period / window used.
    /// * `values` - Last `period` of values to fit a line to.
    /// * `sum_x` - Constant used, represents the sum of the time portion.
    /// * `sum_x_sq` - Constant used, represents the square of the sum of the time portion.
    fn calculate(period: usize, values: &Buffer, sum_x: Num, sum_x_sq: Num) -> (Num, Num) {
        let sum_y: Num = values.sum();
        let sum_xy: Num = (1..=period)
            .zip(values.queue().iter().take(period))
            .map(|(x, y)| x as Num * y)
            .sum();

        // Calculate intercept and slope.
        let period_as: Num = period as Num;
        let slope = (period_as * sum_xy - sum_x * sum_y) / (period_as * sum_x_sq - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / period_as;

        return (intercept, slope);
    }

    /// Predicted value of the dependent variable when all independent variables are set to zero.
    pub fn intercept(&self) -> Num {
        self.intercept
    }

    /// Coefficient associated with the independent variable.
    pub fn slope(&self) -> Num {
        self.slope
    }

    /// Percentage of variance in the dependent variable that can be explained by the independent variable.
    pub fn r_sq(&self) -> Num {
        let mean_y: Num = self.values.mean();
        let predicted_y: Vec<Num> = (1..=self.period())
            .map(|i| self.intercept() + self.slope() * i as Num)
            .collect();

        // Sum of Squares Total (sst) and Sum of Squares Residual (ssr).
        let mut sst: Num = 0.0;
        let mut ssr: Num = 0.0;
        for (i, y) in self.values.queue().iter().enumerate() {
            sst += (y - mean_y).powi(2);
            ssr += (y - predicted_y[i]).powi(2);
        }

        1.0 - (ssr / sst)
    }

    /// Gets the standard deviation for the current line.
    /// - ±1 stdev, 68%
    /// - ±2 stdev, 95%
    /// - ±3 stdev, 99.7%
    pub fn line_stdev(&self) -> Num {
        self.values.stdev(true)
    }

    /// Predicts (forecasts) a future value `distance` away from the current.
    ///
    /// # Arguments
    ///
    /// * `distance` - How far in the future to predict.
    pub fn forecast(&self, distance: usize) -> Num {
        self.intercept() + (self.slope() * (self.period() + distance) as Num)
    }
}

impl Period for LineReg {
    /// Period (window) for the samples.
    fn period(&self) -> usize {
        self.period
    }
}

impl Value for LineReg {
    /// Current and most recent value calculated.
    fn value(&self) -> Num {
        self.value
    }
}

impl Next<Num> for LineReg {
    /// Next Value for the LineReg.
    type Output = Num;

    /// Supply an additional value to recalculate a new LineReg.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: Num) -> Self::Output {
        // Rotate the buffer.
        self.values.shift(value);

        // Get the intercept and slope.
        (self.intercept, self.slope) =
            Self::calculate(self.period(), &self.values, self.sum_x, self.sum_x_sq);

        // Calculate the current value.
        self.value = self.intercept() + (self.slope() * self.period() as Num);
        self.buffer.shift(self.value());

        self.value
    }
}

impl<T> Next<T> for LineReg
where
    T: AsValue,
{
    /// Next Value for the LineReg.
    type Output = Num;

    /// Supply an additional value to recalculate a new LineReg.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    fn next(&mut self, value: T) -> Self::Output {
        self.next(value.as_value())
    }
}

impl Stats for LineReg {
    /// Obtains the total sum of the buffer for LineReg.
    fn sum(&self) -> Num {
        self.buffer.sum()
    }

    /// Mean for the period of the LineReg.
    fn mean(&self) -> Num {
        self.buffer.mean()
    }

    /// Current variance for the period.
    ///
    /// # Arguments
    ///
    /// * `is_sample` - If the data is a Sample or Population, default should be True.
    fn variance(&self, is_sample: bool) -> Num {
        self.buffer.variance(is_sample)
    }

    /// Current standard deviation for the period.
    ///
    /// # Arguments
    ///
    /// * `is_sample` - If the data is a Sample or Population, default should be True.
    fn stdev(&self, is_sample: bool) -> Num {
        self.buffer.stdev(is_sample)
    }
}
