use crate::interval::Interval;
use crate::note::{Note, PitchClass};
use crate::scale::{Mode, ScaleType};

#[derive(Debug)]
pub struct Scale {
    pub tonic: PitchClass,
    pub octave: i8,
    pub scale_type: ScaleType,
    pub mode: Mode,
    pub intervals: Vec<Interval>,
}

impl Scale {
    pub fn new(scale_type: ScaleType, tonic: PitchClass, octave: i8) -> Self {
        let newIntervals = Interval::from_semitones;

        let intervals = match scale_type {
            ScaleType::Diatonic => newIntervals(&[2, 2, 1, 2, 2, 2, 1]),
        };

        match intervals {
            Err(_) => Scale {
                tonic,
                octave,
                scale_type,
                mode: Mode::Ionian,
                intervals: vec![],
            },
            Ok(i) => Scale {
                tonic,
                octave,
                scale_type,
                mode: Mode::Ionian,
                intervals: i,
            },
        }
    }

    pub fn notes(&self) -> Vec<Note> {
        let mut notes: Vec<Note> = vec![];

        for i in 0..self.intervals.len() {
            let note = self.intervals[i].second_note(&Note::new(PitchClass::C, 4));
            println!("{:?}, {:?}", note, self.intervals[i]);
        }

        notes
    }
}
