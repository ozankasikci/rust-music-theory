//! ToMidi trait for simple one-shot MIDI export.

use std::io;
use std::path::Path;

use crate::midi::{Channel, Duration, MidiBuilder, MidiFile, Velocity};
use crate::note::{Note, Notes};

/// Quick MIDI export for Chord, Scale, and other Notes implementors.
pub trait ToMidi: Notes {
    /// Convert to a MidiExport for configuration and saving.
    fn to_midi(&self, duration: Duration, velocity: Velocity) -> MidiExport {
        MidiExport::new(self.notes(), duration, velocity)
    }
}

// Blanket implementation for anything with Notes
impl<T: Notes> ToMidi for T {}

/// Intermediate type for chaining configuration before export.
pub struct MidiExport {
    notes: Vec<Note>,
    duration: Duration,
    velocity: Velocity,
    tempo: u16,
    channel: Channel,
    time_sig: (u8, u8),
}

impl MidiExport {
    /// Create a new MidiExport with default settings.
    pub fn new(notes: Vec<Note>, duration: Duration, velocity: Velocity) -> Self {
        Self {
            notes,
            duration,
            velocity,
            tempo: 120,
            channel: Channel::new(0).unwrap(),
            time_sig: (4, 4),
        }
    }

    /// Set the tempo in BPM.
    pub fn tempo(mut self, bpm: u16) -> Self {
        self.tempo = bpm;
        self
    }

    /// Set the MIDI channel.
    pub fn channel(mut self, ch: Channel) -> Self {
        self.channel = ch;
        self
    }

    /// Set the time signature.
    pub fn time_signature(mut self, numerator: u8, denominator: u8) -> Self {
        self.time_sig = (numerator, denominator);
        self
    }

    /// Convert to MIDI bytes.
    pub fn to_bytes(self) -> Vec<u8> {
        self.into_midi_file().to_bytes()
    }

    /// Save to a MIDI file.
    pub fn save<P: AsRef<Path>>(self, path: P) -> io::Result<()> {
        self.into_midi_file().save(path)
    }

    /// Convert into a MidiFile.
    fn into_midi_file(self) -> MidiFile {
        // Create a simple wrapper that implements Notes
        struct NoteVec(Vec<Note>);
        impl Notes for NoteVec {
            fn notes(&self) -> Vec<Note> {
                self.0.clone()
            }
        }

        let mut builder = MidiBuilder::new();
        builder.add(&NoteVec(self.notes), self.duration, self.velocity);

        MidiFile::new()
            .tempo(self.tempo)
            .time_signature(self.time_sig.0, self.time_sig.1)
            .track(builder, self.channel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::{Chord, Number, Quality};
    use crate::note::{Pitch, PitchSymbol::*};

    #[test]
    fn chord_to_midi() {
        let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
        let bytes = chord
            .to_midi(Duration::Quarter, Velocity::new(100).unwrap())
            .to_bytes();

        // Verify it's a valid MIDI file
        assert_eq!(&bytes[0..4], b"MThd");
    }

    #[test]
    fn scale_to_midi() {
        use crate::scale::{Direction, Mode, Scale, ScaleType};

        let scale = Scale::new(
            ScaleType::Diatonic,
            Pitch::from(C),
            4,
            Some(Mode::Ionian),
            Direction::Ascending,
        )
        .unwrap();

        let bytes = scale
            .to_midi(Duration::Eighth, Velocity::new(80).unwrap())
            .to_bytes();

        assert_eq!(&bytes[0..4], b"MThd");
    }

    #[test]
    fn to_midi_with_options() {
        let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
        let bytes = chord
            .to_midi(Duration::Quarter, Velocity::new(100).unwrap())
            .tempo(90)
            .channel(Channel::new(5).unwrap())
            .time_signature(3, 4)
            .to_bytes();

        assert_eq!(&bytes[0..4], b"MThd");
    }
}
