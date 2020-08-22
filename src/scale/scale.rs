use crate::interval::Interval;
use crate::note::{Note, Notes, Pitch, NoteLetter};
use crate::scale::errors::ScaleError;
use crate::scale::{Mode, ScaleType};
use strum_macros::Display;

/// The direction of the scale; up or down.
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Ascending,
    Descending,
}

/// A scale.
#[derive(Debug, Clone)]
pub struct Scale {
    /// The root note of the scale.
    pub tonic: Pitch,
    /// The octave of the root note of the scale.
    pub octave: u8,
    /// The type of scale (diatonic, melodic minor, harmonic minor).
    pub scale_type: ScaleType,
    /// The mode of the scale.
    pub mode: Option<Mode>,
    /// The list of intervals in the scale.
    pub intervals: Vec<Interval>,
    /// The direction of the scale, ascending or descending.
    pub direction: Direction,
}

impl Scale {
    /// Create a new scale with a given direction.
    pub fn new(
        scale_type: ScaleType,
        tonic: Pitch,
        octave: u8,
        mode: Option<Mode>,
        direction: Direction,
    ) -> Result<Self, ScaleError> {
        let intervals = match scale_type {
            ScaleType::Diatonic => Interval::from_semitones(&[2, 2, 1, 2, 2, 2, 1]),
            ScaleType::HarmonicMinor => Interval::from_semitones(&[2, 1, 2, 2, 1, 3, 1]),
            ScaleType::MelodicMinor => Interval::from_semitones(&[2, 1, 2, 2, 2, 2, 1]),
        }?;

        Ok(Scale {
            tonic,
            octave,
            scale_type,
            mode,
            intervals,
            direction,
        })
    }

    /// Parse a scale from a regex.
    pub fn from_regex_in_direction(string: &str, direction: Direction) -> Result<Self, ScaleError> {
        let (tonic, tonic_match) = Pitch::from_regex(&string.trim())?;
        let mode_string = &string[tonic_match.end()..].trim();
        let (mode, _) = Mode::from_regex(mode_string)?;
        let scale_type = ScaleType::from_mode(mode);
        let octave = 4;
        let scale = Scale::new(scale_type, tonic, octave, Some(mode), direction)?;
        Ok(scale)
    }

    pub fn from_regex(string: &str) -> Result<Self, ScaleError> {
        Self::from_regex_in_direction(string, Direction::Ascending)
    }
}

impl Notes for Scale {
    fn notes(&self) -> Vec<Note> {
        use Direction::*;
        use Mode::*;
        let root_note = Note {
            octave: self.octave,
            pitch: self.tonic,
        };

        let mut intervals_clone = self.intervals.clone();

        // shift the scale based on the mode
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
                    _ => {}
                };
            }
        };

        match &self.direction {
            Ascending => Interval::to_notes(root_note, intervals_clone),
            Descending => Interval::to_notes_reverse(root_note, intervals_clone),
        }
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale {
            tonic: Pitch { letter: NoteLetter::C, accidental: 0 },
            octave: 0,
            scale_type: ScaleType::Diatonic,
            mode: Some(Mode::Ionian),
            intervals: vec![],
            direction: Direction::Ascending,
        }
    }
}
