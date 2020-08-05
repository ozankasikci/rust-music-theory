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
//! use rustmt::note::{Note, Notes, PitchClass, PitchSymbol::*, pclass};
//! use rustmt::scale::{Scale, ScaleType, Mode};
//! use rustmt::chord::{Chord, Number as ChordNumber, Quality as ChordQuality};
//!
//! // to create a Note, specify a pitch class and an octave;
//! let note = Note::new(pclass(A,1), 4);
//! // Note { pclass(A,1), octave: 4 }
//!
//! // Scale Example;
//! let scale = Scale::new(
//!     ScaleType::Diatonic,    // scale type
//!     pclass(C,0),            // tonic
//!     4,                      // octave
//!     Some(Mode::Ionian),     // scale mode
//! ).unwrap();
//!
//! // returns a Vector of the Notes of the scale
//! let scale_notes = scale.notes();
//!
//! // Chord Example;
//! let chord = Chord::new(pclass(C,0), ChordQuality::Major, ChordNumber::Triad);
//!
//! // returns a Vector of the Notes of the chord
//! let chord_notes = chord.notes();

extern crate strum;
pub mod chord;
pub mod interval;
pub mod note;
pub mod scale;
