//! `Rust Music Theory` is a library that provides programmatic implementation of the basis of music theory.
//!
//! ## About
//!
//! `Rust Music Theory` is used to procedurally utilize music theory notions like Note, Chord, Scale,
//! Interval, Key and more. All these theoretical concepts of sound and music are implemented as
//! Separate modules in the library. They sometimes can be used individually, and sometimes need
//! to be used together to correctly reflect the music theory to the code.
//!
//! ## Quick Example
//!
//! The following examples are the simplest usage of the library.
//! Please see the tests folder for detailed examples of the modules.
//!
//! ```no_run
//! extern crate rust_music_theory as rustmt;
//! use rustmt::note::{Note, Notes, Pitch, PitchSymbol::*};
//! use rustmt::scale::{Direction, Scale, ScaleType, Mode};
//! use rustmt::chord::{Chord, Number as ChordNumber, Quality as ChordQuality};
//!
//! // to create a Note, specify a pitch class and an octave;
//! let note = Note::new(Pitch::from(As), 4);
//! // Note { Pitch::new(NoteLetter::A, 1), octave: 4 }
//!
//! // Scale Example;
//! let scale = Scale::new(
//!     ScaleType::Diatonic,    // scale type
//!     Pitch::from(C),                      // tonic
//!     4,                      // octave
//!     Some(Mode::Ionian),     // scale mode
//!     Direction::Ascending,   // direction
//! ).unwrap();
//!
//! // returns a Vector of the Notes of the scale
//! let scale_notes = scale.notes();
//!
//! // Chord Example;
//! let chord = Chord::new(Pitch::from(C), ChordQuality::Major, ChordNumber::Triad);
//!
//! // returns a Vector of the Notes of the chord
//! let chord_notes = chord.notes();
//! ```
//!
//! ## MIDI Export (optional feature)
//!
//! With the `midi` feature enabled, you can export chords and scales to MIDI files:
//!
//! ```toml
//! rust-music-theory = { version = "0.3", features = ["midi"] }
//! ```
//!
//! ```ignore
//! use rustmt::midi::{ToMidi, Duration, Velocity};
//!
//! chord.to_midi(Duration::Quarter, Velocity::new(100).unwrap())
//!     .save("chord.mid")?;
//! ```
//!
//! ## Real-Time Playback (optional feature)
//!
//! With the `midi-playback` feature, play to connected MIDI instruments:
//!
//! ```toml
//! rust-music-theory = { version = "0.3", features = ["midi-playback"] }
//! ```
//!
//! ```ignore
//! use rustmt::midi::{MidiPorts, MidiPlayer};
//!
//! let ports = MidiPorts::list()?;
//! let mut player = MidiPlayer::connect_index(0)?;
//! player.play(&chord, Duration::Quarter, Velocity::new(100).unwrap());
//! ```

pub extern crate strum;
pub mod chord;
pub mod interval;
pub mod note;
pub mod scale;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(feature = "midi")]
pub mod midi;
