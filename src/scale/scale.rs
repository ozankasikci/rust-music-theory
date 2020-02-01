use crate::interval::Interval;
use crate::note::{Accidental, Note, Notes, PitchClass};
use crate::scale::errors::ScaleError;
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
    pub mode: Option<Mode>,
    pub intervals: Vec<Interval>,
    pub direction: Direction,
    pub accidentals: Option<Vec<(Accidental, u8)>>,
}

impl Scale {
    pub fn new(
        scale_type: ScaleType,
        tonic: PitchClass,
        octave: u8,
        mode: Option<Mode>,
        accidentals: Option<Vec<(Accidental, u8)>>,
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
            accidentals,
            ..Default::default()
        })
    }

    pub fn from_regex(string: &str) -> Result<Self, ScaleError> {
        let (tonic, tonic_match) = PitchClass::from_regex(&string.trim())?;
        let mode_string = &string[tonic_match.end()..].trim();
        let (mode, _) = Mode::from_regex(mode_string)?;
        let scale_type = ScaleType::from_mode(&mode);
        let octave = 4;
        let scale = Scale::new(scale_type, tonic, octave, Some(mode), None)?;
        Ok(scale)
    }
}

impl Notes for Scale {
    fn notes(&self) -> Vec<Note> {
        use Mode::*;
        let root_note = Note {
            octave: self.octave,
            pitch_class: self.tonic,
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

        match &self.accidentals {
            Some(accidentals) => {
                for (accidental, degree) in accidentals {
                    let cur_interval_index = usize::from(*degree) - 2;

                    let mut cur_interval_semi_tone =
                        intervals_clone[cur_interval_index].semitone_count;
                    let mut next_interval_semi_tone =
                        intervals_clone[cur_interval_index + 1].semitone_count;

                    match accidental {
                        Accidental::Sharp => {
                            cur_interval_semi_tone = cur_interval_semi_tone + 1;
                            next_interval_semi_tone = next_interval_semi_tone - 1;
                        }
                        Accidental::Flat => {
                            cur_interval_semi_tone = cur_interval_semi_tone - 1;
                            next_interval_semi_tone = next_interval_semi_tone + 1;
                        }
                    }

                    let new_cur_interval_res = Interval::from_semitone(cur_interval_semi_tone);
                    let new_next_interval_res = Interval::from_semitone(next_interval_semi_tone);
                    if let (Ok(new_interval), Ok(new_next_interval)) =
                        (new_cur_interval_res, new_next_interval_res)
                    {
                        intervals_clone[cur_interval_index] = new_interval;
                        intervals_clone[cur_interval_index + 1] = new_next_interval;
                    }
                }
            }
            _ => {}
        }

        Interval::to_notes(root_note, intervals_clone)
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
            accidentals: None,
        }
    }
}
