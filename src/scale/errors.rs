use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ScaleIntervalError;

impl fmt::Display for ScaleIntervalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't determine the intervals for the scale scale!")
    }
}

impl error::Error for ScaleIntervalError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
