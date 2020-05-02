use std::error;
use std::fmt;

/// An error caused while creating an interval.
#[derive(Debug, Clone)]
pub enum IntervalError {
    /// The interval is invalid.
    InvalidInterval,
}

impl fmt::Display for IntervalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid interval!")
    }
}

impl error::Error for IntervalError {}
