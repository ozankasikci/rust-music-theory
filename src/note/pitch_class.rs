use crate::interval::Interval;
use crate::note::errors::NoteError;
use regex::{Match, Regex};
use std::fmt;
use std::str::FromStr;
use strum_macros::EnumIter;

const REGEX_PITCH: &str = "^[ABCDEFGabcdefg][b♭♯#s]?";

/// A pitch class (A, B, C#, etc).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum PitchClass {
    /// C
    C,
    /// C#
    Cs,
    /// D
    D,
    /// D#
    Ds,
    /// E
    E,
    /// F
    F,
    /// F#
    Fs,
    /// G
    G,
    /// G#
    Gs,
    /// A
    A,
    /// A#
    As,
    /// B
    B,
}

impl PitchClass {
    /// Create a pitch class from an integer, where 0 is C and everything climbs up from there,
    /// looping back around once it reaches 12.
    pub fn from_u8(val: u8) -> Self {
        use PitchClass::*;
        match val {
            0 => C,
            1 => Cs,
            2 => D,
            3 => Ds,
            4 => E,
            5 => F,
            6 => Fs,
            7 => G,
            8 => Gs,
            9 => A,
            10 => As,
            11 => B,
            _ => Self::from_u8(val % 12),
        }
    }

    /// Attempt to parse this note from a string. It should contain the name of the note in either
    /// uppercase or lowercase, followed by `#`, `s`, `S`, or `♯` for sharps and `b` or `♭` for
    /// flats.
    pub fn from_str(string: &str) -> Option<Self> {
        use PitchClass::*;
        let mut characters = string.chars();

        let first_char = characters.next()?;
        let mut pitch = match first_char {
            'C' | 'c' => C,
            'D' | 'd' => D,
            'E' | 'e' => E,
            'F' | 'f' => F,
            'G' | 'g' => G,
            'A' | 'a' => A,
            'B' | 'b' => B,
            _ => return None,
        };

        if let Some(second_char) = characters.next() {
            match second_char {
                '#' | 's' | 'S' | '♯' => {
                    let interval = Interval::from_semitone(1);
                    if let Ok(interval) = interval {
                        pitch = Self::from_interval(pitch, interval);
                    }
                }
                'b' | '♭' => {
                    let interval = Interval::from_semitone(11);
                    if let Ok(interval) = interval {
                        pitch = Self::from_interval(pitch, interval);
                    }
                }
                _ => return None,
            }
        }

        if characters.next().is_some() {
            return None;
        }

        Some(pitch)
    }

    /// Create a note by moving up the given note by an interval.
    pub fn from_interval(pitch: Self, interval: Interval) -> Self {
        let current_pitch = pitch.into_u8();
        let new_pitch = current_pitch + interval.semitone_count;

        Self::from_u8(new_pitch)
    }

    /// Parse the note using a regex, with the same algorithm as described in `from_str`.
    pub fn from_regex(string: &str) -> Result<(Self, Match), NoteError> {
        let r_pitch = Regex::new(REGEX_PITCH)?;

        let pitch_match = r_pitch.find(&string).ok_or(NoteError::InvalidPitch)?;

        let pitch_class = Self::from_str(&string[pitch_match.start()..pitch_match.end()])
            .ok_or(NoteError::InvalidPitch)?;

        Ok((pitch_class, pitch_match))
    }

    /// Convert the pitch class into its corresponding integer, where 0 is C and 11 is B.
    pub fn into_u8(self) -> u8 {
        use PitchClass::*;
        match self {
            C => 0,
            Cs => 1,
            D => 2,
            Ds => 3,
            E => 4,
            F => 5,
            Fs => 6,
            G => 7,
            Gs => 8,
            A => 9,
            As => 10,
            B => 11,
        }
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use PitchClass::*;
        match *self {
            C => write!(fmt, "C"),
            Cs => write!(fmt, "C#"),
            D => write!(fmt, "D"),
            Ds => write!(fmt, "D#"),
            E => write!(fmt, "E"),
            F => write!(fmt, "F"),
            Fs => write!(fmt, "F#"),
            G => write!(fmt, "G"),
            Gs => write!(fmt, "G#"),
            A => write!(fmt, "A"),
            As => write!(fmt, "A#"),
            B => write!(fmt, "B"),
        }
    }
}

impl FromStr for PitchClass {
    type Err = NoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or(NoteError::InvalidPitch)
    }
}
