use crate::interval::errors::InvalidIntervalError;
use crate::note::{Note, PitchClass};
use strum_macros::Display;

mod errors;

#[derive(Display, Debug, Copy, Clone)]
pub enum Quality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

#[derive(Display, Debug, Copy, Clone)]
pub enum Number {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave,
}

#[derive(Display, Debug, Copy, Clone)]
pub enum Step {
    Half,
    Whole,
    Tritone,
}

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub semitone_count: u8,
    pub quality: Quality,
    pub number: Number,
    pub step: Option<Step>,
}

impl Interval {
    pub fn from_semitones(semi_tones: &[u8]) -> Result<Vec<Self>, errors::InvalidIntervalError> {
        let mut intervals: Vec<Interval> = vec![];

        if semi_tones.len() == 0 {
            return Err(InvalidIntervalError);
        }

        for i in semi_tones {
            let interval = Self::from_semitone(*i)?;
            intervals.push(interval);
        }

        Ok(intervals)
    }

    pub fn from_semitone(sc: u8) -> Result<Self, errors::InvalidIntervalError> {
        let (mut number, mut quality, mut step): (Number, Quality, Option<Step>);
        step = None;

        match sc {
            0 => {
                number = Number::Unison;
                quality = Quality::Perfect;
            }
            1 => {
                number = Number::Second;
                quality = Quality::Minor;
                step = Some(Step::Half);
            }
            2 => {
                number = Number::Second;
                quality = Quality::Major;
                step = Some(Step::Whole);
            }
            3 => {
                number = Number::Third;
                quality = Quality::Minor;
            }
            4 => {
                number = Number::Third;
                quality = Quality::Major;
            }
            5 => {
                number = Number::Fourth;
                quality = Quality::Perfect;
            }
            6 => {
                number = Number::Fifth;
                quality = Quality::Diminished;
                step = Some(Step::Tritone);
            }
            7 => {
                number = Number::Fifth;
                quality = Quality::Perfect;
            }
            8 => {
                number = Number::Sixth;
                quality = Quality::Minor;
            }
            9 => {
                number = Number::Sixth;
                quality = Quality::Major;
            }
            10 => {
                number = Number::Seventh;
                quality = Quality::Minor;
            }
            11 => {
                number = Number::Seventh;
                quality = Quality::Major;
            }
            12 => {
                number = Number::Octave;
                quality = Quality::Perfect;
            }
            _ => {
                return Err(InvalidIntervalError);
            }
        };

        Ok(Interval {
            semitone_count: sc,
            number,
            quality,
            step,
        })
    }

    pub fn second_note_from(&self, first_note: Note) -> Note {
        let pitch_class = PitchClass::from_interval(&first_note.pitch_class, self);
        let octave = first_note.octave;
        let excess_octave = (first_note.pitch_class as u8 + self.semitone_count - 1) / 12;

        Note {
            octave: octave + excess_octave,
            pitch_class,
        }
    }

    pub fn to_notes(root: Note, intervals: Vec<Interval>) -> Vec<Note> {
        let mut notes = vec![root];

        for i in 0..intervals.len() {
            let last_note = notes.last().unwrap();
            let interval_first_note = Note::new(last_note.pitch_class, last_note.octave);
            let interval_second_note = intervals[i].second_note_from(interval_first_note);
            notes.push(interval_second_note);
        };

        notes
    }
}
