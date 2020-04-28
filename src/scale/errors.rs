use crate::interval::IntervalError;
use crate::note::NoteError;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ScaleError {
    InvalidInterval,
    ModeFromRegex,
    InvalidRegex,
}

impl fmt::Display for ScaleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScaleError::InvalidInterval => {
                write!(f, "Can't determine the intervals for the scale scale!")
            }
            ScaleError::ModeFromRegex => write!(f, "Can't determine the mode!"),
            ScaleError::InvalidRegex => write!(f, "Invalid scale regex!"),
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
            _ => ScaleError::ModeFromRegex,
        }
    }
}

impl From<NoteError> for ScaleError {
    fn from(e: NoteError) -> Self {
        match e {
            _ => ScaleError::InvalidRegex,
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
