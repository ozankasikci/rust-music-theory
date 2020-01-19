use crate::chord::ChordType;
use crate::interval::Interval;
use crate::note::{Note, PitchClass};

pub struct Chord {
    pub root: PitchClass,
    pub octave: u8,
    pub intervals: Vec<Interval>,
    pub chord_type: ChordType,
}

impl Chord {
    pub fn new(root: PitchClass, chord_type: ChordType) -> Self {
        use ChordType::*;
        let intervals = match chord_type {
            Major => Interval::from_semitones(&[4, 3]),
        }
        .unwrap();

        Chord {
            root,
            octave: 4,
            intervals,
            chord_type,
        }
    }

    pub fn notes(&self) -> Vec<Note> {
        let root_note = Note {
            octave: self.octave,
            pitch_class: self.root,
        };
        Interval::to_notes(root_note, self.intervals.clone())
    }
}
