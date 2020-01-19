use crate::interval::Interval;
use crate::note::{Note, PitchClass};
use crate::scale::scale::Direction::Ascending;
use crate::scale::{Mode, ScaleType};
use std::fmt::Error;

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
    pub mode: Option<Mode>,
    pub intervals: Vec<Interval>,
    pub direction: Direction,
}

impl Scale {
    pub fn new(
        scale_type: ScaleType,
        tonic: PitchClass,
        octave: u8,
        mode: Mode,
    ) -> Result<Self, String> {
        let new_intervals = Interval::from_semitones;

        let intervals = match scale_type {
            ScaleType::Diatonic => new_intervals(&[2, 2, 1, 2, 2, 2, 1]),
            ScaleType::HarmonicMinor => new_intervals(&[2, 1, 2, 2, 1, 3, 1]),
            ScaleType::MelodicMinor => new_intervals(&[2, 1, 2, 2, 2, 2, 1]),
        }
        .or(Err("Cant determine the intervals for the scale"))?;

        Ok(Scale {
            tonic,
            octave,
            scale_type,
            mode: Some(mode),
            intervals,
            ..Default::default()
        })
    }

    pub fn notes(&self) -> Vec<Note> {
        use Mode::*;
        let root_note = Note {
            octave: self.octave,
            pitch_class: self.tonic,
        };

        let mut notes: Vec<Note> = vec![root_note];

        let mut intervals_clone = self.intervals.clone();

        match &self.mode {
            None => {}
            Some(mode) => {
                match mode {
                    Ionian => {}
                    Dorian => intervals_clone.rotate_left(1),
                    Phrygian => intervals_clone.rotate_left(2),
                    Lydian => intervals_clone.rotate_left(3),
                    Mixolydian => intervals_clone.rotate_left(4),
                    Aeolian => intervals_clone.rotate_right(2),
                    Locrian => intervals_clone.rotate_right(1),
                };
            }
        };

        for i in 0..intervals_clone.len() {
            let last_note = notes.last().unwrap();
            let interval_first_note = Note::new(last_note.pitch_class, last_note.octave);
            let interval_second_note = intervals_clone[i].second_note_from(interval_first_note);
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
            mode: Some(Mode::Ionian),
            intervals: vec![],
            direction: Direction::Ascending,
        }
    }
}
