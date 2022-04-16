use crate::note::NoteError;
use std::error;
use std::fmt;

/// An error while parsing a chord.
#[derive(Debug, Clone)]
pub enum ChordError {
    InvalidRegex,
    InvalidUnknownChord,
}

impl fmt::Display for ChordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChordError::InvalidRegex => write!(f, "Invalid Regex!"),
            ChordError::InvalidUnknownChord => write!(f, "Invalid/Unknown Chord!"),
        }
    }
}

impl error::Error for ChordError {}

impl From<NoteError> for ChordError {
    fn from(e: NoteError) -> Self {
        match e {
            _ => ChordError::InvalidRegex,
        }
    }
}

impl From<regex::Error> for ChordError {
    fn from(e: regex::Error) -> Self {
        match e {
            _ => ChordError::InvalidRegex,
        }
    }
}
