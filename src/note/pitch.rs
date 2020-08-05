use crate::interval::Interval;
use crate::note::errors::NoteError;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::fmt;
use std::str::FromStr;
use strum_macros::EnumIter;

lazy_static! {
    static ref REGEX_PITCH: Regex = Regex::new("^[ABCDEFGabcdefg][b♭♯#s]?").unwrap();
}

/// A note letter without an accidental.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum NoteLetter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

pub fn pitch(letter: NoteLetter, accidental: i8) -> Pitch {
    Pitch { letter, accidental }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pitch {
    pub letter: NoteLetter,
    pub accidental: i8,
}

impl Pitch {
    /// Create a pitch from an integer, where 0 is C and everything climbs up from there,
    /// looping back around once it reaches 12.
    pub fn from_u8(val: u8) -> Self {
        use NoteLetter::*;
        return match val % 12 {
            0 => { pitch(C, 0) }
            1 => { pitch(C, 1) }
            2 => { pitch(D, 0) }
            3 => { pitch(D, 1) }
            4 => { pitch(E, 0) }
            5 => { pitch(F, 0) }
            6 => { pitch(F, 1) }
            7 => { pitch(G, 0) }
            8 => { pitch(G, 1) }
            9 => { pitch(A, 0) }
            10 => { pitch(A, 1) }
            11 => { pitch(B, 0) }
            _ => panic!("impossible")
        };
    }

    /// Convert the pitch into its corresponding integer, where 0 is C and 11 is B.
    pub fn into_u8(self) -> u8 {
        use NoteLetter::*;
        ((match self.letter {
            C => 0,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
            A => 9,
            B => 11,
        } + self.accidental) % 12) as u8
    }

    /// Attempt to parse a pitch from a string. It should contain the name of the note in either
    /// uppercase or lowercase, followed by `#`, `s`, `S`, or `♯` for sharps and `b` or `♭` for
    /// flats.
    pub fn from_str(string: &str) -> Option<Self> {
        use NoteLetter::*;
        let mut characters = string.chars();

        let first_char = characters.next()?;
        let symbol = match first_char {
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
            return match second_char {
                '#' | 's' | 'S' | '♯' => {
                    Some(Pitch { letter: symbol, accidental: 1 })
                }
                'b' | '♭' => {
                    Some(Pitch { letter: symbol, accidental: -1 })
                }
                _ => None,
            }
        }

        if characters.next().is_some() {
            return None;
        }

        return Some(Pitch { letter: symbol, accidental: 0 })
    }

    /// Create a pitch by moving up the given pitch by an interval.
    pub fn from_interval(pitch: Self, interval: Interval) -> Self {
        let current_pitch = pitch.into_u8();
        let new_pitch = current_pitch + interval.semitone_count;

        Self::from_u8(new_pitch)
    }

    /// Create a pitch by moving down the given pitch by an interval.
    pub fn from_interval_down(pitch: Self, interval: Interval) -> Self {
        let current_pitch = pitch.into_u8();
        let new_pitch = (12 + (current_pitch as i16 - interval.semitone_count as i16)) % 12;

        Self::from_u8(new_pitch as u8)
    }

    /// Parse the pitch using a regex, with the same algorithm as described in `from_str`.
    pub fn from_regex(string: &str) -> Result<(Self, Match), NoteError> {
        let pitch_match = REGEX_PITCH.find(&string).ok_or(NoteError::InvalidPitch)?;

        let pitch = Self::from_str(&string[pitch_match.start()..pitch_match.end()])
            .ok_or(NoteError::InvalidPitch)?;

        Ok((pitch, pitch_match))
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use NoteLetter::*;
        let letter = match self.letter {
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            A => "A",
            B => "B",
        };

        let acc = if self.accidental < 0 { "b" } else { "#" };
        write!(fmt, "{}", letter.to_owned() + &(0..self.accidental.abs()).map(|_| acc).collect::<String>())
    }
}

impl FromStr for Pitch {
    type Err = NoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or(NoteError::InvalidPitch)
    }
}
