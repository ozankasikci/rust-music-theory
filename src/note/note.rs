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

pub trait Notes {
    fn notes(&self) -> Vec<Note>;

    fn print_notes(&self) {
        let notes = self.notes();

        println!("Notes:");
        for (i, note) in notes.iter().enumerate() {
            println!("  {}: {}", i + 1, note.pitch_class)
        }
    }
}
