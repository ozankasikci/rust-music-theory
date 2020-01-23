use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ChordError {
    InvalidRegex,
}

impl fmt::Display for ChordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Regex!")
    }
}

impl error::Error for ChordError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl From<regex::Error> for ChordError {
    fn from(e: regex::Error) -> Self {
        match e {
            _ => ChordError::InvalidRegex,
        }
    }
}

