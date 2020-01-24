use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum IntervalError {
    InvalidInterval,
}

impl fmt::Display for IntervalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid interval!")
    }
}

impl error::Error for IntervalError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
