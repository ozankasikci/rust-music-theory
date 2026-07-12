use crate::chord::errors::ChordError;
use crate::chord::number::Number::Triad;
use crate::chord::{Number, Quality};
use crate::interval::Interval;
use crate::note::{Note, NoteLetter, Notes, Pitch};

/// A chord.
#[derive(Debug, Clone)]
pub struct Chord {
    /// The root note of the chord.
    pub root: Pitch,
    /// The octave of the root note of the chord.
    pub octave: i16,
    /// The intervals within the chord.
    pub intervals: Vec<Interval>,
    /// The quality of the chord: major, minor, diminished, etc.
    pub quality: Quality,
    /// The superscript number of the chord: 3, 7, maj7, etc.
    pub number: Number,
    /// The inversion of the chord: 0=root position, 1=first inversion, etc.
    pub inversion: u8,
    /// A non-chord bass note supplied by a slash chord such as C/F#.
    pub bass: Option<Pitch>,
}

impl Chord {
    /// Create a new chord.
    pub fn new(root: Pitch, quality: Quality, number: Number) -> Self {
        Self::try_new(root, quality, number).expect("unsupported chord quality/number combination")
    }

    /// Try to create a new chord, returning an error for unsupported quality/number combinations.
    pub fn try_new(root: Pitch, quality: Quality, number: Number) -> Result<Self, ChordError> {
        Self::try_with_inversion(root, quality, number, 0)
    }

    /// Create a new chord with a given inversion.
    pub fn with_inversion(root: Pitch, quality: Quality, number: Number, inversion: u8) -> Self {
        Self::try_with_inversion(root, quality, number, inversion)
            .expect("unsupported chord or invalid inversion")
    }

    /// Try to create a chord in a specific inversion.
    pub fn try_with_inversion(
        root: Pitch,
        quality: Quality,
        number: Number,
        inversion: u8,
    ) -> Result<Self, ChordError> {
        let intervals = Self::try_chord_intervals(quality, number)?;
        if inversion as usize > intervals.len() {
            return Err(ChordError::InvalidInversion(inversion));
        }
        Ok(Chord {
            root,
            octave: 4,
            intervals,
            quality,
            number,
            inversion,
            bass: None,
        })
    }

    pub fn from_string(string: &str) -> Result<Self, ChordError> {
        let normalized = string.replace(',', "");
        let notes: Vec<Pitch> = normalized
            .split_whitespace()
            .map(|pitch| Pitch::from_str(pitch).ok_or(ChordError::InvalidRegex))
            .collect::<Result<_, _>>()?;

        if notes.is_empty() {
            return Err(ChordError::UnknownIntervalPattern(vec![]));
        }

        let intervals: Vec<u8> = notes
            .windows(2)
            .map(|window| {
                let first = window[0].into_u8();
                let second = window[1].into_u8();
                if first < second {
                    second - first
                } else {
                    second + 12 - first
                }
            })
            .collect();

        Chord::from_interval(notes[0], &intervals)
    }

    pub fn from_interval(root: Pitch, interval: &[u8]) -> Result<Self, ChordError> {
        use Number::*;
        use Quality::*;
        let (quality, number) = match *interval {
            [4, 3] => (Major, Triad),
            [3, 4] => (Minor, Triad),
            [2, 5] => (Suspended2, Triad),
            [5, 2] => (Suspended4, Triad),
            [4, 4] => (Augmented, Triad),
            [3, 3] => (Diminished, Triad),
            [4, 3, 4] => (Major, Seventh),
            [3, 4, 3] => (Minor, Seventh),
            [4, 4, 2] => (Augmented, Seventh),
            [4, 4, 3] => (Augmented, MajorSeventh),
            [3, 3, 3] => (Diminished, Seventh),
            [3, 3, 4] => (HalfDiminished, Seventh),
            [3, 4, 4] => (Minor, MajorSeventh),
            [4, 3, 3] => (Dominant, Seventh),
            [4, 3, 3, 4] => (Dominant, Ninth),
            [4, 3, 4, 3] => (Major, Ninth),
            [3, 4, 3, 4] => (Minor, Ninth),
            [4, 3, 3, 4, 3] => (Dominant, Eleventh),
            [4, 3, 4, 3, 3] => (Major, Eleventh),
            [3, 4, 3, 4, 3] => (Minor, Eleventh),
            [4, 3, 3, 4, 3, 4] => (Dominant, Thirteenth),
            [4, 3, 4, 3, 3, 4] => (Major, Thirteenth),
            [3, 4, 3, 4, 3, 4] => (Minor, Thirteenth),
            _ => return Err(ChordError::UnknownIntervalPattern(interval.to_vec())),
        };
        Self::try_new(root, quality, number)
    }

    pub fn chord_intervals(quality: Quality, number: Number) -> Vec<Interval> {
        Self::try_chord_intervals(quality, number)
            .expect("unsupported chord quality/number combination")
    }

    /// Return the adjacent intervals for a supported chord.
    pub fn try_chord_intervals(
        quality: Quality,
        number: Number,
    ) -> Result<Vec<Interval>, ChordError> {
        use Number::*;
        use Quality::*;
        let semitones: &[u8] = match (&quality, &number) {
            (Major, Triad) => &[4, 3],
            (Minor, Triad) => &[3, 4],
            (Suspended2, Triad) => &[2, 5],
            (Suspended4, Triad) => &[5, 2],
            (Augmented, Triad) => &[4, 4],
            (Diminished, Triad) => &[3, 3],
            (Major, Seventh | MajorSeventh) => &[4, 3, 4],
            (Minor, Seventh) => &[3, 4, 3],
            (Augmented, Seventh) => &[4, 4, 2],
            (Augmented, MajorSeventh) => &[4, 4, 3],
            (Diminished, Seventh) => &[3, 3, 3],
            (HalfDiminished, Seventh) => &[3, 3, 4],
            (Minor, MajorSeventh) => &[3, 4, 4],
            (Dominant, Seventh) => &[4, 3, 3],
            (Dominant, Ninth) => &[4, 3, 3, 4],
            (Major, Ninth) => &[4, 3, 4, 3],
            (Minor, Ninth) => &[3, 4, 3, 4],
            (Dominant, Eleventh) => &[4, 3, 3, 4, 3],
            (Major, Eleventh) => &[4, 3, 4, 3, 3],
            (Minor, Eleventh) => &[3, 4, 3, 4, 3],
            (Dominant, Thirteenth) => &[4, 3, 3, 4, 3, 4],
            (Major, Thirteenth) => &[4, 3, 4, 3, 3, 4],
            (Minor, Thirteenth) => &[3, 4, 3, 4, 3, 4],
            _ => {
                return Err(ChordError::UnsupportedChord(format!(
                    "{} {}",
                    quality, number
                )))
            }
        };
        Ok(Interval::from_semitones(semitones).unwrap())
    }

    /// Parse a chord using a regex.
    pub fn from_regex(string: &str) -> Result<Self, ChordError> {
        let string = string.trim();
        let (pitch, pitch_match) = Pitch::from_regex(string)?;
        let remainder = &string[pitch_match.end()..];
        let (descriptor, slash) = if let Some((descriptor, slash)) = remainder.split_once('/') {
            if slash.contains('/') || slash.trim().is_empty() {
                return Err(ChordError::InvalidRegex);
            }
            (descriptor.trim(), Some(slash.trim()))
        } else {
            (remainder.trim(), None)
        };

        let (quality, number) = if descriptor.is_empty() {
            (Quality::Major, Triad)
        } else if let Ok((number, number_match)) = Number::from_regex(descriptor) {
            let number_match = number_match.ok_or(ChordError::InvalidRegex)?;
            if number_match.start() != 0 || number_match.end() != descriptor.len() {
                return Err(ChordError::InvalidRegex);
            }
            let quality = match number {
                Number::MajorSeventh => Quality::Major,
                Number::Seventh | Number::Ninth | Number::Eleventh | Number::Thirteenth => {
                    Quality::Dominant
                }
                Number::Triad => Quality::Major,
            };
            (quality, number)
        } else if let Some((quality, number_string)) = descriptor
            .strip_prefix('m')
            .map(|rest| (Quality::Minor, rest))
            .or_else(|| descriptor.strip_prefix('M').map(|rest| (Quality::Major, rest)))
            .filter(|(_, rest)| !rest.is_empty() && Number::from_regex(rest).is_ok())
        {
            let (number, number_match) = Number::from_regex(number_string)?;
            let number_match = number_match.ok_or(ChordError::InvalidRegex)?;
            if number_match.start() != 0 || number_match.end() != number_string.len() {
                return Err(ChordError::InvalidRegex);
            }
            (quality, number)
        } else {
            let (quality, quality_match) = Quality::from_regex(descriptor)?;
            let quality_match = quality_match.ok_or(ChordError::InvalidRegex)?;
            if quality_match.start() != 0 {
                return Err(ChordError::InvalidRegex);
            }
            let number_string = descriptor[quality_match.end()..].trim();
            let number = if number_string.is_empty() {
                Triad
            } else {
                let (number, number_match) = Number::from_regex(number_string)?;
                let number_match = number_match.ok_or(ChordError::InvalidRegex)?;
                if number_match.start() != 0 || number_match.end() != number_string.len() {
                    return Err(ChordError::InvalidRegex);
                }
                number
            };
            (quality, number)
        };

        let mut chord = Chord::try_new(pitch, quality, number)?;
        if let Some(slash) = slash {
            if let Ok(inversion) = slash.parse::<u8>() {
                return Chord::try_with_inversion(pitch, quality, number, inversion);
            }
            let (bass_note, bass_match) = Pitch::from_regex(slash)?;
            if bass_match.end() != slash.len() {
                return Err(ChordError::InvalidRegex);
            }
            let inversion = chord
                .notes()
                .iter()
                .position(|note| note.pitch == bass_note)
                .map(|position| position as u8);
            if let Some(inversion) = inversion {
                chord = Chord::try_with_inversion(pitch, quality, number, inversion)?;
            } else {
                chord.bass = Some(bass_note);
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

        // Chord members are spelled by their generic interval above the root,
        // rather than by a global sharp/flat preference.
        let letter_offsets: &[i16] = match (self.quality, self.number) {
            (Quality::Suspended2, Number::Triad) => &[0, 1, 4],
            (Quality::Suspended4, Number::Triad) => &[0, 3, 4],
            (_, Number::Triad) => &[0, 2, 4],
            (_, Number::Seventh | Number::MajorSeventh) => &[0, 2, 4, 6],
            (_, Number::Ninth) => &[0, 2, 4, 6, 8],
            (_, Number::Eleventh) => &[0, 2, 4, 6, 8, 10],
            (_, Number::Thirteenth) => &[0, 2, 4, 6, 8, 10, 12],
        };
        for (note, letter_offset) in notes.iter_mut().zip(letter_offsets) {
            let letter = self.root.letter.offset(*letter_offset);
            note.pitch = Pitch::from_u8_with_letter(note.pitch.into_u8(), letter);
        }

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

        if let Some(bass) = self.bass {
            let first = &notes[0];
            let bass_octave = if bass.into_u8() < first.pitch.into_u8() {
                first.octave
            } else {
                first.octave - 1
            };
            notes.insert(0, Note::new(bass, bass_octave));
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
            bass: None,
        }
    }
}
