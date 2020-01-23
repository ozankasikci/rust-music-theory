use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum NoteError {
    InvalidPitch,
}

impl fmt::Display for NoteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Pitch Class!")
    }
}

impl error::Error for NoteError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl From<regex::Error> for NoteError {
    fn from(e: regex::Error) -> Self {
        match e {
            _ => NoteError::InvalidPitch,
        }
    }
}

