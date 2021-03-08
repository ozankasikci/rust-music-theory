use crate::chord::errors::ChordError;
use crate::chord::number::Number::Triad;
use crate::chord::{Number, Quality};
use crate::interval::Interval;
use crate::note::{Note, NoteError, Notes, Pitch, NoteLetter};

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
    pub fn with_inversion(
        root: Pitch,
        quality: Quality,
        number: Number,
        inversion: u8,
    ) -> Self {
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

    pub fn from_string(string: &str) -> Self {
        let notes: Vec<Pitch> = string.to_string()
                    .replace(",", "")
                    .split_whitespace()
                    .into_iter()
                    .map(|x| Pitch::from_str(x).expect(&format!("Invalid note {:?}.", x)))
                    .collect();

        let intervals: Vec<u8> = notes.iter()
                    .map(|&x| Pitch::into_u8(x) % 12)
                    .zip(notes[1..].iter().map(|&x| Pitch::into_u8(x)))
                    .map(|(x, y)| if x < y {y - x} else {y + 12 - x})
                    .collect();

        Chord::from_interval(notes[0], &intervals)
    }

    pub fn from_interval(root: Pitch, interval: &[u8]) -> Self {
        use Number::*;
        use Quality::*;
        let (quality, number) = match interval {
            &[4, 3] => (Major, Triad),
            &[3, 4] => (Minor, Triad),
            &[2, 5] => (Suspended2, Triad),
            &[5, 2] => (Suspended4, Triad),
            &[4, 4] => (Augmented, Triad),
            &[3, 3] => (Diminished, Triad),
            &[4, 3, 4] => (Major, Seventh),
            &[3, 4, 3] => (Minor, Seventh),
            &[4, 4, 2] => (Augmented, Seventh),
            &[4, 4, 3] => (Augmented, MajorSeventh),
            &[3, 3, 3] => (Diminished, Seventh),
            &[3, 3, 4] => (HalfDiminished, Seventh),
            &[3, 4, 4] => (Minor, MajorSeventh),
            &[4, 3, 3] => (Dominant, Seventh),
            &[4, 3, 3, 4] => (Dominant, Ninth),
            &[4, 3, 4, 3] => (Major, Ninth),
            &[4, 3, 3, 4, 4] => (Dominant, Eleventh),
            &[4, 3, 4, 3, 3] => (Major, Eleventh),
            &[3, 4, 3, 4, 3] => (Minor, Eleventh),
            &[4, 3, 3, 4, 3, 4] => (Dominant, Thirteenth),
            &[4, 3, 4, 3, 3, 4] => (Major, Thirteenth),
            &[3, 4, 3, 4, 3, 4] => (Minor, Thirteenth),
            _ => panic!(format!("Couldn't create chord! {:?}", interval))
        };
        Self::new(root, quality, number)
    }

    pub fn chord_intervals(quality: Quality, number: Number) -> Vec<Interval> {
        use Number::*;
        use Quality::*;
        match (&quality, &number) {
            (Major, Triad) => Interval::from_semitones(&[4, 3]),
            (Minor, Triad) => Interval::from_semitones(&[3, 4]),
            (Suspended2, Triad) => Interval::from_semitones(&[2, 5]),
            (Suspended4, Triad) => Interval::from_semitones(&[5, 7]),
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

        let chord = Chord::with_inversion(
            pitch,
            quality,
            number,
            inversion_num_option.unwrap_or(0),
        );

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
            root: Pitch { letter: NoteLetter::C, accidental: 0 },
            octave: 4,
            intervals: vec![],
            quality: Quality::Major,
            number: Number::Triad,
            inversion: 0,
        }
    }
}
