use std::fmt;

#[derive( Debug)]
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
