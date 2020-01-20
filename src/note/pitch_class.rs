use crate::interval::Interval;
use strum_macros::{EnumIter};
use std::fmt;

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
