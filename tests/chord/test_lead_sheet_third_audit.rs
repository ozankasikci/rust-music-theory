extern crate rust_music_theory as theory;

use std::collections::HashMap;
use std::panic;
use std::time::{Duration, Instant};

use theory::chord::{
    Chord, ChordBuilder, ChordError, ChordExtension, ChordSpec, SeventhQuality, Suspension,
    TriadQuality,
};
use theory::note::{Note, NoteLetter, Notes, Pitch};

fn configure_builder(
    root: Pitch,
    triad: TriadQuality,
    seventh: Option<SeventhQuality>,
    extension: ChordExtension,
    suspension: Option<Suspension>,
) -> ChordBuilder {
    let mut builder = Chord::builder(root)
        .triad_quality(triad)
        .extension(extension);
    if let Some(seventh) = seventh {
        builder = builder.seventh_quality(seventh);
    }
    if let Some(suspension) = suspension {
        builder = builder.suspension(suspension);
    }
    builder
}

fn scientific_semitones(note: &Note) -> i32 {
    let natural = match note.pitch.letter {
        NoteLetter::C => 0,
        NoteLetter::D => 2,
        NoteLetter::E => 4,
        NoteLetter::F => 5,
        NoteLetter::G => 7,
        NoteLetter::A => 9,
        NoteLetter::B => 11,
    };
    note.octave as i32 * 12 + natural + note.pitch.accidental as i32
}

#[derive(Clone, Copy)]
enum Operation {
    Add(u8, i8),
    Alter(u8, i8),
    Omit(u8),
    Altered,
}

fn apply(builder: ChordBuilder, operation: Operation) -> Result<ChordBuilder, ChordError> {
    match operation {
        Operation::Add(degree, alteration) => builder.add(degree, alteration),
        Operation::Alter(degree, alteration) => builder.alter(degree, alteration),
        Operation::Omit(degree) => Ok(builder.omit(degree)),
        Operation::Altered => Ok(builder.altered()),
    }
}

fn register_canonical(symbols: &mut HashMap<String, ChordSpec>, chord: Chord) {
    let canonical = chord.canonical_symbol();
    if let Some(previous) = symbols.insert(canonical.clone(), chord.spec().clone()) {
        assert_eq!(
            previous,
            *chord.spec(),
            "canonical collision: {}",
            canonical
        );
    }
    let reparsed = Chord::parse(&canonical).unwrap();
    assert_eq!(reparsed.spec(), chord.spec(), "{}", canonical);
    assert_eq!(reparsed.formula(), chord.formula(), "{}", canonical);
    assert_eq!(reparsed.notes(), chord.notes(), "{}", canonical);
}

#[test]
fn every_valid_two_modifier_builder_has_a_unique_round_trippable_symbol() {
    let root = Pitch::new(NoteLetter::C, -1);
    let operations = [
        Operation::Add(2, 0),
        Operation::Add(3, 0),
        Operation::Add(9, -1),
        Operation::Add(13, 1),
        Operation::Alter(5, -1),
        Operation::Alter(5, 1),
        Operation::Alter(9, -1),
        Operation::Alter(11, 1),
        Operation::Alter(13, -1),
        Operation::Omit(3),
        Operation::Omit(5),
        Operation::Omit(7),
        Operation::Omit(9),
        Operation::Omit(11),
        Operation::Altered,
    ];
    let mut symbols = HashMap::new();

    for triad in [
        TriadQuality::Major,
        TriadQuality::Minor,
        TriadQuality::Diminished,
        TriadQuality::Augmented,
        TriadQuality::Power,
    ] {
        for seventh in [
            None,
            Some(SeventhQuality::Major),
            Some(SeventhQuality::Minor),
            Some(SeventhQuality::Diminished),
        ] {
            for extension in [
                ChordExtension::Triad,
                ChordExtension::Sixth,
                ChordExtension::SixNine,
                ChordExtension::Seventh,
                ChordExtension::Ninth,
                ChordExtension::Eleventh,
                ChordExtension::Thirteenth,
            ] {
                for suspension in [None, Some(Suspension::Second), Some(Suspension::Fourth)] {
                    let base = configure_builder(root, triad, seventh, extension, suspension);
                    if let Ok(chord) = base.clone().build() {
                        register_canonical(&mut symbols, chord);
                    }
                    for first in operations {
                        for second in operations {
                            let first_builder = match apply(base.clone(), first) {
                                Ok(builder) => builder,
                                Err(_) => continue,
                            };
                            let second_builder = match apply(first_builder, second) {
                                Ok(builder) => builder,
                                Err(_) => continue,
                            };
                            if let Ok(chord) = second_builder.build() {
                                register_canonical(&mut symbols, chord);
                            }
                        }
                    }
                }
            }
        }
    }

    assert!(symbols.len() > 500, "only {} unique symbols", symbols.len());
}

#[test]
fn every_common_root_and_bass_spelling_round_trips_with_correct_state() {
    let letters = [
        NoteLetter::C,
        NoteLetter::D,
        NoteLetter::E,
        NoteLetter::F,
        NoteLetter::G,
        NoteLetter::A,
        NoteLetter::B,
    ];
    let suffixes = [
        "", "m", "dim7", "augMaj7", "5", "6/9", "7", "maj9#11", "mMaj13", "7sus4", "add13", "7alt",
    ];

    for root_letter in letters {
        for root_alteration in -2..=2 {
            let root = Pitch::new(root_letter, root_alteration);
            for suffix in suffixes {
                let base_symbol = format!("{}{}", root, suffix);
                let base = Chord::parse(&base_symbol).unwrap();
                for bass_letter in letters {
                    for bass_alteration in -2..=2 {
                        let bass = Pitch::new(bass_letter, bass_alteration);
                        let symbol = format!("{}/{}", base_symbol, bass);
                        let chord = Chord::parse(&symbol).unwrap();
                        let member_index = base
                            .notes()
                            .iter()
                            .position(|note| note.pitch.into_u8() == bass.into_u8());
                        match member_index {
                            Some(0) => {
                                assert_eq!(chord.inversion(), 0, "{}", symbol);
                                assert_eq!(chord.bass(), None, "{}", symbol);
                            }
                            Some(index) => {
                                assert_eq!(chord.inversion(), index as u8, "{}", symbol);
                                assert_eq!(
                                    chord.bass(),
                                    Some(chord.notes()[0].pitch),
                                    "{}",
                                    symbol
                                );
                            }
                            None => {
                                assert_eq!(chord.inversion(), 0, "{}", symbol);
                                assert_eq!(chord.bass(), Some(bass), "{}", symbol);
                                assert!(
                                    scientific_semitones(&chord.notes()[0])
                                        < scientific_semitones(&chord.notes()[1]),
                                    "{}",
                                    symbol
                                );
                            }
                        }
                        let canonical = chord.canonical_symbol();
                        let reparsed = Chord::parse(&canonical).unwrap();
                        assert_eq!(reparsed.spec(), chord.spec(), "{}", symbol);
                        assert_eq!(reparsed.notes(), chord.notes(), "{}", symbol);
                    }
                }
            }
        }
    }
}

fn error_position(error: &ChordError) -> Option<usize> {
    match error {
        ChordError::InvalidRoot { position }
        | ChordError::UnexpectedToken { position, .. }
        | ChordError::InvalidModifierAt { position, .. }
        | ChordError::ConflictingModifiersAt { position, .. }
        | ChordError::InvalidSlashBass { position }
        | ChordError::UnsupportedConstruction { position, .. } => Some(*position),
        _ => None,
    }
}

#[test]
fn all_generated_error_positions_are_original_utf8_boundaries() {
    let fragments = [
        "C", "b", "#", "𝄫", "Δ", "7", "sus", "add", "no", "/", "(", ")", ",", "🎵",
    ];
    for first in fragments {
        for second in fragments {
            for third in fragments {
                for fourth in fragments {
                    let symbol = format!("{}{}{}{}", first, second, third, fourth);
                    if let Err(error) = Chord::parse(&symbol) {
                        if let Some(position) = error_position(&error) {
                            assert!(position <= symbol.len(), "{}: {:?}", symbol, error);
                            assert!(symbol.is_char_boundary(position), "{}: {:?}", symbol, error);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn adversarial_long_errors_remain_fast_and_positioned() {
    let symbol = format!("C{}", "w".repeat(16_384));
    let started = Instant::now();
    let error = Chord::parse(&symbol).unwrap_err();
    assert!(started.elapsed() < Duration::from_secs(5));
    assert!(matches!(
        error,
        ChordError::UnexpectedToken { position: 1, .. }
    ));

    let unicode = format!("C7({})", "♭9,".repeat(4_096));
    let started = Instant::now();
    let result = panic::catch_unwind(|| Chord::parse(&unicode));
    assert!(result.is_ok());
    assert!(started.elapsed() < Duration::from_secs(5));
}

#[test]
fn extreme_chord_octaves_never_panic_and_clamp_midi() {
    for octave in [i16::MIN, i16::MIN + 1, -1, 0, 9, i16::MAX - 1, i16::MAX] {
        for symbol in ["Cbmaj13", "B#7alt", "Cbbadd13/F#"] {
            let chord = Chord::parse(symbol).unwrap().with_octave(octave);
            let result = panic::catch_unwind(|| chord.notes());
            assert!(result.is_ok(), "{} octave {}", symbol, octave);
            for note in result.unwrap() {
                assert!(note.midi_pitch() <= 127);
            }
        }
    }
}
