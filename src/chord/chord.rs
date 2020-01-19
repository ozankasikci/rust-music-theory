use crate::interval::Interval;
use crate::note::{Note, PitchClass};

#[derive(Debug)]
pub enum Quality {
    MajorTriad,
    MinorTriad,
    AugmentedTriad,
    DiminishedTriad,
    MajorSeventh,
    MinorSeventh,
    AugmentedSeventh,
    AugmentedMajorSeventh,
    DiminishedSeventh,
    HalfDiminishedSeventh,
    MinorMajorSeventh,
    DominantSeventh,
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
            AugmentedTriad => Interval::from_semitones(&[4, 4]),
            DiminishedTriad => Interval::from_semitones(&[3, 3]),
            MajorSeventh => Interval::from_semitones(&[4, 3, 4]),
            MinorSeventh => Interval::from_semitones(&[3, 4, 3]),
            AugmentedSeventh => Interval::from_semitones(&[4, 4, 2]),
            AugmentedMajorSeventh => Interval::from_semitones(&[4, 4, 3]),
            DiminishedSeventh => Interval::from_semitones(&[3, 3, 3]),
            HalfDiminishedSeventh => Interval::from_semitones(&[3, 3, 4]),
            MinorMajorSeventh => Interval::from_semitones(&[3, 4, 4]),
            DominantSeventh => Interval::from_semitones(&[4, 3, 3]),
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
