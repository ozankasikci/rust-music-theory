# Lead-Sheet Chord Symbols

Rust Music Theory 0.5 represents a chord as a validated `ChordSpec` plus a resolved
`ChordFormula`. The parser, builder, note generator, CLI, and WASM interface all consume this same
model, so displayed symbols cannot disagree with playback notes.

## Parsing and canonical formatting

```rust
use rust_music_theory::chord::Chord;

let chord = Chord::parse("C7(b9,#11)")?;
assert_eq!(chord.canonical_symbol(), "C7b9#11");

let alias: Chord = "CΔ9".parse()?;
assert_eq!(alias.to_string(), "Cmaj9");
# Ok::<(), rust_music_theory::chord::ChordError>(())
```

Canonical output is ASCII. Input-only aliases include `Δ`, `^`, `M`, `ma`, `-`, `mi`, `min`, `°`,
`o`, `ø`, `h`, `+`, `69`, `6add9`, minor-major forms such as `m/M7`, English
quality/extension names, `sus` for `sus4`, parentheses,
and comma-separated modifiers.

Bare `Δ` means major seventh and `mΔ` means minor-major seventh; numbered forms retain their
extension (`Δ9`, `Δ13`). Unicode `♭`, `♯`, `𝄫`, and `𝄪` are input aliases and format back to
portable ASCII accidentals.

The parser resolves a symbol in this order:

1. base triad and extension;
2. suspension, replacing the third;
3. alterations, replacing a present degree or adding an absent one;
4. additions;
5. omissions.

`sus4add3` explicitly restores the third. Duplicate, contradictory, and redundant modifiers are
errors. `7alt` is fixed to `1 3 b5 #5 b7 b9 #9` and cannot take additional modifiers.
Error byte positions always refer to the original UTF-8 input, before alias and whitespace
normalization.

Modifier formatting is deterministic: suspension, alterations by degree, additions by degree,
omissions by degree, then slash bass. For example, `C7(no5,add13,#11,b9)` becomes
`C7b9#11add13no5`.

## Supported families

| Family | Examples |
| --- | --- |
| Triads | `C`, `Cm`, `Cdim`, `Caug`, `C5` |
| Sixths | `C6`, `Cm6`, `C6/9`, `Cm6/9` |
| Sevenths | `C7`, `Cmaj7`, `Cm7`, `CmMaj7`, `Cm7b5`, `Cdim7`, `Caug7`, `CaugMaj7` |
| Extensions | dominant, major, minor, and minor-major `9`, `11`, `13` |
| Suspensions | `Csus2`, `Csus4`, `C7sus4`, `Cm7sus4` |
| Added tones | `add2`, `add4`, `add6`, `add9`, `add11`, `add13` |
| Alterations | flat/sharp `5`, `9`, `11`, `13` |
| Omissions | `no3`, `no5`, `no7`, `no9`, `no11`, `no13`; `omit` is an alias |
| Altered | `C7alt` |
| Slash bass | chord-tone inversions and non-chord basses, plus numeric inversions such as `C/1` |

The root and every generated chord tone retain a generic letter. This produces theoretical but
correct spellings such as `C#maj9#11` = `C# E# G# B# D# F##` and
`Cb7b9` = `Cb Eb Gb Bbb Dbb`.

Octaves follow the written step, alteration, and octave independently. Consequently `Cb4` has MIDI
pitch 59 while `B#4` has MIDI pitch 72, and compound additions retain their written octave.

Core formulas follow MusicXML's chord-kind definitions. Extensions are complete structural
formulas rather than guitar or keyboard voicing suggestions: an eleventh includes its ninth, and
a thirteenth includes its ninth and eleventh. Simple and compound additions remain distinct in
the generated octave (`add2` versus `add9`, `add6` versus `add13`). The accepted shorthand aliases
are informed by Tonal's chord dictionary and ChordPro's built-in extension registry. This crate's
`7alt` formula is deliberately fixed to `1 3 b5 #5 b7 b9 #9`; it is a documented library contract,
not an attempt to preserve every external library's different `alt` voicing.

References: [MusicXML chord kinds](https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/kind-value/),
[MusicXML degree operations](https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/degree/),
[MusicXML MIDI-compatible pitch](https://www.w3.org/2021/06/musicxml40/tutorial/midi-compatible-part/),
[Tonal chord dictionary](https://github.com/tonaljs/tonal/blob/main/packages/chord-type/data.ts),
and [ChordPro chord parsing](https://www.chordpro.org/chordpro/chordpro-chords/).

## Builder and normalized types

```rust
use rust_music_theory::chord::{Chord, ChordExtension, SeventhQuality};
use rust_music_theory::note::{NoteLetter, Pitch};

let chord = Chord::builder(Pitch::new(NoteLetter::C, 0))
    .extension(ChordExtension::Ninth)
    .seventh_quality(SeventhQuality::Major)
    .alter(11, 1)?
    .bass(Pitch::new(NoteLetter::G, 0))
    .build()?;

assert_eq!(chord.to_string(), "Cmaj9#11/G");
# Ok::<(), rust_music_theory::chord::ChordError>(())
```

The public normalized types are `TriadQuality`, `SeventhQuality`, `ChordExtension`, `Suspension`,
`ChordTone`, `ChordModifier`, `ChordSpec`, and `ChordFormula`. Read them through `Chord::spec()` and
`Chord::formula()`.

## Slash bass behavior

A chord-member bass becomes an inversion. A non-chord bass is inserted below the chord. Canonical
formatting always writes the actual note: `C/1` becomes `C/E`.

## Migration from 0.4

The legacy constructors remain available:

```rust
Chord::new(root, quality, number);
Chord::try_new(root, quality, number);
Chord::with_inversion(root, quality, number, inversion);
```

`Quality` and `Number` now adapt to a validated `ChordSpec`; `Power`, `Fifth`, `Sixth`, and
`SixNine` cover the added core families. `Chord::from_regex` is deprecated and delegates to
`Chord::parse`.

Chord fields are private in 0.5. Replace direct access as follows:

| 0.4 | 0.5 |
| --- | --- |
| `chord.root` | `chord.root()` |
| `chord.octave` | `chord.octave()` |
| `chord.intervals` | `chord.intervals()` |
| `chord.quality` | `chord.quality()` |
| `chord.number` | `chord.number()` |
| `chord.inversion` | `chord.inversion()` |
| `chord.bass` | `chord.bass()` |

Use `Chord::builder` instead of a struct literal. This prevents interval, symbol, inversion, and
bass metadata from diverging.

## CLI and WASM

The CLI accepts compact symbols directly and has a normalization command:

```console
rustmt chord Cmaj9#11/G
rustmt chord normalize Cø7
rustmt chord list
```

WASM callers can use `parse_chord_symbol(symbol)`. It returns a serialized `WasmChord` with
`canonical_symbol`, `bass`, `modifiers`, `formula`, and notes, or throws the Rust parse error. The
legacy `generate_chord(root, quality, number)` function remains available.
