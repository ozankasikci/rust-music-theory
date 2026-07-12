extern crate rust_music_theory as theory;

use std::panic;

use theory::chord::{
    Chord, ChordBuilder, ChordError, ChordExtension, ChordTone, SeventhQuality, Suspension,
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
    (note.octave as i32 + 1) * 12 + natural + note.pitch.accidental as i32
}

fn assert_round_trip(chord: Chord) {
    let canonical = chord.canonical_symbol();
    let reparsed = Chord::parse(&canonical)
        .unwrap_or_else(|error| panic!("{} should reparse: {}", canonical, error));
    assert_eq!(reparsed.spec(), chord.spec(), "{}", canonical);
    assert_eq!(reparsed.formula(), chord.formula(), "{}", canonical);
    assert_eq!(reparsed.notes(), chord.notes(), "{}", canonical);
}

#[test]
fn power_chord_marker_is_never_silently_discarded() {
    assert_eq!(Chord::parse("C5").unwrap().canonical_symbol(), "C5");
    assert_eq!(Chord::parse("C5/E").unwrap().canonical_symbol(), "C5/E");

    for invalid in ["C5add9", "C5no3", "C5sus2", "C5sus4", "C5b9", "C5#11"] {
        assert!(
            Chord::parse(invalid).is_err(),
            "{} should be rejected",
            invalid
        );
    }
}

#[test]
fn quality_markers_are_not_accepted_when_their_meaning_would_be_lost() {
    for invalid in [
        "CmMaj", "CmMaj5", "CmMaj6", "CmMaj6/9", "CaugMaj", "Cdom5", "Cdom6", "Cdom6/9",
    ] {
        assert!(
            Chord::parse(invalid).is_err(),
            "{} should be rejected",
            invalid
        );
    }

    for valid in ["Cmaj", "Cmaj6", "CmMaj7", "CmMaj9", "CaugMaj7", "Cdom"] {
        assert_round_trip(Chord::parse(valid).unwrap());
    }
}

#[test]
fn unicode_double_accidentals_parse_and_canonicalize_to_ascii() {
    let double_flat = Pitch::from_str("C𝄫").expect("Unicode double-flat should parse");
    assert_eq!(double_flat, Pitch::new(NoteLetter::C, -2));
    assert_eq!(double_flat.to_string(), "Cbb");

    for (input, canonical, pitches) in [
        ("C𝄫maj7", "Cbbmaj7", vec!["Cbb", "Ebb", "Gbb", "Bbb"]),
        ("C𝄪maj7", "C##maj7", vec!["C##", "E##", "G##", "B##"]),
    ] {
        let chord = Chord::parse(input).unwrap();
        assert_eq!(chord.canonical_symbol(), canonical);
        assert_eq!(
            chord
                .notes()
                .iter()
                .map(|note| note.pitch.to_string())
                .collect::<Vec<_>>(),
            pitches
        );
        assert_round_trip(chord);
    }
}

#[test]
fn delta_aliases_keep_their_major_seventh_meaning() {
    for (input, canonical) in [
        ("CΔ", "Cmaj7"),
        ("C∆", "Cmaj7"),
        ("CmΔ", "CmMaj7"),
        ("CΔ#11", "Cmaj7#11"),
        ("CΔ/E", "Cmaj7/E"),
        ("CΔ9", "Cmaj9"),
        ("CΔ13", "Cmaj13"),
    ] {
        let chord = Chord::parse(input).unwrap();
        assert_eq!(chord.canonical_symbol(), canonical, "{}", input);
        assert_round_trip(chord);
    }
}

#[test]
fn theoretical_roots_preserve_scientific_octaves_and_midi_intervals() {
    let roots = [
        Pitch::new(NoteLetter::C, -2),
        Pitch::new(NoteLetter::C, -1),
        Pitch::new(NoteLetter::B, 1),
        Pitch::new(NoteLetter::B, 2),
        Pitch::new(NoteLetter::E, 1),
        Pitch::new(NoteLetter::F, -1),
    ];
    let suffixes = [
        "", "m", "dim7", "6/9", "maj7", "mMaj9", "13", "maj9#11", "add2", "add9", "add13", "7alt",
    ];

    for root in roots {
        for suffix in suffixes {
            let symbol = format!("{}{}", root, suffix);
            let chord = Chord::parse(&symbol).unwrap();
            let root_note = Note::new(root, 4);
            let root_absolute = scientific_semitones(&root_note);
            for (note, tone) in chord.notes().iter().zip(chord.formula().tones()) {
                let expected = root_absolute + tone.semitones() as i32;
                assert_eq!(scientific_semitones(note), expected, "{}", symbol);
                assert_eq!(
                    note.midi_pitch() as i32,
                    expected.clamp(0, 127),
                    "{}",
                    symbol
                );
            }
        }
    }
}

#[test]
fn theoretical_inversions_and_slash_basses_are_strictly_ascending_in_midi() {
    for base in ["Cbmaj9", "B#maj9", "Cbb13", "B##7alt"] {
        let root_position = Chord::parse(base).unwrap();
        for inversion in 0..root_position.formula().tones().len() {
            let chord = Chord::parse(&format!("{}/{}", base, inversion)).unwrap();
            assert!(
                chord
                    .notes()
                    .windows(2)
                    .all(|window| scientific_semitones(&window[0])
                        < scientific_semitones(&window[1])),
                "{} inversion {}: {:?}",
                base,
                inversion,
                chord.notes()
            );
        }
    }

    for symbol in ["Cbmaj7/D", "B#maj7/F#", "Cbb7/E", "B##m7/A"] {
        let chord = Chord::parse(symbol).unwrap();
        let notes = chord.notes();
        assert!(
            scientific_semitones(&notes[0]) < scientific_semitones(&notes[1]),
            "{}",
            symbol
        );
    }
}

#[test]
fn every_valid_builder_base_and_single_modifier_has_a_parseable_canonical_form() {
    let root = Pitch::new(NoteLetter::C, -1);
    let triads = [
        TriadQuality::Major,
        TriadQuality::Minor,
        TriadQuality::Diminished,
        TriadQuality::Augmented,
        TriadQuality::Power,
    ];
    let sevenths = [
        None,
        Some(SeventhQuality::Major),
        Some(SeventhQuality::Minor),
        Some(SeventhQuality::Diminished),
    ];
    let extensions = [
        ChordExtension::Triad,
        ChordExtension::Sixth,
        ChordExtension::SixNine,
        ChordExtension::Seventh,
        ChordExtension::Ninth,
        ChordExtension::Eleventh,
        ChordExtension::Thirteenth,
    ];
    let suspensions = [None, Some(Suspension::Second), Some(Suspension::Fourth)];

    for triad in triads {
        for seventh in sevenths {
            for extension in extensions {
                for suspension in suspensions {
                    let base = configure_builder(root, triad, seventh, extension, suspension);
                    if let Ok(chord) = base.clone().build() {
                        assert_round_trip(chord);
                    }

                    for degree in [2, 3, 4, 6, 9, 11, 13] {
                        for alteration in -2..=2 {
                            if let Ok(chord) = base.clone().add(degree, alteration).unwrap().build()
                            {
                                assert_round_trip(chord);
                            }
                        }
                    }
                    for degree in [5, 9, 11, 13] {
                        for alteration in [-2, -1, 1, 2] {
                            if let Ok(chord) =
                                base.clone().alter(degree, alteration).unwrap().build()
                            {
                                assert_round_trip(chord);
                            }
                        }
                    }
                    for degree in [3, 5, 7, 9, 11, 13] {
                        if let Ok(chord) = base.clone().omit(degree).build() {
                            assert_round_trip(chord);
                        }
                    }
                    if let Ok(chord) = base.clone().altered().build() {
                        assert_round_trip(chord);
                    }
                }
            }
        }
    }
}

#[test]
fn chord_tone_degree_math_and_validation_are_exhaustive() {
    for (degree, natural) in [
        (1, 0),
        (2, 2),
        (3, 4),
        (4, 5),
        (5, 7),
        (6, 9),
        (7, 11),
        (9, 14),
        (11, 17),
        (13, 21),
    ] {
        for alteration in -2..=2 {
            let tone = ChordTone::new(degree, alteration).unwrap();
            assert_eq!(tone.semitones(), natural + alteration as i16);
            assert_eq!(tone.letter_offset(), (degree as i16 - 1).rem_euclid(7));
        }
    }

    for invalid_degree in [0, 8, 10, 12, 14, u8::MAX] {
        assert!(ChordTone::new(invalid_degree, 0).is_err());
    }
    for invalid_alteration in [i8::MIN, -3, 3, i8::MAX] {
        assert!(ChordTone::new(5, invalid_alteration).is_err());
    }
}

#[test]
fn midi_conversion_clamps_both_ends_without_panicking() {
    let cases = [
        (Note::new(Pitch::new(NoteLetter::C, 0), i16::MIN), 0),
        (Note::new(Pitch::new(NoteLetter::C, -1), -1), 0),
        (Note::new(Pitch::new(NoteLetter::B, 1), 4), 72),
        (Note::new(Pitch::new(NoteLetter::C, -1), 4), 59),
        (Note::new(Pitch::new(NoteLetter::B, 0), i16::MAX), 127),
    ];

    for (note, expected) in cases {
        let result = panic::catch_unwind(|| note.midi_pitch());
        assert!(result.is_ok(), "{:?} panicked", note);
        assert_eq!(result.unwrap(), expected, "{:?}", note);
    }
}

#[test]
fn generated_token_streams_never_panic_the_parser() {
    let fragments = [
        "C", "b", "#", "𝄫", "𝄪", "7", "maj", "sus", "add", "no", "/", "(", ")", ",", "🎵",
    ];
    for first in fragments {
        for second in fragments {
            for third in fragments {
                for fourth in fragments {
                    let symbol = format!("{}{}{}{}", first, second, third, fourth);
                    assert!(
                        panic::catch_unwind(|| Chord::parse(&symbol)).is_ok(),
                        "{}",
                        symbol
                    );
                }
            }
        }
    }
}

#[test]
fn parse_errors_point_to_original_bytes_after_alias_and_unicode_normalization() {
    let cases = [
        ("CM7/", 4),
        ("Cøwat", 3),
        ("C Δ9 wat", 6),
        ("C7(♭9,wat)", 8),
        ("C major seventh wat", 16),
    ];

    for (symbol, expected) in cases {
        let error = Chord::parse(symbol).unwrap_err();
        let position = match &error {
            ChordError::UnexpectedToken { position, .. }
            | ChordError::InvalidModifierAt { position, .. }
            | ChordError::ConflictingModifiersAt { position, .. }
            | ChordError::InvalidSlashBass { position }
            | ChordError::UnsupportedConstruction { position, .. } => *position,
            other => panic!("{} returned the wrong error: {:?}", symbol, other),
        };
        assert_eq!(position, expected, "{}: {:?}", symbol, error);
        assert!(
            symbol.is_char_boundary(position),
            "{}: {}",
            symbol,
            position
        );
    }
}

#[test]
fn modifier_errors_point_to_the_invalid_or_conflicting_operation() {
    let cases = [
        ("C7add5", 2),
        ("C7b9#9", 4),
        ("C7altb9", 5),
        ("Cdim7b5", 5),
        ("C7(no5,add5)", 7),
    ];

    for (symbol, expected) in cases {
        let error = Chord::parse(symbol).unwrap_err();
        let position = match &error {
            ChordError::InvalidModifierAt { position, .. }
            | ChordError::ConflictingModifiersAt { position, .. } => *position,
            other => panic!("{} returned the wrong error: {:?}", symbol, other),
        };
        assert_eq!(position, expected, "{}: {:?}", symbol, error);
    }
}
