use crate::interval::Interval;
use crate::note::errors::NoteError;
use crate::note::pitch_symbol::PitchSymbol;
use crate::scale::{Direction, Mode};
use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;
use strum_macros::EnumIter;

lazy_static! {
    static ref REGEX_PITCH: Regex = Regex::new("^[ABCDEFGabcdefg][b♭#sS♯𝄪x]*").unwrap();
}

/// A note letter without an accidental.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, Hash)]
pub enum NoteLetter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl NoteLetter {
    pub(crate) fn index(self) -> i16 {
        use NoteLetter::*;
        match self {
            C => 0,
            D => 1,
            E => 2,
            F => 3,
            G => 4,
            A => 5,
            B => 6,
        }
    }

    pub(crate) fn offset(self, steps: i16) -> Self {
        use NoteLetter::*;
        const LETTERS: [NoteLetter; 7] = [C, D, E, F, G, A, B];
        let current = self.index();
        LETTERS[(current + steps).rem_euclid(7) as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pitch {
    pub letter: NoteLetter,
    pub accidental: i8,
}

impl Pitch {
    /// Create a pitch with a given note letter and accidental
    pub fn new(letter: NoteLetter, accidental: i8) -> Self {
        Self { letter, accidental }
    }

    /// Create a pitch from an integer, where 0 is C and everything climbs up from there,
    /// looping back around once it reaches 12.
    pub fn from_u8(val: u8) -> Self {
        use NoteLetter::*;
        return match val % 12 {
            0 => Pitch::new(C, 0),
            1 => Pitch::new(C, 1),
            2 => Pitch::new(D, 0),
            3 => Pitch::new(D, 1),
            4 => Pitch::new(E, 0),
            5 => Pitch::new(F, 0),
            6 => Pitch::new(F, 1),
            7 => Pitch::new(G, 0),
            8 => Pitch::new(G, 1),
            9 => Pitch::new(A, 0),
            10 => Pitch::new(A, 1),
            11 => Pitch::new(B, 0),
            _ => unreachable!("val % 12 should always be 0-11"),
        };
    }

    /// Spell a pitch class using a required note letter.
    ///
    /// Tonal scales and tertian chords need both a sounding pitch class and a
    /// diatonic letter. For example, the minor third above B-flat is D-flat,
    /// not the enharmonically equivalent C-sharp.
    pub(crate) fn from_u8_with_letter(val: u8, letter: NoteLetter) -> Self {
        use NoteLetter::*;
        let natural_pitch = match letter {
            C => 0i16,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
            A => 9,
            B => 11,
        };
        let difference = val as i16 % 12 - natural_pitch;
        let accidental = (difference + 6).rem_euclid(12) - 6;
        Self::new(letter, accidental as i8)
    }

    /// Create a pitch from an integer with a preferred spelling based on mode and scale type
    pub fn from_u8_with_scale_context(val: u8, mode: Option<Mode>, direction: Direction) -> Self {
        use super::PitchSymbol;
        use Mode::*;
        use PitchSymbol::*;

        let pitch_number = val % 12;

        // Determine spelling based on mode and pitch number
        let use_flats = match (mode, pitch_number) {
            // Dorian: flat 3rd and 7th
            (Some(Dorian), 3 | 10) => true,
            // Phrygian: flat 2nd, 3rd, 6th, 7th
            (Some(Phrygian), 1 | 3 | 8 | 10) => true,
            // Lydian: all sharps
            (Some(Lydian), _) => false,
            // Mixolydian: flat 7th
            (Some(Mixolydian), 10) => true,
            // Aeolian: flat 3rd, 6th, 7th
            (Some(Aeolian), 3 | 8 | 10) => true,
            // Locrian: flat 2nd, 3rd, 5th, 6th, 7th
            (Some(Locrian), 1 | 3 | 6 | 8 | 10) => true,
            // For melodic minor and other modes, use the direction
            (None, _) => matches!(direction, Direction::Descending),
            // For other modes, use sharps by default
            _ => false,
        };

        if use_flats {
            match pitch_number {
                0 => Pitch::from(C),
                1 => Pitch::from(Db),
                2 => Pitch::from(D),
                3 => Pitch::from(Eb),
                4 => Pitch::from(E),
                5 => Pitch::from(F),
                6 => Pitch::from(Gb),
                7 => Pitch::from(G),
                8 => Pitch::from(Ab),
                9 => Pitch::from(A),
                10 => Pitch::from(Bb),
                11 => Pitch::from(B),
                _ => unreachable!(),
            }
        } else {
            match pitch_number {
                0 => Pitch::from(C),
                1 => Pitch::from(Cs),
                2 => Pitch::from(D),
                3 => Pitch::from(Ds),
                4 => Pitch::from(E),
                5 => Pitch::from(F),
                6 => Pitch::from(Fs),
                7 => Pitch::from(G),
                8 => Pitch::from(Gs),
                9 => Pitch::from(A),
                10 => Pitch::from(As),
                11 => Pitch::from(B),
                _ => unreachable!(),
            }
        }
    }

    /// Create a pitch from an integer with a preferred spelling based on direction
    pub fn from_u8_with_direction(val: u8, direction: Direction) -> Self {
        // Default to no mode and diatonic scale when only direction is provided
        Self::from_u8_with_scale_context(val, None, direction)
    }

    /// Convert the pitch into its corresponding integer, where 0 is C and 11 is B.
    pub fn into_u8(self) -> u8 {
        use NoteLetter::*;
        let base = match self.letter {
            C => 0,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
            A => 9,
            B => 11,
        };

        (base as i16 + self.accidental as i16).rem_euclid(12) as u8
    }

    /// Create a pitch by moving up the given pitch by an interval.
    pub fn from_interval(pitch: Self, interval: Interval) -> Self {
        let current_pitch = pitch.into_u8();
        let new_pitch = current_pitch + interval.semitone_count;
        let letter_steps = match interval.number {
            crate::interval::Number::Unison | crate::interval::Number::Octave => 0,
            crate::interval::Number::Second => 1,
            crate::interval::Number::Third => 2,
            crate::interval::Number::Fourth => 3,
            crate::interval::Number::Fifth => 4,
            crate::interval::Number::Sixth => 5,
            crate::interval::Number::Seventh => 6,
        };

        Self::from_u8_with_letter(new_pitch, pitch.letter.offset(letter_steps))
    }

    /// Create a pitch by moving down the given pitch by an interval.
    pub fn from_interval_down(pitch: Self, interval: Interval) -> Self {
        let current_pitch = pitch.into_u8();
        let new_pitch = (12 + (current_pitch as i16 - interval.semitone_count as i16)) % 12;
        let letter_steps = match interval.number {
            crate::interval::Number::Unison | crate::interval::Number::Octave => 0,
            crate::interval::Number::Second => 1,
            crate::interval::Number::Third => 2,
            crate::interval::Number::Fourth => 3,
            crate::interval::Number::Fifth => 4,
            crate::interval::Number::Sixth => 5,
            crate::interval::Number::Seventh => 6,
        };

        Self::from_u8_with_letter(new_pitch as u8, pitch.letter.offset(-letter_steps))
    }

    /// Create a pitch by moving up the given pitch by an interval with scale context.
    pub fn from_interval_with_context(
        pitch: Self,
        interval: Interval,
        _mode: Option<Mode>,
        _direction: Direction,
    ) -> Self {
        Self::from_interval(pitch, interval)
    }

    /// Create a pitch by moving down the given pitch by an interval with scale context.
    pub fn from_interval_down_with_context(
        pitch: Self,
        interval: Interval,
        _mode: Option<Mode>,
        _direction: Direction,
    ) -> Self {
        Self::from_interval_down(pitch, interval)
    }

    /// Attempt to parse a pitch from a string. It should contain the name of the note in either
    /// uppercase or lowercase, followed by `#`, `s`, `S`, or `♯` for sharps and `b` or `♭` for
    /// flats.
    pub fn from_str(string: &str) -> Option<Self> {
        use NoteLetter::*;
        let mut characters = string.chars();

        let first_char = characters.next()?;
        let letter = match first_char {
            'C' | 'c' => C,
            'D' | 'd' => D,
            'E' | 'e' => E,
            'F' | 'f' => F,
            'G' | 'g' => G,
            'A' | 'a' => A,
            'B' | 'b' => B,
            _ => return None,
        };

        let mut accidental = 0;
        let sharps: HashMap<char, i8> =
            [('#', 1), ('s', 1), ('S', 1), ('♯', 1), ('𝄪', 2), ('x', 2)]
                .iter()
                .cloned()
                .collect();
        let flats: HashMap<char, i8> = [('b', -1), ('♭', -1)].iter().cloned().collect();
        let mut active_map: Option<&HashMap<char, i8>> = None;
        for ch in characters {
            if let Some(map) = active_map {
                if !map.contains_key(&ch) {
                    return None;
                }
                accidental += map.get(&ch).unwrap();
            } else {
                if sharps.contains_key(&ch) {
                    active_map = Some(&sharps);
                    accidental += sharps.get(&ch).unwrap();
                } else if flats.contains_key(&ch) {
                    active_map = Some(&flats);
                    accidental += flats.get(&ch).unwrap();
                } else {
                    return None;
                }
            }
        }

        return Some(Pitch { letter, accidental });
    }

    /// Parse the pitch using a regex, with the same algorithm as described in `from_str`.
    pub fn from_regex(string: &str) -> Result<(Self, Match<'_>), NoteError> {
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
        write!(
            fmt,
            "{}",
            letter.to_owned() + &(0..self.accidental.abs()).map(|_| acc).collect::<String>()
        )
    }
}

impl FromStr for Pitch {
    type Err = NoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or(NoteError::InvalidPitch)
    }
}
