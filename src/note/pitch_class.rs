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

/// A pitch class (A, B, C#, etc).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum PitchSymbol { C, D, E, F, G, A, B }

pub fn pclass(symbol: PitchSymbol, accidental: i8) -> PitchClass {
    PitchClass { symbol, accidental }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PitchClass {
    pub symbol: PitchSymbol,
    pub accidental: i8,
}

impl PitchClass {
    /// Create a pitch class from an integer, where 0 is C and everything climbs up from there,
    /// looping back around once it reaches 12.
    pub fn from_u8(val: u8) -> Self {
        use PitchSymbol::*;
        return match val % 12 {
            0 => { pclass(C,0) }
            1 => { pclass(C,1) }
            2 => { pclass(D,0) }
            3 => { pclass(D,1) }
            4 => { pclass(E,0) }
            5 => { pclass(F,0) }
            6 => { pclass(F,1) }
            7 => { pclass(G,0) }
            8 => { pclass(G,1) }
            9 => { pclass(A,0) }
            10 => { pclass(A,1) }
            11 => { pclass(B,0) }
            _ => panic!("impossible")
        };
    }

    /// Attempt to parse this note from a string. It should contain the name of the note in either
    /// uppercase or lowercase, followed by `#`, `s`, `S`, or `♯` for sharps and `b` or `♭` for
    /// flats.
    pub fn from_str(string: &str) -> Option<Self> {
        use PitchSymbol::*;
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
            match second_char {
                '#' | 's' | 'S' | '♯' => {
                    return Some (PitchClass { symbol , accidental: 1 })
                }
                'b' | '♭' => {
                    return Some (PitchClass { symbol, accidental: -1 })
                }
                _ => return None,
            }
        }

        if characters.next().is_some() {
            return None;
        }

        return Some(PitchClass { symbol, accidental: 0 })
    }

    /// Create a note by moving up the given note by an interval.
    pub fn from_interval(pitch: Self, interval: Interval) -> Self {
        let current_pitch = pitch.into_u8();
        let new_pitch = current_pitch + interval.semitone_count;

        Self::from_u8(new_pitch)
    }

    /// Parse the note using a regex, with the same algorithm as described in `from_str`.
    pub fn from_regex(string: &str) -> Result<(Self, Match), NoteError> {
        let pitch_match = REGEX_PITCH.find(&string).ok_or(NoteError::InvalidPitch)?;

        let pitch_class = Self::from_str(&string[pitch_match.start()..pitch_match.end()])
            .ok_or(NoteError::InvalidPitch)?;

        Ok((pitch_class, pitch_match))
    }

    /// Convert the pitch class into its corresponding integer, where 0 is C and 11 is B.
    pub fn into_u8(self) -> u8 {
        use PitchSymbol::*;
        ((match self.symbol {
            C => 0,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
            A => 9,
            B => 11,
        } + self.accidental) % 12) as u8
        
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use PitchSymbol::*;
        let sym = match self.symbol {
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            A => "A",
            B => "B",
        };

        let acc = if self.accidental < 0 { "b" } else { "#" };
        write!(fmt, "{}", sym.to_owned() + &(0..self.accidental.abs()).map(|_| acc).collect::<String>())
    }
}

impl FromStr for PitchClass {
    type Err = NoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or(NoteError::InvalidPitch)
    }
}
