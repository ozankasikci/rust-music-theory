//! MIDI track builder with sequential and absolute positioning.

use crate::midi::event::MidiEvent;
use crate::midi::{Channel, Duration, Velocity};
use crate::note::Notes;

/// Default PPQ (Pulses Per Quarter Note).
pub const DEFAULT_PPQ: u16 = 480;

/// Builds a single MIDI track with sequential/absolute positioning.
#[derive(Debug, Clone)]
pub struct MidiBuilder {
    pub(crate) events: Vec<MidiEvent>,
    pub(crate) cursor: u32,
    pub(crate) ppq: u16,
    pub(crate) channel: Channel,
}

impl MidiBuilder {
    /// Create a new MIDI builder with default settings.
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            cursor: 0,
            ppq: DEFAULT_PPQ,
            channel: Channel::new(0).unwrap(),
        }
    }

    /// Create a new MIDI builder with a specific PPQ.
    pub fn with_ppq(ppq: u16) -> Self {
        Self {
            events: Vec::new(),
            cursor: 0,
            ppq,
            channel: Channel::new(0).unwrap(),
        }
    }

    /// Get the current cursor position in ticks.
    pub fn cursor(&self) -> u32 {
        self.cursor
    }

    /// Get the PPQ setting.
    pub fn ppq(&self) -> u16 {
        self.ppq
    }

    /// Add notes from anything implementing the Notes trait.
    ///
    /// All notes are played simultaneously (chord-style) for the given duration.
    /// The cursor advances by the duration after adding.
    pub fn add<N: Notes>(
        &mut self,
        notes: &N,
        duration: Duration,
        velocity: Velocity,
    ) -> &mut Self {
        let ticks = duration.to_ticks(self.ppq);
        let note_off_tick = self.cursor + ticks;

        for note in notes.notes() {
            let pitch = note.midi_pitch();

            self.events.push(MidiEvent::NoteOn {
                tick: self.cursor,
                channel: self.channel,
                pitch,
                velocity,
            });

            self.events.push(MidiEvent::NoteOff {
                tick: note_off_tick,
                channel: self.channel,
                pitch,
            });
        }

        self.cursor = note_off_tick;
        self
    }

    /// Insert silence (rest) of the given duration.
    ///
    /// Advances the cursor without creating any note events.
    pub fn rest(&mut self, duration: Duration) -> &mut Self {
        self.cursor += duration.to_ticks(self.ppq);
        self
    }

    /// Jump to an absolute beat position.
    ///
    /// Useful for layering multiple parts or jumping to specific points.
    /// Beat 0.0 is the start, 1.0 is one quarter note in, etc.
    pub fn at_beat(&mut self, beat: f32) -> &mut Self {
        self.cursor = (beat * self.ppq as f32) as u32;
        self
    }

    /// Jump to an absolute tick position.
    pub fn at_tick(&mut self, tick: u32) -> &mut Self {
        self.cursor = tick;
        self
    }

    /// Insert a tempo change at the current position.
    ///
    /// Tempo is specified in BPM (beats per minute).
    pub fn tempo(&mut self, bpm: u16) -> &mut Self {
        self.events.push(MidiEvent::Tempo {
            tick: self.cursor,
            microseconds_per_beat: MidiEvent::bpm_to_microseconds(bpm),
        });
        self
    }

    /// Insert a time signature change at the current position.
    ///
    /// For example, `time_signature(6, 8)` sets 6/8 time.
    pub fn time_signature(&mut self, numerator: u8, denominator: u8) -> &mut Self {
        self.events.push(MidiEvent::TimeSignature {
            tick: self.cursor,
            numerator,
            denominator,
        });
        self
    }
}

impl Default for MidiBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::{Chord, Number, Quality};
    use crate::note::{Pitch, PitchSymbol::*};

    #[test]
    fn new_builder_defaults() {
        let builder = MidiBuilder::new();
        assert_eq!(builder.cursor(), 0);
        assert_eq!(builder.ppq(), 480);
        assert!(builder.events.is_empty());
    }

    #[test]
    fn builder_with_custom_ppq() {
        let builder = MidiBuilder::with_ppq(960);
        assert_eq!(builder.ppq(), 960);
    }

    #[test]
    fn add_chord_creates_note_events() {
        let mut builder = MidiBuilder::new();
        let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
        let velocity = Velocity::new(100).unwrap();

        builder.add(&chord, Duration::Quarter, velocity);

        // C major triad = 3 notes, each needs NoteOn and NoteOff
        assert_eq!(builder.events.len(), 6);
    }

    #[test]
    fn add_advances_cursor() {
        let mut builder = MidiBuilder::new();
        let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
        let velocity = Velocity::new(100).unwrap();

        assert_eq!(builder.cursor(), 0);
        builder.add(&chord, Duration::Quarter, velocity);
        assert_eq!(builder.cursor(), 480); // Quarter note at 480 PPQ
    }

    #[test]
    fn add_sequential_timing() {
        let mut builder = MidiBuilder::new();
        let c_chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
        let g_chord = Chord::new(Pitch::from(G), Quality::Major, Number::Triad);
        let velocity = Velocity::new(100).unwrap();

        builder.add(&c_chord, Duration::Quarter, velocity);
        builder.add(&g_chord, Duration::Quarter, velocity);

        // First chord events at tick 0, second chord at tick 480
        let first_note_on = &builder.events[0];
        let second_chord_note_on = &builder.events[6];

        assert_eq!(first_note_on.tick(), 0);
        assert_eq!(second_chord_note_on.tick(), 480);
    }

    #[test]
    fn rest_advances_cursor() {
        let mut builder = MidiBuilder::new();

        builder.rest(Duration::Quarter);
        assert_eq!(builder.cursor(), 480);

        builder.rest(Duration::Half);
        assert_eq!(builder.cursor(), 1440); // 480 + 960
    }

    #[test]
    fn rest_creates_no_events() {
        let mut builder = MidiBuilder::new();
        builder.rest(Duration::Quarter);
        assert!(builder.events.is_empty());
    }

    #[test]
    fn at_beat_sets_absolute_position() {
        let mut builder = MidiBuilder::new();

        builder.at_beat(2.0);
        assert_eq!(builder.cursor(), 960); // 2 beats * 480 PPQ

        builder.at_beat(0.5);
        assert_eq!(builder.cursor(), 240); // 0.5 beats * 480 PPQ
    }

    #[test]
    fn at_beat_allows_layering() {
        let mut builder = MidiBuilder::new();
        let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
        let velocity = Velocity::new(100).unwrap();

        // Add chord at beat 0
        builder.add(&chord, Duration::Whole, velocity);
        assert_eq!(builder.cursor(), 1920);

        // Jump back to beat 2 to layer something
        builder.at_beat(2.0);
        assert_eq!(builder.cursor(), 960);
    }

    #[test]
    fn tempo_creates_event() {
        let mut builder = MidiBuilder::new();
        builder.tempo(120);

        assert_eq!(builder.events.len(), 1);
        match &builder.events[0] {
            MidiEvent::Tempo {
                tick,
                microseconds_per_beat,
            } => {
                assert_eq!(*tick, 0);
                assert_eq!(*microseconds_per_beat, 500_000); // 120 BPM
            }
            _ => panic!("Expected Tempo event"),
        }
    }

    #[test]
    fn time_signature_creates_event() {
        let mut builder = MidiBuilder::new();
        builder.time_signature(6, 8);

        assert_eq!(builder.events.len(), 1);
        match &builder.events[0] {
            MidiEvent::TimeSignature {
                tick,
                numerator,
                denominator,
            } => {
                assert_eq!(*tick, 0);
                assert_eq!(*numerator, 6);
                assert_eq!(*denominator, 8);
            }
            _ => panic!("Expected TimeSignature event"),
        }
    }

    #[test]
    fn tempo_change_mid_track() {
        let mut builder = MidiBuilder::new();
        let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
        let velocity = Velocity::new(100).unwrap();

        builder.tempo(120);
        builder.add(&chord, Duration::Whole, velocity);
        builder.tempo(140); // Tempo change after first chord

        // Find tempo events
        let tempo_events: Vec<_> = builder
            .events
            .iter()
            .filter(|e| matches!(e, MidiEvent::Tempo { .. }))
            .collect();

        assert_eq!(tempo_events.len(), 2);
        assert_eq!(tempo_events[0].tick(), 0);
        assert_eq!(tempo_events[1].tick(), 1920); // After whole note
    }
}
