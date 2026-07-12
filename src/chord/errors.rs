use crate::note::NoteError;
use std::error;
use std::fmt;

/// An error while parsing a chord.
#[derive(Debug, Clone)]
pub enum ChordError {
    InvalidRegex,
    UnknownIntervalPattern(Vec<u8>),
    UnsupportedChord(String),
    InvalidInversion(u8),
}

impl fmt::Display for ChordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChordError::InvalidRegex => write!(f, "Invalid Regex!"),
            ChordError::UnknownIntervalPattern(intervals) => {
                write!(f, "Unknown chord interval pattern: {:?}", intervals)
            }
            ChordError::UnsupportedChord(chord) => write!(f, "Unsupported chord: {}", chord),
            ChordError::InvalidInversion(inversion) => {
                write!(f, "Invalid chord inversion: {}", inversion)
            }
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
