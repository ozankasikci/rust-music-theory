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
//! extern crate rust_music_theory as rmt;
//! use rmt::note::{Note, Notes, PitchClass, Accidental};
//! use rmt::scale::{Scale, ScaleType, Mode};
//! use rmt::chord::{Chord, Number as ChordNumber, Quality as ChordQuality};
//!
//! // to create a Note, specify a pitch class and an octave;
//! let note = Note::new(PitchClass::As, 4);
//! // Note { pitch_class: A, octave: 4 }
//!
//! // Scale Example;
//! let scale = Scale::new(
//!     ScaleType::Diatonic,    // scale type
//!     PitchClass::C,          // tonic
//!     4,                      // octave
//!     Some(Mode::Ionian),     // scale mode
//!     Some(vec![(Accidental::Sharp, 6)]) // accidentals that are out of the scale
//! ).unwrap();
//!
//! // returns a Vector of the Notes of the scale
//! let scale_notes = scale.notes();
//!
//! // Chord Example;
//! let chord = Chord::new(PitchClass::C, ChordQuality::Major, ChordNumber::Triad);
//!
//! // returns a Vector of the Notes of the chord
//! let chord_notes = chord.notes();
//!

extern crate strum;
pub mod chord;
pub mod interval;
pub mod note;
pub mod scale;
