use crate::interval::IntervalError;
use crate::note::NoteError;
use crate::scale::{Mode, ScaleType};
use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaleError {
    InvalidInterval,
    ModeFromRegex,
    InvalidRegex,
    IncompatibleMode { scale_type: ScaleType, mode: Mode },
}

impl fmt::Display for ScaleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScaleError::InvalidInterval => {
                write!(f, "Can't determine the intervals for the scale!")
            }
            ScaleError::ModeFromRegex => write!(f, "Can't determine the mode!"),
            ScaleError::InvalidRegex => write!(f, "Invalid scale regex!"),
            ScaleError::IncompatibleMode { scale_type, mode } => write!(
                f,
                "Mode {} belongs to {}, not {}",
                mode.canonical_name(),
                mode.scale_type(),
                scale_type
            ),
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
