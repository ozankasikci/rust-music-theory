use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ScaleError {
    InvalidInterval,
}

impl fmt::Display for ScaleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScaleError::InvalidInterval => write!(f, "Can't determine the intervals for the scale scale!"),
        }
    }
}

impl error::Error for ScaleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

