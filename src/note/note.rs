use crate::note::Pitch;
use std::fmt;
use std::fmt::Formatter;

/// A note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    /// The pitch of the note (A, B, C#, etc).
    pub pitch: Pitch,
    /// The octave of the note in standard notation.
    pub octave: i16,
}

impl Note {
    /// Create a new note.
    pub fn new(pitch: Pitch, octave: i16) -> Self {
        Note { pitch, octave }
    }

    /// Convert to MIDI pitch number (0-127).
    ///
    /// Middle C (C4) = 60, A4 (440Hz) = 69.
    /// Uses standard MIDI octave convention where octave -1 starts at 0.
    pub fn midi_pitch(&self) -> u8 {
        let semitone = self.pitch.into_u8();
        let midi_value = (self.octave as u16 + 1) * 12 + semitone as u16;
        midi_value.min(127) as u8
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.pitch)
    }
}

/// A type that can produce a sequence of notes.
pub trait Notes {
    /// Get the sequence of notes.
    fn notes(&self) -> Vec<Note>;

    /// Print the sequence of notes.
    ///
    /// By default this function will print out each notes' index and its pitch class. For example,
    /// printing out C major would look like:
    /// ```text
    /// Notes:
    ///   1: C
    ///   2: E
    ///   3: G
    /// ```
    fn print_notes(&self) {
        let notes = self.notes();

        println!("Notes:");
        for (i, note) in notes.iter().enumerate() {
            println!("  {}: {}", i + 1, note.pitch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::PitchSymbol::*;

    #[test]
    fn midi_pitch_middle_c() {
        let note = Note::new(Pitch::from(C), 4);
        assert_eq!(note.midi_pitch(), 60);
    }

    #[test]
    fn midi_pitch_a440() {
        let note = Note::new(Pitch::from(A), 4);
        assert_eq!(note.midi_pitch(), 69);
    }

    #[test]
    fn midi_pitch_octaves() {
        assert_eq!(Note::new(Pitch::from(C), 0).midi_pitch(), 12);
        assert_eq!(Note::new(Pitch::from(C), 1).midi_pitch(), 24);
        assert_eq!(Note::new(Pitch::from(C), 2).midi_pitch(), 36);
        assert_eq!(Note::new(Pitch::from(C), 3).midi_pitch(), 48);
        assert_eq!(Note::new(Pitch::from(C), 5).midi_pitch(), 72);
    }

    #[test]
    fn midi_pitch_accidentals() {
        assert_eq!(Note::new(Pitch::from(Cs), 4).midi_pitch(), 61);
        assert_eq!(Note::new(Pitch::from(Db), 4).midi_pitch(), 61);
        assert_eq!(Note::new(Pitch::from(Fs), 4).midi_pitch(), 66);
    }

    #[test]
    fn midi_pitch_clamps_to_127() {
        // Very high octave should clamp
        let note = Note::new(Pitch::from(G), 10);
        assert!(note.midi_pitch() <= 127);
    }
}
