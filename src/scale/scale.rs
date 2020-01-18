use crate::interval::Interval;
use crate::note::{Note, PitchClass};
use crate::scale::scale::Direction::Ascending;
use crate::scale::{Mode, ScaleType};

#[derive(Debug)]
pub enum Direction {
    Ascending,
    Descending,
}

#[derive(Debug)]
pub struct Scale {
    pub tonic: PitchClass,
    pub octave: u8,
    pub scale_type: ScaleType,
    pub mode: Mode,
    pub intervals: Vec<Interval>,
    pub direction: Direction,
}

impl Scale {
    pub fn new(scale_type: ScaleType, tonic: PitchClass, octave: u8) -> Self {
        let new_intervals = Interval::from_semitones;

        let intervals = match scale_type {
            ScaleType::Diatonic => new_intervals(&[2, 2, 1, 2, 2, 2, 1]),
            ScaleType::HarmonicMinor => new_intervals(&[2, 1, 2, 2, 1, 3, 1]),
            ScaleType::MelodicMinor => new_intervals(&[2, 1, 2, 2, 2, 2, 1]),
        };

        match intervals {
            Err(_) => Scale {
                tonic,
                octave,
                scale_type,
                mode: Mode::Ionian,
                ..Default::default()
            },
            Ok(i) => Scale {
                tonic,
                octave,
                scale_type,
                mode: Mode::Ionian,
                intervals: i,
                ..Default::default()
            },
        }
    }

    pub fn notes(&self) -> Vec<Note> {
        let root_note = Note {
            octave: self.octave,
            pitch_class: self.tonic,
        };

        let mut notes: Vec<Note> = vec![root_note];

        for i in 0..self.intervals.len() {
            let last_note = notes.last().unwrap();
            let interval_first_note = Note::new(last_note.pitch_class, last_note.octave);
            let interval_second_note = self.intervals[i].second_note_from(interval_first_note);
            notes.push(interval_second_note);
        }

        notes
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale {
            tonic: PitchClass::C,
            octave: 0,
            scale_type: ScaleType::Diatonic,
            mode: Mode::Ionian,
            intervals: vec![],
            direction: Direction::Ascending,
        }
    }
}
