//! MIDI file export combining multiple tracks.

use std::io;
use std::path::Path;

use midly::num::{u15, u24, u28, u4, u7};
use midly::MetaMessage;
use midly::{Format, Header, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind};

use crate::midi::event::MidiEvent;
use crate::midi::{Channel, MidiBuilder};

/// Combines multiple tracks into a complete MIDI file.
#[derive(Debug, Clone)]
pub struct MidiFile {
    tracks: Vec<(MidiBuilder, Channel)>,
    default_tempo: u16,
    default_time_sig: (u8, u8),
    ppq: u16,
}

impl MidiFile {
    /// Create a new MIDI file with default settings.
    ///
    /// Defaults: 120 BPM, 4/4 time, 480 PPQ.
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            default_tempo: 120,
            default_time_sig: (4, 4),
            ppq: 480,
        }
    }

    /// Set the default tempo in BPM.
    pub fn tempo(mut self, bpm: u16) -> Self {
        self.default_tempo = bpm;
        self
    }

    /// Set the default time signature.
    pub fn time_signature(mut self, numerator: u8, denominator: u8) -> Self {
        self.default_time_sig = (numerator, denominator);
        self
    }

    /// Set the PPQ (Pulses Per Quarter Note).
    pub fn ppq(mut self, ppq: u16) -> Self {
        self.ppq = ppq;
        self
    }

    /// Add a track on the specified channel.
    pub fn track(mut self, builder: MidiBuilder, channel: Channel) -> Self {
        self.tracks.push((builder, channel));
        self
    }

    /// Get the number of tracks.
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Convert to MIDI bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let smf = self.to_smf();
        let mut bytes = Vec::new();
        smf.write(&mut bytes)
            .expect("Failed to write MIDI to bytes");
        bytes
    }

    /// Save to a MIDI file.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let bytes = self.to_bytes();
        std::fs::write(path, bytes)
    }

    /// Convert to midly Smf structure.
    fn to_smf(&self) -> Smf<'static> {
        let header = Header::new(Format::Parallel, Timing::Metrical(u15::new(self.ppq)));

        let mut smf = Smf::new(header);

        // Track 0: Tempo and time signature
        let tempo_track: Track = vec![
            TrackEvent {
                delta: u28::new(0),
                kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::new(
                    MidiEvent::bpm_to_microseconds(self.default_tempo),
                ))),
            },
            TrackEvent {
                delta: u28::new(0),
                kind: TrackEventKind::Meta(MetaMessage::TimeSignature(
                    self.default_time_sig.0,
                    self.denominator_to_power(self.default_time_sig.1),
                    24, // MIDI clocks per metronome click
                    8,  // 32nd notes per quarter note
                )),
            },
            TrackEvent {
                delta: u28::new(0),
                kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
            },
        ];
        smf.tracks.push(tempo_track);

        // Add note tracks
        for (builder, channel) in &self.tracks {
            let track = self.builder_to_track(builder, *channel);
            smf.tracks.push(track);
        }

        smf
    }

    /// Convert a MidiBuilder to a midly Track.
    fn builder_to_track(&self, builder: &MidiBuilder, channel: Channel) -> Track<'static> {
        let mut track: Track = Vec::new();
        let mut events = builder.events.clone();

        // Sort events by tick
        events.sort_by_key(|e| e.tick());

        let mut last_tick = 0u32;

        for event in events {
            let delta = event.tick() - last_tick;
            last_tick = event.tick();

            let track_event = match event {
                MidiEvent::NoteOn {
                    pitch, velocity, ..
                } => TrackEvent {
                    delta: u28::new(delta),
                    kind: TrackEventKind::Midi {
                        channel: u4::new(channel.value()),
                        message: MidiMessage::NoteOn {
                            key: u7::new(pitch),
                            vel: u7::new(velocity.value()),
                        },
                    },
                },
                MidiEvent::NoteOff { pitch, .. } => TrackEvent {
                    delta: u28::new(delta),
                    kind: TrackEventKind::Midi {
                        channel: u4::new(channel.value()),
                        message: MidiMessage::NoteOff {
                            key: u7::new(pitch),
                            vel: u7::new(0),
                        },
                    },
                },
                MidiEvent::Tempo {
                    microseconds_per_beat,
                    ..
                } => TrackEvent {
                    delta: u28::new(delta),
                    kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::new(microseconds_per_beat))),
                },
                MidiEvent::TimeSignature {
                    numerator,
                    denominator,
                    ..
                } => TrackEvent {
                    delta: u28::new(delta),
                    kind: TrackEventKind::Meta(MetaMessage::TimeSignature(
                        numerator,
                        self.denominator_to_power(denominator),
                        24,
                        8,
                    )),
                },
            };
            track.push(track_event);
        }

        // End of track
        track.push(TrackEvent {
            delta: u28::new(0),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });

        track
    }

    /// Convert time signature denominator to MIDI power-of-2 format.
    /// MIDI stores denominator as power of 2 (e.g., 4 -> 2, 8 -> 3).
    fn denominator_to_power(&self, denom: u8) -> u8 {
        match denom {
            1 => 0,
            2 => 1,
            4 => 2,
            8 => 3,
            16 => 4,
            32 => 5,
            _ => 2, // Default to quarter note
        }
    }
}

impl Default for MidiFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::{Chord, Number, Quality};
    use crate::midi::builder::MidiBuilder;
    use crate::midi::{Duration, Velocity};
    use crate::note::{Pitch, PitchSymbol::*};

    #[test]
    fn new_file_defaults() {
        let file = MidiFile::new();
        assert_eq!(file.default_tempo, 120);
        assert_eq!(file.default_time_sig, (4, 4));
        assert_eq!(file.ppq, 480);
        assert_eq!(file.track_count(), 0);
    }

    #[test]
    fn builder_pattern() {
        let file = MidiFile::new().tempo(90).time_signature(6, 8).ppq(960);

        assert_eq!(file.default_tempo, 90);
        assert_eq!(file.default_time_sig, (6, 8));
        assert_eq!(file.ppq, 960);
    }

    #[test]
    fn add_tracks() {
        let track1 = MidiBuilder::new();
        let track2 = MidiBuilder::new();

        let file = MidiFile::new()
            .track(track1, Channel::new(0).unwrap())
            .track(track2, Channel::new(1).unwrap());

        assert_eq!(file.track_count(), 2);
    }

    #[test]
    fn to_bytes_creates_valid_midi() {
        let mut track = MidiBuilder::new();
        let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
        track.add(&chord, Duration::Quarter, Velocity::new(100).unwrap());

        let file = MidiFile::new().track(track, Channel::new(0).unwrap());

        let bytes = file.to_bytes();

        // MIDI files start with "MThd"
        assert_eq!(&bytes[0..4], b"MThd");
    }

    #[test]
    fn to_bytes_includes_all_tracks() {
        let track1 = MidiBuilder::new();
        let track2 = MidiBuilder::new();

        let file = MidiFile::new()
            .track(track1, Channel::new(0).unwrap())
            .track(track2, Channel::new(1).unwrap());

        let bytes = file.to_bytes();

        // Count "MTrk" occurrences (track headers)
        // Should be 3: 1 tempo track + 2 note tracks
        let mtrk_count = bytes.windows(4).filter(|w| w == b"MTrk").count();
        assert_eq!(mtrk_count, 3);
    }
}
