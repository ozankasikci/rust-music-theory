use crate::note::errors::NoteError;
use crate::interval::Interval;
use regex::{Match, Regex};
use std::error;
use std::fmt;
use strum_macros::EnumIter;

const REGEX_PITCH: &str = "^[ABCDEFGabcdefg]";
const REGEX_PITCH_ACCIDENTAL: &str = "^[ABCDEFGabcdefg][b♯#]";

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum PitchClass {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl PitchClass {
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

    pub fn from_str(str: &str) -> Self {
        use PitchClass::*;
        match str {
            "C" | "c" => C,
            "C#" | "Cs" | "c#" | "cs" => Cs,
            "D" | "d" => D,
            "D#" | "Ds" | "d#" | "ds" => Ds,
            "E" | "e" => E,
            "F" | "f" => F,
            "F#" | "Fs" | "f#" | "fs" => Fs,
            "G" | "g" => G,
            "G#" | "Gs" | "g#" | "gs" => Gs,
            "A" | "a" => A,
            "A#" | "As" | "a#" | "as" => As,
            "B" | "b" => B,
            _ => C,
        }
    }

    pub fn from_interval(pitch: &Self, interval: &Interval) -> Self {
        let current_pitch = *pitch as u8;
        let new_pitch = current_pitch + interval.semitone_count;

        Self::from_u8(new_pitch)
    }

    pub fn from_regex(string: &str) -> Result<(Self, Match), NoteError> {
        let r_pitch = Regex::new(REGEX_PITCH)?;
        let r_pitch_accidental = Regex::new(REGEX_PITCH_ACCIDENTAL)?;

        let pitch = r_pitch_accidental
            .find(&string)
            .or_else(|| r_pitch.find(&string))
            .ok_or(NoteError::InvalidPitch)?;

        Ok((Self::from_str(&string[pitch.start()..pitch.end()]), pitch))
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
