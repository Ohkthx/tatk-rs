use crate::Num;

pub trait Line {
    /// Current period of the Line (window being looked at.)
    fn period(&self) -> usize;
    /// Value of the Line.
    fn value(&self) -> Num;
    /// Value of the line after the `value` addition.
    fn next(&mut self, value: Num) -> Num;
}

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
