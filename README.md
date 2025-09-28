## Rust Music Theory

[![Build Status](https://travis-ci.com/ozankasikci/rust-music-theory.svg?branch=master)](https://travis-ci.com/ozankasikci/rust-music-theory)
[![Coverage Status](https://coveralls.io/repos/github/ozankasikci/rust-music-theory/badge.svg?branch=master)](https://coveralls.io/github/ozankasikci/rust-music-theory?branch=master)
[![Crates.io](https://img.shields.io/crates/v/rust-music-theory.svg?style=flat-square)](https://crates.io/crates/rust-music-theory)
[![Documentation](https://docs.rs/rust-music-theory/badge.svg)](https://docs.rs/rust-music-theory)

A library and executable that provides programmatic implementation of the basis of the music theory.
## Table of Contents

- [Overview](#overview)
- [Usage as a Library](#usage-as-a-library)
- [Usage as an Executable](#usage-as-an-executable)
- [Interactive Playground](#interactive-playground)
- [Building From Source](#building-from-source)
- [Roadmap](#roadmap)

## Overview

`Rust Music Theory` is used to procedurally utilize music theory notions like Note, Chord, Scale,
Interval and more. The main purpose of this library is to let music theory be used in other programs and produce music/audio in a programmatic way.

## Usage as a Library
Add `rust-music-theory` as a dependency in your Cargo.toml.
```toml
[dependencies]
rust-music-theory = "0.2"
```

After installing the dependencies, you can use the library as follows.
```rust
extern crate rust_music_theory as rustmt;
use rustmt::note::{Note, Notes, PitchClass};
use rustmt::scale::{Scale, ScaleType, Mode, Direction};
use rustmt::chord::{Chord, Number as ChordNumber, Quality as ChordQuality};

// to create a Note, specify a pitch class and an octave;
let note = Note::new(PitchClass::As, 4);

// Scale Example;
let scale = Scale::new(
    ScaleType::Diatonic,    // scale type
    PitchClass::C,          // tonic
    4,                      // octave
    Some(Mode::Ionian),     // scale mode
    Direction::Ascending,   // scale direction
).unwrap();

// returns a Vector of the Notes of the scale
let scale_notes = scale.notes();

// Chord Example;
let chord = Chord::new(PitchClass::C, ChordQuality::Major, ChordNumber::Triad);

// returns a Vector of the Notes of the chord
let chord_notes = chord.notes();

```

This is the simplest form of the usage. For detailed examples, please see the tests folder.

## Usage as an Executable

`cargo install --git https://github.com/ozankasikci/rust-music-theory`

This lets cargo install the library as an executable called `rustmt`. Some usage examples;

`rustmt scale D Locrian`
```yaml
Notes:
  1: D
  2: D#
  3: F
  4: G
  5: G#
  6: A#
  7: C
  8: D
```
`rustmt chord C# Dominant Eleventh`
```yaml
Notes:
  1: C#
  2: F
  3: G#
  4: B
  5: D#
  6: G
```

`rustmt scale list`
```yaml
Available Scales:
 - Major|Ionian
 - Minor|Aeolian
 - Dorian
 - Phrygian
 - Lydian
 - Mixolydian
 - Locrian
 - Harmonic Minor
 - Melodic Minor
```


`rustmt chord list`
```yaml
Available chords:
 - Major Triad
 - Minor Triad
 - Suspended2 Triad
 - Suspended4 Triad
 - Augmented Triad
 - Diminished Triad
 - Major Seventh
 - Minor Seventh
 - Augmented Seventh
 - Augmented Major Seventh
 - Diminished Seventh
 - Half Diminished Seventh
 - Minor Major Seventh
 - Dominant Seventh
 - Dominant Ninth
 - Major Ninth
 - Dominant Eleventh
 - Major Eleventh
 - Minor Eleventh
 - Dominant Thirteenth
 - Major Thirteenth
 - Minor Thirteenth
```

## Interactive Playground

Try the library in your browser with the interactive WASM playground:

[**https://ozankasikci.github.io/rust-music-theory/**](https://ozankasikci.github.io/rust-music-theory/)

The playground allows you to experiment with scales, chords, and other music theory concepts directly in your browser without any installation.

## Building From Source

The binary is implemented as a regex parser cli that returns the notes of the given scale/chord.
To quickly build and run the executable locally;

`git clone http://github.com/ozankasikci/rust-music-theory && cd rust-music-theory`

Then you can directly compile using cargo. An example;

`cargo run scale D Locrian`
```yaml
Notes:
  1: D
  2: D#
  3: F
  4: G
  5: G#
  6: A#
  7: C
  8: D
```

[1]: https://en.wikipedia.org/wiki/Cadence
## Roadmap
- [ ] Add missing modes for Melodic & Harmonic minor scales
    - [ ] Add support for arbitrary accidentals
    - [ ] Add support for the alternative names of the modes to regex parser
- [x] Properly display enharmonic spelling
- [x] Add inversion support for chords
- [ ] Add a mechanism to find the chord from the given notes
