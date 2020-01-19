use crate::interval::Interval;
use crate::note::{Note, PitchClass};

#[derive(Debug)]
pub enum Quality {
    MajorTriad,
    MinorTriad,
}

#[derive(Debug)]
pub struct Chord {
    pub root: PitchClass,
    pub octave: u8,
    pub intervals: Vec<Interval>,
    pub quality: Quality,
}

impl Chord {
    pub fn new(root: PitchClass, quality: Quality) -> Self {
        use Quality::*;
        let intervals = match quality {
            MajorTriad => Interval::from_semitones(&[4, 3]),
            MinorTriad => Interval::from_semitones(&[3, 4]),
        }
        .unwrap();

        Chord {
            root,
            octave: 4,
            intervals,
            quality,
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
