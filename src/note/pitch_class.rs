use crate::interval::Interval;
use crate::note::errors::NoteError;
use regex::{Match, Regex};
use std::fmt;
use strum_macros::EnumIter;

const REGEX_PITCH: &str = "^[ABCDEFGabcdefg]";
const REGEX_PITCH_ACCIDENTAL: &str = "^[ABCDEFGabcdefg][b♭♯#s]";

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

    pub fn from_str(string: &str) -> Option<Self> {
        use PitchClass::*;
        let characters: Vec<char> = string.chars().collect();

        let mut pitch = match characters[0] {
            'C' | 'c' => C,
            'D' | 'd' => D,
            'E' | 'e' => E,
            'F' | 'f' => F,
            'G' | 'g' => G,
            'A' | 'a' => A,
            'B' | 'b' => B,
            _ => return None,
        };

        if characters.len() > 1 {
            let second_char = characters[1];
            match second_char {
                '#' | 's' | 'S' | '♯' => {
                    let interval = Interval::from_semitone(1);
                    if interval.is_ok() {
                        pitch = Self::from_interval(&pitch, &interval.unwrap());
                    }
                }
                'b' | '♭' => {
                    let interval = Interval::from_semitone(11);
                    if interval.is_ok() {
                        pitch = Self::from_interval(&pitch, &interval.unwrap());
                    }
                }
                _ => return None,
            }
        }

        Some(pitch)
    }

    pub fn from_interval(pitch: &Self, interval: &Interval) -> Self {
        let current_pitch = *pitch as u8;
        let new_pitch = current_pitch + interval.semitone_count;

        Self::from_u8(new_pitch)
    }

    pub fn from_regex(string: &str) -> Result<(Self, Match), NoteError> {
        let r_pitch = Regex::new(REGEX_PITCH)?;
        let r_pitch_accidental = Regex::new(REGEX_PITCH_ACCIDENTAL)?;

        let pitch_match = r_pitch_accidental
            .find(&string)
            .or_else(|| r_pitch.find(&string))
            .ok_or(NoteError::InvalidPitch)?;

        let pitch_class = Self::from_str(&string[pitch_match.start()..pitch_match.end()])
            .ok_or(NoteError::InvalidPitch)?;

        Ok((pitch_class, pitch_match))
    }

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
