use crate::interval::Interval;
use crate::note::{Note, PitchClass, Notes};
use crate::chord::{Quality, Number};
use regex::Match;
use crate::chord::errors::ChordError;

#[derive(Debug)]
pub struct Chord {
    pub root: PitchClass,
    pub octave: u8,
    pub intervals: Vec<Interval>,
    pub quality: Quality,
    pub number: Number,
}

impl Chord {
    pub fn new(root: PitchClass, quality: Quality, number: Number) -> Self {
        use Number::*;
        use Quality::*;
        let intervals = match (&quality, &number) {
            (Major, Triad) => Interval::from_semitones(&[4, 3]),
            (Minor, Triad) => Interval::from_semitones(&[3, 4]),
            (Augmented, Triad) => Interval::from_semitones(&[4, 4]),
            (Diminished, Triad) => Interval::from_semitones(&[3, 3]),
            (Major, Seventh) => Interval::from_semitones(&[4, 3, 4]),
            (Minor, Seventh) => Interval::from_semitones(&[3, 4, 3]),
            (Augmented, Seventh) => Interval::from_semitones(&[4, 4, 2]),
            (AugmentedMajor, Seventh) => Interval::from_semitones(&[4, 4, 3]),
            (Diminished, Seventh) => Interval::from_semitones(&[3, 3, 3]),
            (HalfDiminished, Seventh) => Interval::from_semitones(&[3, 3, 4]),
            (MinorMajor, Seventh) => Interval::from_semitones(&[3, 4, 4]),
            (Dominant, Seventh) => Interval::from_semitones(&[4, 3, 3]),
            _ => Interval::from_semitones(&[4, 3]),
        }
        .unwrap();

        Chord {
            root,
            octave: 4,
            intervals,
            quality,
            number,
        }
    }

    pub fn from_regex(string: &str) -> Result<Self, ChordError> {
        let (pitch_class, pitch_match) = PitchClass::from_regex(&string).unwrap();
        let (quality, quality_match) = Quality::from_regex(&string[pitch_match.end()..]).unwrap();
        let (number, number_match) = Number::from_regex(&string[quality_match.end()..]).unwrap();

        Ok(Chord::new(pitch_class, quality, number))
    }
}

impl Notes for Chord {
    fn notes(&self) -> Vec<Note> {
        let root_note = Note {
            octave: self.octave,
            pitch_class: self.root,
        };
        Interval::to_notes(root_note, self.intervals.clone())
    }
}


impl Default for Chord {
    fn default() -> Self {
        Chord {
            root: PitchClass::C,
            octave: 4,
            intervals: vec![],
            quality: Quality::Major,
            number: Number::Triad,
        }
    }
}
