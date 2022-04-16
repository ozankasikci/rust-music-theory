use std::fmt;

use crate::chord::errors::ChordError;
use crate::chord::number::Number::Triad;
use crate::chord::{Number, Quality};
use crate::interval::Interval;
use crate::note::{Note, NoteError, NoteLetter, Notes, Pitch};

/// A chord.
#[derive(Debug, Clone)]
pub struct Chord {
    /// The root note of the chord.
    pub root: Pitch,
    /// The octave of the root note of the chord.
    pub octave: u8,
    /// The intervals within the chord.
    pub intervals: Vec<Interval>,
    /// The quality of the chord: major, minor, diminished, etc.
    pub quality: Quality,
    /// The superscript number of the chord: 3, 7, maj7, etc.
    pub number: Number,
    /// The inversion of the chord: 0=root position, 1=first inversion, etc.
    pub inversion: u8,
}

impl Chord {
    /// Create a new chord.
    pub fn new(root: Pitch, quality: Quality, number: Number) -> Self {
        Self::with_inversion(root, quality, number, 0)
    }

    /// Create a new chord with a given inversion.
    pub fn with_inversion(root: Pitch, quality: Quality, number: Number, inversion: u8) -> Self {
        let intervals = Self::chord_intervals(quality, number);
        let inversion = inversion % (intervals.len() + 1) as u8;
        Chord {
            root,
            octave: 4,
            intervals,
            quality,
            number,
            inversion,
        }
    }

    pub fn from_string(string: &str) -> Result<Self, ChordError> {
        let notes: Vec<Pitch> = string
            .to_string()
            .replace(",", "")
            .split_whitespace()
            .into_iter()
            .map(|x| Pitch::from_str(x).expect(&format!("Invalid note {:?}.", x)))
            .collect();

        let intervals: Vec<u8> = notes
            .iter()
            .map(|&x| Pitch::into_u8(x) % 12)
            .zip(notes[1..].iter().map(|&x| Pitch::into_u8(x)))
            .map(|(x, y)| if x < y { y - x } else { y + 12 - x })
            .collect();

        match unknown_position_interval(&intervals) {
            Some(info) => Ok(Self::with_inversion(
                notes[info.root_note_index],
                info.quality,
                info.number,
                info.inversion,
            )),
            None => Err(ChordError::InvalidUnknownChord),
        }
    }

    pub fn from_interval(root: Pitch, interval: &[u8]) -> Self {
        let (quality, number) = assume_root_position_interval(interval)
            .expect(&format!("Couldn't create chord! {:?}", interval));

        Self::new(root, quality, number)
    }

    pub fn chord_intervals(quality: Quality, number: Number) -> Vec<Interval> {
        use Number::*;
        use Quality::*;
        match (&quality, &number) {
            (Major, Triad) => Interval::from_semitones(&[4, 3]),
            (Minor, Triad) => Interval::from_semitones(&[3, 4]),
            (Suspended2, Triad) => Interval::from_semitones(&[2, 5]),
            (Suspended4, Triad) => Interval::from_semitones(&[5, 2]),
            (Augmented, Triad) => Interval::from_semitones(&[4, 4]),
            (Diminished, Triad) => Interval::from_semitones(&[3, 3]),
            (Major, Seventh) => Interval::from_semitones(&[4, 3, 4]),
            (Minor, Seventh) => Interval::from_semitones(&[3, 4, 3]),
            (Augmented, Seventh) => Interval::from_semitones(&[4, 4, 2]),
            (Augmented, MajorSeventh) => Interval::from_semitones(&[4, 4, 3]),
            (Diminished, Seventh) => Interval::from_semitones(&[3, 3, 3]),
            (HalfDiminished, Seventh) => Interval::from_semitones(&[3, 3, 4]),
            (Minor, MajorSeventh) => Interval::from_semitones(&[3, 4, 4]),
            (Dominant, Seventh) => Interval::from_semitones(&[4, 3, 3]),
            (Dominant, Ninth) => Interval::from_semitones(&[4, 3, 3, 4]),
            (Major, Ninth) => Interval::from_semitones(&[4, 3, 4, 3]),
            (Dominant, Eleventh) => Interval::from_semitones(&[4, 3, 3, 4, 4]),
            (Major, Eleventh) => Interval::from_semitones(&[4, 3, 4, 3, 3]),
            (Minor, Eleventh) => Interval::from_semitones(&[3, 4, 3, 4, 3]),
            (Dominant, Thirteenth) => Interval::from_semitones(&[4, 3, 3, 4, 3, 4]),
            (Major, Thirteenth) => Interval::from_semitones(&[4, 3, 4, 3, 3, 4]),
            (Minor, Thirteenth) => Interval::from_semitones(&[3, 4, 3, 4, 3, 4]),
            _ => Interval::from_semitones(&[4, 3]),
        }
        .unwrap()
    }

    /// Parse a chord using a regex.
    pub fn from_regex(string: &str) -> Result<Self, ChordError> {
        let (pitch, pitch_match) = Pitch::from_regex(&string)?;

        let slash_option = string.find('/');
        let bass_note_result = if let Some(slash) = slash_option {
            Pitch::from_regex(&string[slash + 1..].trim())
        } else {
            Err(NoteError::InvalidPitch)
        };
        let inversion_num_option = if let Some(slash) = slash_option {
            string[slash + 1..].trim().parse::<u8>().ok()
        } else {
            None
        };

        let (quality, quality_match_option) = Quality::from_regex(
            &string[pitch_match.end()..slash_option.unwrap_or_else(|| string.len())].trim(),
        )?;

        let number = if let Some(quality_match) = quality_match_option {
            Number::from_regex(&string[quality_match.end()..])
                .unwrap_or((Triad, None))
                .0
        } else {
            Triad
        };

        let chord =
            Chord::with_inversion(pitch, quality, number, inversion_num_option.unwrap_or(0));

        if let Ok((bass_note, _)) = bass_note_result {
            let inversion = chord
                .notes()
                .iter()
                .position(|note| note.pitch == bass_note)
                .unwrap_or(0);

            if inversion != 0 {
                return Ok(Chord::with_inversion(
                    pitch,
                    quality,
                    number,
                    inversion as u8,
                ));
            }
        }

        Ok(chord)
    }
}

impl Notes for Chord {
    fn notes(&self) -> Vec<Note> {
        let root_note = Note {
            pitch: self.root,
            octave: self.octave,
        };
        let mut notes = Interval::to_notes(root_note, self.intervals.clone());
        notes.rotate_left(self.inversion as usize);

        // Normalize to the correct octave
        if notes[0].octave > self.octave {
            let diff = notes[0].octave - self.octave;
            notes.iter_mut().for_each(|note| note.octave -= diff);
        }

        // Ensure that octave increments at the right notes
        for i in 1..notes.len() {
            if notes[i].pitch.into_u8() <= notes[i - 1].pitch.into_u8() {
                notes[i].octave = notes[i - 1].octave + 1;
            } else if notes[i].octave < notes[i - 1].octave {
                notes[i].octave = notes[i - 1].octave;
            }
        }
        notes
    }
}

impl Default for Chord {
    fn default() -> Self {
        Chord {
            root: Pitch {
                letter: NoteLetter::C,
                accidental: 0,
            },
            octave: 4,
            intervals: vec![],
            quality: Quality::Major,
            number: Number::Triad,
            inversion: 0,
        }
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let summary = format!("{} {} {}", self.root, self.quality, self.number);

        match self.inversion {
            0 => write!(f, "{}", summary),
            1 => write!(f, "{}, 1st Inversion", summary),
            2 => write!(f, "{}, 2nd Inversion", summary),
            3 => write!(f, "{}, 3rd Inversion", summary),
            n => write!(f, "{}, {}th Inversion", summary, n),
        }
    }
}

struct UnknownPositionInterval {
    quality: Quality,
    number: Number,
    root_note_index: usize,
    inversion: u8,
}

impl UnknownPositionInterval {
    fn new(quality: Quality, number: Number, root_note_index: usize, inversion: u8) -> Self {
        Self {
            quality,
            number,
            root_note_index,
            inversion,
        }
    }
}

/// # Purpose
/// Given a slice of intervals, return the:
/// - Chord Quality
/// - Chord Number
/// - Root Note Index (for the array of notes used to find the input intervals)
/// - Inversion Position
///
/// Returns `None` if the intervals are not a chord
///
/// # Implementation
/// In order, match intervals against known patterns for:
/// - Root position chords
/// - Inverted chords
fn unknown_position_interval(interval: &[u8]) -> Option<UnknownPositionInterval> {
    use Number::*;
    use Quality::*;

    match assume_root_position_interval(interval) {
        Some((quality, number)) => Some(UnknownPositionInterval::new(quality, number, 0, 0)),
        None => match interval {
            &[3, 5] => Some(UnknownPositionInterval::new(Major, Triad, 2, 1)),
            &[5, 4] => Some(UnknownPositionInterval::new(Major, Triad, 1, 2)),
            &[4, 5] => Some(UnknownPositionInterval::new(Minor, Triad, 2, 1)),
            &[5, 3] => Some(UnknownPositionInterval::new(Minor, Triad, 1, 2)),
            &[5, 5] => Some(UnknownPositionInterval::new(Suspended2, Triad, 2, 1)),
            _ => None,
            // Conflicts:
            // &[5, 2] => Sus2 Triad 2nd inversion, Sus4 Triad root position
            // &[4, 4] => Augmented Triad root position + 1st inversion + 2nd inversion
            // &[3, 3] => Diminished Triad root position + 1st inversion + 2nd inversion
        },
    }
}

/// Determine the chord quality and number assuming that the chord is in root position (tonic is bottom note)
fn assume_root_position_interval(interval: &[u8]) -> Option<(Quality, Number)> {
    use Number::*;
    use Quality::*;

    match interval {
        &[4, 3] => Some((Major, Triad)),
        &[3, 4] => Some((Minor, Triad)),
        &[2, 5] => Some((Suspended2, Triad)),
        &[5, 2] => Some((Suspended4, Triad)),
        &[4, 4] => Some((Augmented, Triad)),
        &[3, 3] => Some((Diminished, Triad)),
        &[4, 3, 4] => Some((Major, Seventh)),
        &[3, 4, 3] => Some((Minor, Seventh)),
        &[4, 4, 2] => Some((Augmented, Seventh)),
        &[4, 4, 3] => Some((Augmented, MajorSeventh)),
        &[3, 3, 3] => Some((Diminished, Seventh)),
        &[3, 3, 4] => Some((HalfDiminished, Seventh)),
        &[3, 4, 4] => Some((Minor, MajorSeventh)),
        &[4, 3, 3] => Some((Dominant, Seventh)),
        &[4, 3, 3, 4] => Some((Dominant, Ninth)),
        &[4, 3, 4, 3] => Some((Major, Ninth)),
        &[4, 3, 3, 4, 4] => Some((Dominant, Eleventh)),
        &[4, 3, 4, 3, 3] => Some((Major, Eleventh)),
        &[3, 4, 3, 4, 3] => Some((Minor, Eleventh)),
        &[4, 3, 3, 4, 3, 4] => Some((Dominant, Thirteenth)),
        &[4, 3, 4, 3, 3, 4] => Some((Major, Thirteenth)),
        &[3, 4, 3, 4, 3, 4] => Some((Minor, Thirteenth)),
        _ => None,
    }
}
