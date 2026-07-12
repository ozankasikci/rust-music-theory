use crate::note::NoteError;
use std::error;
use std::fmt;

/// An error while parsing a chord.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChordError {
    InvalidRegex,
    EmptySymbol,
    InvalidRoot { position: usize },
    UnexpectedToken { position: usize, token: String },
    InvalidModifier(String),
    InvalidModifierAt { position: usize, modifier: String },
    ConflictingModifiers(String),
    ConflictingModifiersAt { position: usize, message: String },
    InvalidSlashBass { position: usize },
    InvalidDegree(u8),
    InvalidAlteration(i8),
    InvalidSpecification(String),
    UnsupportedConstruction { position: usize, message: String },
    UnknownIntervalPattern(Vec<u8>),
    UnsupportedChord(String),
    InvalidInversion(u8),
}

impl fmt::Display for ChordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChordError::InvalidRegex => write!(f, "Invalid Regex!"),
            ChordError::EmptySymbol => write!(f, "Chord symbol is empty"),
            ChordError::InvalidRoot { position } => {
                write!(f, "Invalid chord root at byte {}", position)
            }
            ChordError::UnexpectedToken { position, token } => {
                write!(f, "Unexpected token {:?} at byte {}", token, position)
            }
            ChordError::InvalidModifier(modifier) => {
                write!(f, "Invalid chord modifier: {}", modifier)
            }
            ChordError::InvalidModifierAt { position, modifier } => {
                write!(
                    f,
                    "Invalid chord modifier {} at byte {}",
                    modifier, position
                )
            }
            ChordError::ConflictingModifiers(message) => {
                write!(f, "Conflicting chord modifiers: {}", message)
            }
            ChordError::ConflictingModifiersAt { position, message } => {
                write!(
                    f,
                    "Conflicting chord modifiers at byte {}: {}",
                    position, message
                )
            }
            ChordError::InvalidSlashBass { position } => {
                write!(f, "Invalid slash bass at byte {}", position)
            }
            ChordError::InvalidDegree(degree) => {
                write!(f, "Invalid chord degree: {}", degree)
            }
            ChordError::InvalidAlteration(alteration) => {
                write!(f, "Invalid chord alteration: {}", alteration)
            }
            ChordError::InvalidSpecification(message) => {
                write!(f, "Invalid chord specification: {}", message)
            }
            ChordError::UnsupportedConstruction { position, message } => {
                write!(
                    f,
                    "Unsupported chord construction at byte {}: {}",
                    position, message
                )
            }
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
    fn from(_: NoteError) -> Self {
        ChordError::InvalidRegex
    }
}

impl From<regex::Error> for ChordError {
    fn from(_: regex::Error) -> Self {
        ChordError::InvalidRegex
    }
}
