use crate::interval::errors::IntervalError;
use crate::note::{Note, PitchClass};
use strum_macros::Display;

/// The quality of an interval; major, minor, etc.
#[derive(Display, Debug, Copy, Clone)]
pub enum Quality {
    /// A perfect interval; unisons, fourths, fifths, and octaves.
    Perfect,
    /// A major interval.
    Major,
    /// A minor interval.
    Minor,
    /// An augmented interval.
    Augmented,
    /// A diminished interval.
    Diminished,
}

/// The number of an interval.
#[derive(Display, Debug, Copy, Clone)]
pub enum Number {
    /// The unison interval (the same note).
    Unison,
    /// The second interval.
    Second,
    /// The third interval.
    Third,
    /// The fourth interval.
    Fourth,
    /// The fifth interval.
    Fifth,
    /// The sixth interval.
    Sixth,
    /// The seventh interval.
    Seventh,
    /// The octave interval (the same note, but one octave above).
    Octave,
}

/// A step between notes.
#[derive(Display, Debug, Copy, Clone)]
pub enum Step {
    /// A semitone step.
    Half,
    /// A tone step.
    Whole,
    /// A tritone step.
    Tritone,
}

/// An interval between two notes.
#[derive(Debug, Copy, Clone)]
pub struct Interval {
    /// The number of semitones between the notes.
    pub semitone_count: u8,
    /// The quality of the interval.
    pub quality: Quality,
    /// The number of the interval.
    pub number: Number,
    /// The step of the interval.
    pub step: Option<Step>,
}

impl Interval {
    /// Create a new interval.
    pub fn new(semitone_count: u8, quality: Quality, number: Number, step: Option<Step>) -> Self {
        Interval {
            semitone_count,
            quality,
            number,
            step,
        }
    }

    /// Creates multiple intervals each based on the number of semitones from the root.
    ///
    /// # Errors
    ///
    /// Fails if `sc` is greater than 12.
    pub fn from_semitones(semi_tones: &[u8]) -> Result<Vec<Self>, IntervalError> {
        let mut intervals: Vec<Interval> = vec![];

        if semi_tones.is_empty() {
            return Err(IntervalError::InvalidInterval);
        }

        for i in semi_tones {
            let interval = Self::from_semitone(*i)?;
            intervals.push(interval);
        }

        Ok(intervals)
    }

    /// Create an interval based on the number of semitones from the root.
    ///
    /// # Errors
    ///
    /// Fails if `sc` is greater than 12.
    pub fn from_semitone(sc: u8) -> Result<Self, IntervalError> {
        let (number, quality, mut step): (Number, Quality, Option<Step>);
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
                return Err(IntervalError::InvalidInterval);
            }
        };

        Ok(Interval {
            semitone_count: sc,
            number,
            quality,
            step,
        })
    }

    /// Move the given note up by this interval.
    pub fn second_note_from(self, first_note: Note) -> Note {
        let pitch_class = PitchClass::from_interval(first_note.pitch_class, self);
        let octave = first_note.octave;
        let excess_octave = (first_note.pitch_class as u8 + self.semitone_count - 1) / 12;

        Note {
            octave: octave + excess_octave,
            pitch_class,
        }
    }

    /// Produce the list of notes that have had each interval applied in order.
    pub fn to_notes(root: Note, intervals: impl IntoIterator<Item = Interval>) -> Vec<Note> {
        let mut notes = vec![root];

        for interval in intervals {
            let last_note = notes.last().unwrap();
            let interval_first_note = Note::new(last_note.pitch_class, last_note.octave);
            let interval_second_note = interval.second_note_from(interval_first_note);
            notes.push(interval_second_note);
        }

        notes
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            semitone_count: 0,
            quality: Quality::Major,
            number: Number::Unison,
            step: None,
        }
    }
}
