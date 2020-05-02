use std::error;
use std::fmt;

/// An error caused when parsing a note.
#[derive(Debug, Clone)]
pub enum NoteError {
    /// The note's pitch was invalid.
    InvalidPitch,
}

impl fmt::Display for NoteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Pitch Class!")
    }
}

impl error::Error for NoteError {}

impl From<regex::Error> for NoteError {
    fn from(e: regex::Error) -> Self {
        match e {
            _ => NoteError::InvalidPitch,
        }
    }
}
