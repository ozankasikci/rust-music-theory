use crate::interval::Interval;
use crate::note::{Note, NoteLetter, Notes, Pitch};
use crate::scale::errors::ScaleError;
use crate::scale::{Mode, ScaleType};
use strum_macros::Display;

/// The direction of the scale; up or down.
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Ascending,
    Descending,
}

fn raw_midi(note: &Note) -> i32 {
    use NoteLetter::*;
    let natural = match note.pitch.letter {
        C => 0,
        D => 2,
        E => 4,
        F => 5,
        G => 7,
        A => 9,
        B => 11,
    };
    (note.octave as i32 + 1) * 12 + natural + note.pitch.accidental as i32
}

fn synchronize_octaves(notes: &mut [Note], direction: Direction) {
    for index in 1..notes.len() {
        let previous = raw_midi(&notes[index - 1]);
        let pitch_value = raw_midi(&Note::new(notes[index].pitch, -1));
        let octave_plus_one = match direction {
            Direction::Ascending => (previous - pitch_value).div_euclid(12) + 1,
            Direction::Descending => (previous - 1 - pitch_value).div_euclid(12),
        };
        notes[index].octave = (octave_plus_one - 1).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
    }
}

/// A scale.
#[derive(Debug, Clone)]
pub struct Scale {
    /// The root note of the scale.
    pub tonic: Pitch,
    /// The octave of the root note of the scale.
    pub octave: i16,
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
        octave: i16,
        mode: Option<Mode>,
        direction: Direction,
    ) -> Result<Self, ScaleError> {
        if let Some(mode) = mode {
            if mode.scale_type() != scale_type {
                return Err(ScaleError::IncompatibleMode { scale_type, mode });
            }
        }

        let classical_melodic_descent = scale_type == ScaleType::MelodicMinor
            && direction == Direction::Descending
            && mode.map(Mode::rotation).unwrap_or(0) == 0;
        let mut intervals = match scale_type {
            ScaleType::Diatonic => Interval::from_semitones(&[2, 2, 1, 2, 2, 2, 1]),
            ScaleType::HarmonicMinor => Interval::from_semitones(&[2, 1, 2, 2, 1, 3, 1]),
            ScaleType::MelodicMinor if classical_melodic_descent => {
                // Preserve the classical base scale: it descends as natural minor.
                Interval::from_semitones(&[2, 1, 2, 2, 1, 2, 2])
            }
            ScaleType::MelodicMinor => Interval::from_semitones(&[2, 1, 2, 2, 2, 2, 1]),
            ScaleType::PentatonicMajor => Interval::from_semitones(&[2, 2, 3, 2, 3]),
            ScaleType::PentatonicMinor => Interval::from_semitones(&[3, 2, 2, 3, 2]),
            ScaleType::Blues => Interval::from_semitones(&[3, 2, 1, 1, 3, 2]),
            ScaleType::Chromatic => Interval::from_semitones(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            ScaleType::WholeTone => Interval::from_semitones(&[2, 2, 2, 2, 2, 2]),
        }?;

        if let Some(mode) = mode {
            if mode.scale_type() == scale_type {
                intervals.rotate_left(mode.rotation());
            }
        }

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

    pub fn absolute_intervals(&self) -> Vec<Interval> {
        let mut qualities = Vec::new();
        let mut sum = 0;
        for interval in &self.intervals {
            qualities.push(Interval::from_semitone(sum).unwrap());
            sum += interval.semitone_count;
        }
        qualities
    }
}

impl Notes for Scale {
    fn notes(&self) -> Vec<Note> {
        use Direction::*;

        let root_note = Note {
            octave: self.octave,
            pitch: self.tonic,
        };

        let intervals_clone = self.intervals.clone();

        let mut notes = match &self.direction {
            Ascending => Interval::to_notes(root_note, intervals_clone),
            Descending => Interval::to_notes_reverse(root_note, intervals_clone),
        };

        if self.scale_type == ScaleType::Chromatic {
            let major_steps = [2u8, 2, 1, 2, 2, 2, 1];
            let mut pitch_class = self.tonic.into_u8();
            let mut diatonic = vec![self.tonic];
            for (degree, step) in major_steps.iter().enumerate().take(6) {
                pitch_class = (pitch_class + step) % 12;
                diatonic.push(Pitch::from_u8_with_letter(
                    pitch_class,
                    self.tonic.letter.offset(degree as i16 + 1),
                ));
            }

            for note in &mut notes {
                let pitch_class = note.pitch.into_u8();
                note.pitch = if let Some(diatonic_pitch) =
                    diatonic.iter().find(|pitch| pitch.into_u8() == pitch_class)
                {
                    *diatonic_pitch
                } else {
                    let adjacent_pitch_class = match self.direction {
                        Ascending => (pitch_class + 11) % 12,
                        Descending => (pitch_class + 1) % 12,
                    };
                    let adjacent = diatonic
                        .iter()
                        .find(|pitch| pitch.into_u8() == adjacent_pitch_class)
                        .unwrap();
                    Pitch::from_u8_with_letter(pitch_class, adjacent.letter)
                };
            }
            notes.first_mut().unwrap().pitch = self.tonic;
            notes.last_mut().unwrap().pitch = self.tonic;
            synchronize_octaves(&mut notes, self.direction);
            return notes;
        }

        if self.scale_type == ScaleType::WholeTone {
            for note in &mut notes {
                note.pitch = Pitch::from_u8_with_direction(note.pitch.into_u8(), self.direction);
            }
            notes.first_mut().unwrap().pitch = self.tonic;
            notes.last_mut().unwrap().pitch = self.tonic;
            synchronize_octaves(&mut notes, self.direction);
            return notes;
        }

        // Scale spellings follow scale-degree letters. This keeps all seven
        // letter names in heptatonic scales and permits required double
        // accidentals in theoretical keys.
        let ascending_degrees: &[i16] = match self.scale_type {
            ScaleType::Diatonic | ScaleType::HarmonicMinor | ScaleType::MelodicMinor => {
                &[0, 1, 2, 3, 4, 5, 6, 7]
            }
            ScaleType::PentatonicMajor => &[0, 1, 2, 4, 5, 7],
            ScaleType::PentatonicMinor => &[0, 2, 3, 4, 6, 7],
            // Minor blues: 1, flat 3, 4, sharp 4, 5, flat 7.
            ScaleType::Blues => &[0, 2, 3, 3, 4, 6, 7],
            ScaleType::WholeTone => unreachable!(),
            ScaleType::Chromatic => unreachable!(),
        };
        let degree_offsets: Vec<i16> = match self.direction {
            Ascending => ascending_degrees.to_vec(),
            Descending => {
                let octave_degree = *ascending_degrees.last().unwrap();
                ascending_degrees
                    .iter()
                    .rev()
                    .map(|degree| degree - octave_degree)
                    .collect()
            }
        };
        for (note, degree_offset) in notes.iter_mut().zip(degree_offsets) {
            let letter = self.tonic.letter.offset(degree_offset);
            note.pitch = Pitch::from_u8_with_letter(note.pitch.into_u8(), letter);
        }

        synchronize_octaves(&mut notes, self.direction);

        notes
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale {
            tonic: Pitch {
                letter: NoteLetter::C,
                accidental: 0,
            },
            octave: 0,
            scale_type: ScaleType::Diatonic,
            mode: Some(Mode::Ionian),
            intervals: vec![],
            direction: Direction::Ascending,
        }
    }
}
