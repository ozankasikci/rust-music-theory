## Rust Music Theory

[![Build Status](https://travis-ci.com/ozankasikci/rust-music-theory.svg?branch=master)](https://travis-ci.com/ozankasikci/rust-music-theory)
[![Coverage Status](https://coveralls.io/repos/github/ozankasikci/rust-music-theory/badge.svg?branch=master)](https://coveralls.io/github/ozankasikci/rust-music-theory?branch=master)
[![Crates.io](https://img.shields.io/crates/v/rust-music-theory.svg?style=flat-square)](https://crates.io/crates/rust-music-theory)
[![Documentation](https://docs.rs/rust-music-theory/badge.svg)](https://docs.rs/rust-music-theory)

A library and executable that provides programmatic implementation of the basis of the music theory.
## Table of Contents

- [Overview](#overview)
- [Usage as a Library](#usage-as-a-library)
- [Lead-Sheet Chord Symbols](#lead-sheet-chord-symbols)
- [MIDI Support](#midi-support)
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
rust-music-theory = "0.5"
```

After installing the dependencies, you can use the library as follows.
```rust
extern crate rust_music_theory as rustmt;
use rustmt::note::{Note, Notes, Pitch, PitchSymbol::*};
use rustmt::scale::{Scale, ScaleType, Mode, Direction};
use rustmt::chord::{Chord, Number as ChordNumber, Quality as ChordQuality};

// to create a Note, specify a pitch and an octave;
let note = Note::new(Pitch::from(As), 4);

// Scale Example;
let scale = Scale::new(
    ScaleType::Diatonic,    // scale type
    Pitch::from(C),         // tonic
    4,                      // octave
    Some(Mode::Ionian),     // scale mode
    Direction::Ascending,   // scale direction
).unwrap();

// returns a Vector of the Notes of the scale
let scale_notes = scale.notes();

// Chord Example;
let chord = Chord::new(Pitch::from(C), ChordQuality::Major, ChordNumber::Triad);

// returns a Vector of the Notes of the chord
let chord_notes = chord.notes();

```

This is the simplest form of the usage. For detailed examples, please see the tests folder.

## Lead-Sheet Chord Symbols

Version 0.5 adds a normalized lead-sheet chord model and a compact parser. Parsing preserves
generic note spelling, resolves modifiers in musical order, and formats every accepted alias as
portable ASCII.

```rust
use rust_music_theory::chord::Chord;
use rust_music_theory::note::Notes;

let chord = Chord::parse("Cmaj9#11/G")?;
assert_eq!(chord.canonical_symbol(), "Cmaj9#11/G");
assert_eq!(
    chord.notes().iter().map(|note| note.pitch.to_string()).collect::<Vec<_>>(),
    ["G", "B", "D", "F#", "C", "E"]
);

// FromStr and Display use the same normalized grammar.
let half_diminished: Chord = "Cø7".parse()?;
assert_eq!(half_diminished.to_string(), "Cm7b5");
# Ok::<(), rust_music_theory::chord::ChordError>(())
```

Common supported forms include:

- Families: `C`, `Cm`, `Cdim`, `Caug`, `C5`, `C6`, `Cm6`, `C6/9`, `Cm6/9`
- Sevenths: `C7`, `Cmaj7`, `Cm7`, `CmMaj7`, `Cm7b5`, `Cdim7`, `Caug7`, `CaugMaj7`
- Extensions: dominant, major, minor, and minor-major `9`, `11`, and `13`
- Suspensions and additions: `C7sus4`, `Cm7sus4`, `Csus2`, `Cadd2`, `Cadd4`, `Cadd6`, `Cadd9`, `Cadd11`, `Cadd13`
- Alterations and omissions: `C7b5`, `C7#9`, `Cmaj9#11`, `C7no5`, `C13omit11`
- Altered and slash chords: `C7alt`, `C/E`, `C/F#`, `C/1`
- Aliases and groups: `CΔ`, `CΔ9`, `CmΔ`, `C^7`, `CM7`, `Cma7`, `C-7`, `Cmi7`, `Cm/M7`, `C°7`, `Cø7`, `Ch7`, `C+7`, `C69`, `C6add9`, `C7(b9,#11)`

ASCII and Unicode single/double accidentals are accepted on roots and slash basses. Generated note
octaves follow written scientific pitch, so `Cb4` plays as MIDI 59 and `B#4` as MIDI 72.

`Chord::builder(root)` exposes the same validated model for programmatic construction. See
[the chord-symbol reference](docs/chord-symbols.md) for grammar, canonicalization, builder usage,
errors, and the 0.4 to 0.5 migration.

## MIDI Support

The library supports MIDI file export and real-time MIDI playback to hardware/software synthesizers.

### MIDI File Export

Enable the `midi` feature to export chords and scales to MIDI files:

```toml
[dependencies]
rust-music-theory = { version = "0.5", features = ["midi"] }
```

```rust
use rust_music_theory::chord::{Chord, Quality, Number};
use rust_music_theory::note::{Pitch, PitchSymbol::*};
use rust_music_theory::midi::{MidiBuilder, MidiFile, Duration, Velocity, Channel};

// Quick export
let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
chord.to_midi(Duration::Quarter, Velocity::new(100).unwrap())
    .save("chord.mid")?;

// Multi-track composition
let mut builder = MidiBuilder::new();
builder
    .tempo(120)
    .add(&chord, Duration::Whole, Velocity::new(90).unwrap());

MidiFile::new()
    .tempo(120)
    .track(builder, Channel::new(0).unwrap())
    .save("song.mid")?;
```

### Real-Time MIDI Playback

Enable the `midi-playback` feature to play notes on connected MIDI devices (hardware synths, DAWs like Ableton):

```toml
[dependencies]
rust-music-theory = { version = "0.5", features = ["midi-playback"] }
```

```rust
use rust_music_theory::chord::{Chord, Quality, Number};
use rust_music_theory::note::{Pitch, PitchSymbol::*};
use rust_music_theory::midi::playback::{MidiPorts, MidiPlayer};
use rust_music_theory::midi::{Duration, Velocity};

// List available MIDI ports
let ports = MidiPorts::list()?;
for (i, name) in ports.iter().enumerate() {
    println!("{}: {}", i, name);
}

// Connect and play
let mut player = MidiPlayer::connect_index(0)?;
player.set_tempo(120);

let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
player.play(&chord, Duration::Quarter, Velocity::new(100).unwrap());

// Control Change (filter, modulation, etc.)
player.control_change(1, 64);   // Modulation wheel
player.control_change(74, 100); // Filter cutoff

// Program Change (switch instruments)
player.program_change(0);                    // Piano
player.program_change_with_bank(5, 0, 1);   // Bank 1, program 5

// MIDI Clock (sync with DAW)
player.start_clock();  // Sends clock at 24 PPQ
// ... play notes ...
player.stop_clock();
```

#### Using with Ableton Live

1. **macOS**: Enable IAC Driver in Audio MIDI Setup
2. In Ableton: Preferences → Link, Tempo & MIDI → Enable Track for IAC Driver
3. Create a MIDI track, set input to IAC Driver
4. Run your Rust code - notes play through Ableton's instruments!

For MIDI clock sync, enable "Ext" sync in Ableton's transport.

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
  2: E#
  3: G#
  4: B
  5: D#
  6: F#
```

Compact symbols and canonical normalization are also available:

```console
$ rustmt chord C7sus4
Notes:
  1: C
  2: F
  3: G
  4: Bb

$ rustmt chord normalize 'C7(b9,#11)'
C7b9#11
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
Supported chord syntax:
 - Major Triad: C
 - Sevenths: C7, Cmaj7, Cm7, CmMaj7, Cm7b5, Cdim7, Caug7, CaugMaj7
 - Altered dominant: C7alt
 - Slash basses and inversions: C/E, C/F#, C/1
 ...
```

## Interactive Playground

Try the library in your browser with the interactive WASM playground:

[**https://ozankasikci.github.io/rust-music-theory/**](https://ozankasikci.github.io/rust-music-theory/)

![Playground Screenshot](docs/playground-screenshot.png)

## Building From Source

The binary returns the notes of the requested scale or chord. Chords use the normalized
Unicode-aware tokenizer; the older regex-named chord entry point is retained only as a deprecated
compatibility wrapper.
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
- [x] MIDI file export
- [x] Real-time MIDI playback
- [x] MIDI Control Change & Program Change
- [x] MIDI Clock (master mode)
- [x] Properly display enharmonic spelling
- [x] Add inversion support for chords
- [ ] Add missing modes for Melodic & Harmonic minor scales
- [ ] Add support for arbitrary accidentals
- [ ] Add a mechanism to find the chord from the given notes
- [ ] MIDI input (receive from external devices)
