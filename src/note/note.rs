use crate::note::PitchClass;
use std::fmt;
use std::fmt::Formatter;

/// A note.
#[derive(Debug, Clone)]
pub struct Note {
    /// The pitch class of the note (A, B, C#, etc).
    pub pitch_class: PitchClass,
    /// The octave of the note in standard notation.
    pub octave: u8,
}

impl Note {
    /// Create a new note.
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

/// A type that can produce a sequence of notes.
pub trait Notes {
    /// Get the sequence of notes.
    fn notes(&self) -> Vec<Note>;

    /// Print the sequence of notes.
    ///
    /// By default this function will print out each notes' index and its pitch class. For example,
    /// printing out C major would look like:
    /// ```text
    /// Notes:
    ///   1: C
    ///   2: E
    ///   3: G
    /// ```
    fn print_notes(&self) {
        let notes = self.notes();

        println!("Notes:");
        for (i, note) in notes.iter().enumerate() {
            println!("  {}: {}", i + 1, note.pitch_class)
        }
    }
}
