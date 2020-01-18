use std::fmt;
use crate::interval::Interval;

#[derive(Debug, Copy, Clone)]
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
           1 =>  C,
            2 =>  Cs,
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
            _ => {
                B
            }
        }
    }

    pub fn from_interval(pitch: &Self,interval: &Interval) -> Self {
        let current_pitch = *pitch as u8;
        println!("Current pitch: {}, Semitone Count: {}", current_pitch, interval.semitone_count);

        let new_pitch = (current_pitch + interval.semitone_count) % 12;
        Self::from_u8(new_pitch)
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PitchClass::C => write!(fmt, "C"),
            PitchClass::Cs => write!(fmt, "C#"),
            PitchClass::D => write!(fmt, "D"),
            PitchClass::Ds => write!(fmt, "D#"),
            PitchClass::E => write!(fmt, "E"),
            PitchClass::F => write!(fmt, "F"),
            PitchClass::Fs => write!(fmt, "F#"),
            PitchClass::G => write!(fmt, "G"),
            PitchClass::Gs => write!(fmt, "G#"),
            PitchClass::A => write!(fmt, "A"),
            PitchClass::As => write!(fmt, "A#"),
            PitchClass::B => write!(fmt, "B"),
        }
    }
}
