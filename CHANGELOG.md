## Unreleased

### Features

- Add every harmonic-minor and melodic-minor mode with canonical names, established aliases,
  validated scale-family metadata, CLI generation, and WASM identifiers

### Fixes

- Preserve classical melodic-minor descent while derived melodic-minor modes use the jazz pitch
  collection in both directions
- Synchronize scale octaves after theoretical respelling so augmented steps remain strictly ordered

## v0.5.0 - 2026-07-12

### Features

- Add complete common jazz/pop lead-sheet chord symbols with aliases, modifiers, slash basses,
  canonical ASCII formatting, and exact theoretical note spelling
- Add normalized `ChordSpec`/`ChordFormula` theory types, `Chord::parse`, `FromStr`, `Display`,
  getters, and `Chord::builder`
- Add compact chord support and `chord normalize` to the CLI
- Add `parse_chord_symbol` and normalized chord metadata to WASM

### Fixes

- Preserve power-chord and minor-major quality markers instead of silently reinterpreting invalid
  combinations
- Correct scientific octaves and MIDI pitches for theoretical notes such as C-flat and B-sharp
- Accept Unicode double-flat roots and basses and canonicalize all accidentals to ASCII
- Report parser error positions against the original input after aliases, Unicode, and whitespace
  normalization
- Keep source-position tracking linear for long or adversarial invalid symbols

### Breaking Changes

- Make `Chord` fields private so symbol metadata and playback formulas remain consistent; legacy
  constructors and quality/number adapters remain available

## v0.4.0 - 2026-07-12

### Features
- Add MIDI file export for notes, chords, scales, tracks, tempo, and time signatures
- Add optional real-time MIDI playback, port discovery, program changes, control changes, and MIDI clock
- Add Minor Ninth chord support and conventional compact chord symbols such as `C7`, `Cmaj7`, and `Cm7`
- Add fallible chord construction APIs and runnable Node/WASM tests

### Fixes
- Preserve written interval and chord spelling, including flats and double accidentals
- Correct chord extensions, slash bass notes, invalid-input handling, and inversion validation
- Correct classical melodic-minor descent, blues and chromatic spelling, and modal key signatures
- Prevent octave underflow during descending transposition
- Mark MIDI playback examples with their required Cargo feature

### Breaking Changes
- Change public note, chord, scale, and WASM octave values from `u8` to `i16`
- Reject unsupported chord quality and number combinations instead of silently producing a major triad

## v0.2.0 - 2020-08-19

### Features
- Add support for inversion of intervals (by @henryksloan)
- Add support for descending scales (by @henryksloan)
- Add support for descending a note by a given interval (by @henryksloan)

### Breaking Changes
- `Scale::new` method now expects an additional Direction argument

## v0.1.7 - 2020-05-03

### Improvements
- Small performance improvements
- Fix Clippy lint errors

## v0.1.6 - 2020-01-26

### Features
- Add support for Note, Interval, Chord, Scale
- Add a cli binary to parse the given chord/scale string using regex
- Raise test coverage to 75 percent
- Add initial documentation for crates.io
