//! Cross, used to check if lines cross.
//!
//! Death Cross: `short_line` (reactive) crosses below `long_line` (historic).
//!
//! Golden Cross: `short_line` (reactive) crosses above `long_line` (historic).
use crate::traits::{Next, Value};
use crate::Num;

/// Cross, used to check if lines cross.
///
/// Death Cross: `short_line` (reactive) crosses below `long_line` (historic).
///
/// Golden Cross: `short_line` (reactive) crosses above `long_line` (historic).
#[derive(Debug)]
pub struct Cross<L>
where
    L: Value,
{
    // Shorter line (shorter period)
    short_line: L,
    // Longer line (longer period)
    long_line: L,
    // Holds if the lines crossed or not.
    crossed: bool,
}

impl<L> Cross<L>
where
    L: Value + Next<Num>,
{
    /// Creates a new Cross with the supplied two lines.
    ///
    /// Death Cross: `short_line` (reactive) crosses below `long_line` (historic).
    ///
    /// Golden Cross: `short_line` (reactive) crosses above `long_line` (historic).
    ///
    /// ## Arguments
    ///
    /// * `short_line` - Shorter or more reactive line.
    /// * `long_line` - Longer or more historic line.
    pub fn new(short_line: L, long_line: L) -> Self {
        Self {
            short_line,
            long_line,
            crossed: false,
        }
    }

    /// Checks if the `short_line` and `long_line` crossed.
    pub fn crossed(&self) -> bool {
        self.crossed
    }

    /// True if the lines have crossed and the `short_line` is above the `long_line`.
    pub fn is_golden(&self) -> bool {
        self.crossed() && self.short_line.value() > self.long_line.value()
    }

    /// True if the lines have crossed and the `short_line` is below the `long_line`.
    pub fn is_death(&self) -> bool {
        self.crossed() && self.short_line.value() < self.long_line.value()
    }

    /// Supply an additional value to recalculate a cross.
    ///
    /// # Arguments
    ///
    /// * `value` - New value to add to period.
    pub fn next(&mut self, value: Num) -> bool {
        let was_below: bool = self.short_line.value() < self.long_line.value();

        // Progress both lines.
        self.short_line.next(value);
        self.long_line.next(value);

        self.crossed = was_below != (self.short_line.value() < self.long_line.value());
        self.crossed()
    }
}
