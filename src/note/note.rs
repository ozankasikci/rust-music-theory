use crate::note::PitchClass;
use std::fmt;
use std::fmt::{Error, Formatter};

#[derive(Debug)]
pub struct Note {
    pub pitch_class: PitchClass,
    pub octave: u8,
}

impl Note {
    pub fn new(pitch_class: PitchClass, octave: u8) -> Self {
        Note {
            pitch_class,
            octave,
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.pitch_class)
    }
}
