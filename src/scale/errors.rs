use crate::interval::IntervalError;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ScaleError {
    InvalidInterval,
    ModeFromRegexError,
}

impl fmt::Display for ScaleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScaleError::InvalidInterval => {
                write!(f, "Can't determine the intervals for the scale scale!")
            }
            ScaleError::ModeFromRegexError => write!(f, "Can't determine the mode!"),
        }
    }
}

impl error::Error for ScaleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<regex::Error> for ScaleError {
    fn from(e: regex::Error) -> Self {
        match e {
            _ => ScaleError::ModeFromRegexError,
        }
    }
}

impl From<IntervalError> for ScaleError {
    fn from(e: IntervalError) -> Self {
        match e {
            _ => ScaleError::InvalidInterval,
        }
    }
}
