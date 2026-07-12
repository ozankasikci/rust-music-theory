//! MIDI export functionality for rust-music-theory.
//!
//! This module provides MIDI file export capabilities, transforming music theory
//! constructs like Chords and Scales into playable MIDI files.
//!
//! # Feature Flag
//!
//! Enable with the `midi` feature flag:
//! ```toml
//! rust-music-theory = { version = "0.4", features = ["midi"] }
//! ```
//!
//! # Quick Export
//!
//! The simplest way to export is using the [`ToMidi`] trait:
//!
//! ```ignore
//! use rust_music_theory::chord::{Chord, Quality, Number};
//! use rust_music_theory::note::{Pitch, PitchSymbol::*};
//! use rust_music_theory::midi::{ToMidi, Duration, Velocity};
//!
//! let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
//! chord.to_midi(Duration::Quarter, Velocity::new(100).unwrap())
//!     .save("chord.mid")?;
//! ```
//!
//! # Programmatic Composition
//!
//! For more control, use [`MidiBuilder`] and [`MidiFile`]:
//!
//! ```ignore
//! use rust_music_theory::chord::{Chord, Quality, Number};
//! use rust_music_theory::note::{Pitch, PitchSymbol::*};
//! use rust_music_theory::midi::{MidiBuilder, MidiFile, Duration, Velocity, Channel};
//!
//! // Build a chord progression
//! let mut chords = MidiBuilder::new();
//! chords
//!     .add(&Chord::new(Pitch::from(C), Quality::Major, Number::Triad),
//!          Duration::Whole, Velocity::new(90).unwrap())
//!     .add(&Chord::new(Pitch::from(G), Quality::Major, Number::Triad),
//!          Duration::Whole, Velocity::new(90).unwrap());
//!
//! // Build a melody on a separate track
//! let mut melody = MidiBuilder::new();
//! melody
//!     .at_beat(2.0)  // Start at beat 2
//!     .add(&some_notes, Duration::Eighth, Velocity::new(100).unwrap());
//!
//! // Combine and export
//! MidiFile::new()
//!     .tempo(120)
//!     .track(chords, Channel::new(0).unwrap())
//!     .track(melody, Channel::new(1).unwrap())
//!     .save("song.mid")?;
//! ```
//!
//! # Real-Time Playback (optional feature)
//!
//! With the `midi-playback` feature, you can play notes on connected MIDI devices:
//!
//! ```toml
//! rust-music-theory = { version = "0.4", features = ["midi-playback"] }
//! ```
//!
//! ```ignore
//! use rust_music_theory::midi::{MidiPorts, MidiPlayer, Duration, Velocity};
//!
//! let ports = MidiPorts::list()?;
//! let mut player = MidiPlayer::connect_index(0)?;
//! player.set_tempo(120);
//! player.play(&chord, Duration::Quarter, Velocity::new(100).unwrap());
//! ```

mod builder;
mod duration;
pub(crate) mod event;
mod export;
mod file;
mod types;

#[cfg(feature = "midi-playback")]
pub mod playback;

pub use builder::{MidiBuilder, DEFAULT_PPQ};
pub use duration::Duration;
pub use export::{MidiExport, ToMidi};
pub use file::MidiFile;
pub use types::{Channel, Velocity};

#[cfg(feature = "midi-playback")]
pub use playback::{PlaybackError, MidiPorts, MidiPlayer};
