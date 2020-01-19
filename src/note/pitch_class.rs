use crate::interval::Interval;
use strum_macros::{EnumIter};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum PitchClass {
    C = 1,
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
            1 => C,
            2 => Cs,
            3 => D,
            4 => Ds,
            5 => E,
            6 => F,
            7 => Fs,
            8 => G,
            9 => Gs,
            10 => A,
            11 => As,
            12 => B,
            rest => Self::from_u8(val % 12),
        }
    }

    pub fn from_interval(pitch: &Self, interval: &Interval) -> Self {
        let current_pitch = *pitch as u8;
        let new_pitch = current_pitch + interval.semitone_count;

        Self::from_u8(new_pitch)
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
