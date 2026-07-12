extern crate rust_music_theory as theory;

use theory::chord::{
    Chord, ChordError, ChordExtension, ChordModifier, Number, Quality, SeventhQuality, Suspension,
    TriadQuality,
};
use theory::note::{NoteLetter, Notes, Pitch};

fn pitches(symbol: &str) -> Vec<String> {
    Chord::parse(symbol)
        .unwrap()
        .notes()
        .iter()
        .map(|note| note.pitch.to_string())
        .collect()
}

#[test]
fn parses_requested_symbols_with_canonical_forms_and_spelling() {
    let cases: &[(&str, &str, &[&str])] = &[
        ("C7sus4", "C7sus4", &["C", "F", "G", "Bb"]),
        ("Cm7b5", "Cm7b5", &["C", "Eb", "Gb", "Bb"]),
        ("Cø7", "Cm7b5", &["C", "Eb", "Gb", "Bb"]),
        ("Cadd9", "Cadd9", &["C", "E", "G", "D"]),
        ("C7no5", "C7no5", &["C", "E", "Bb"]),
        ("C5", "C5", &["C", "G"]),
        ("C6", "C6", &["C", "E", "G", "A"]),
        ("Cm6", "Cm6", &["C", "Eb", "G", "A"]),
        ("C6/9", "C6/9", &["C", "E", "G", "A", "D"]),
        ("C7alt", "C7alt", &["C", "E", "Gb", "G#", "Bb", "Db", "D#"]),
        ("CΔ9", "Cmaj9", &["C", "E", "G", "B", "D"]),
        ("C7(b9,#11)", "C7b9#11", &["C", "E", "G", "Bb", "Db", "F#"]),
    ];

    for (input, canonical, expected) in cases {
        let chord = Chord::parse(input).unwrap_or_else(|error| panic!("{}: {}", input, error));
        assert_eq!(chord.canonical_symbol(), *canonical, "{}", input);
        assert_eq!(pitches(input), *expected, "{}", input);
        let reparsed = Chord::parse(&chord.canonical_symbol()).unwrap();
        assert_eq!(reparsed.spec(), chord.spec(), "{}", input);
    }
}

#[test]
fn slash_basses_and_numeric_inversions_stay_synchronized() {
    let chord = Chord::parse("Cmaj9#11/G").unwrap();
    assert_eq!(chord.canonical_symbol(), "Cmaj9#11/G");
    assert_eq!(chord.bass().unwrap().to_string(), "G");
    assert_eq!(pitches("Cmaj9#11/G"), ["G", "B", "D", "F#", "C", "E"]);

    let numeric = Chord::parse("C/1").unwrap();
    assert_eq!(numeric.canonical_symbol(), "C/E");
    assert_eq!(numeric.inversion(), 1);

    let six_nine = Chord::parse("C6/9/E").unwrap();
    assert_eq!(six_nine.canonical_symbol(), "C6/9/E");
    assert_eq!(six_nine.inversion(), 1);

    let non_member = Chord::parse("C/F#").unwrap();
    assert_eq!(non_member.inversion(), 0);
    assert_eq!(non_member.canonical_symbol(), "C/F#");
    assert_eq!(pitches("C/F#"), ["F#", "C", "E", "G"]);
}

#[test]
fn builder_produces_the_same_normalized_specification() {
    let root = Pitch::new(NoteLetter::C, 0);
    let built = Chord::builder(root)
        .extension(ChordExtension::Ninth)
        .seventh_quality(SeventhQuality::Major)
        .alter(11, 1)
        .unwrap()
        .bass(Pitch::new(NoteLetter::G, 0))
        .build()
        .unwrap();
    assert_eq!(built.canonical_symbol(), "Cmaj9#11/G");
    assert_eq!(built.spec(), Chord::parse("Cmaj9#11/G").unwrap().spec());

    let suspended = Chord::builder(root)
        .extension(ChordExtension::Seventh)
        .seventh_quality(SeventhQuality::Minor)
        .suspension(Suspension::Fourth)
        .add(3, 0)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(suspended.canonical_symbol(), "C7sus4add3");
    assert!(suspended.spec().modifiers().contains(&ChordModifier::Add(
        theory::chord::ChordTone::new(3, 0).unwrap()
    )));
}

#[test]
fn builder_rejects_incompatible_seventh_qualities() {
    let root = Pitch::new(NoteLetter::C, 0);
    assert!(Chord::builder(root)
        .triad_quality(TriadQuality::Diminished)
        .extension(ChordExtension::Seventh)
        .seventh_quality(SeventhQuality::Major)
        .build()
        .is_err());
    assert!(Chord::builder(root)
        .extension(ChordExtension::Seventh)
        .seventh_quality(SeventhQuality::Diminished)
        .build()
        .is_err());
}

#[test]
fn rejects_conflicts_and_unsupported_tokens_without_panicking() {
    for symbol in [
        "C7b9b9",
        "C7b9#9",
        "Csus2sus4",
        "C7altb9",
        "Cadd5",
        "C7no2",
        "Cdim7b5",
        "Caug7#5",
        "C/",
        "C//E",
        "C[maj7]",
        "C7(b9",
        "C7b9)",
    ] {
        let result = std::panic::catch_unwind(|| Chord::parse(symbol));
        assert!(result.is_ok(), "{} panicked", symbol);
        assert!(result.unwrap().is_err(), "{} should be rejected", symbol);
    }
}

#[test]
fn theoretical_roots_keep_generic_letter_spelling() {
    assert_eq!(pitches("C#maj9#11"), ["C#", "E#", "G#", "B#", "D#", "F##"]);
    assert_eq!(pitches("Cb7b9"), ["Cb", "Eb", "Gb", "Bbb", "Dbb"]);
}

#[test]
fn public_spec_types_describe_the_normalized_chord() {
    let chord = Chord::parse("Cm7b5").unwrap();
    assert_eq!(chord.spec().triad_quality(), TriadQuality::Diminished);
    assert_eq!(chord.spec().seventh_quality(), Some(SeventhQuality::Minor));
    assert_eq!(chord.spec().extension(), ChordExtension::Seventh);
    assert_eq!(
        chord
            .formula()
            .tones()
            .iter()
            .map(|tone| (tone.degree(), tone.alteration()))
            .collect::<Vec<_>>(),
        vec![(1, 0), (3, -1), (5, -1), (7, -1)]
    );
}

#[test]
fn legacy_power_and_sixth_adapters_remain_available() {
    let root = Pitch::new(NoteLetter::C, 0);
    assert_eq!(
        Chord::new(root, Quality::Power, Number::Fifth).canonical_symbol(),
        "C5"
    );
    assert_eq!(
        Chord::new(root, Quality::Major, Number::Sixth).canonical_symbol(),
        "C6"
    );
    assert_eq!(
        Chord::new(root, Quality::Minor, Number::SixNine).canonical_symbol(),
        "Cm6/9"
    );
    assert_eq!(Chord::from_interval(root, &[7]).unwrap().to_string(), "C5");
    assert_eq!(
        Chord::from_interval(root, &[4, 3, 2, 5])
            .unwrap()
            .to_string(),
        "C6/9"
    );
}

#[test]
#[allow(deprecated)]
fn deprecated_regex_name_delegates_to_the_new_parser() {
    assert_eq!(
        Chord::from_regex("Cø7").unwrap().canonical_symbol(),
        "Cm7b5"
    );
}

#[test]
fn every_core_family_round_trips_across_natural_sharp_and_flat_roots() {
    let roots = ["C", "F#", "Bb", "Cb", "E#"];
    let suffixes = [
        "", "m", "dim", "aug", "5", "6", "m6", "6/9", "m6/9", "7", "maj7", "m7", "mMaj7", "m7b5",
        "dim7", "aug7", "augMaj7", "9", "maj9", "m9", "mMaj9", "11", "maj11", "m11", "mMaj11",
        "13", "maj13", "m13", "mMaj13", "sus2", "sus4", "7sus4", "add2", "add4", "add6", "add9",
        "add11", "add13", "7b5", "7#5", "7b9", "7#9", "7#11", "7b13", "7no5", "7alt",
    ];

    for root in roots {
        for suffix in suffixes {
            let symbol = format!("{}{}", root, suffix);
            let chord = Chord::parse(&symbol)
                .unwrap_or_else(|error| panic!("{} should parse: {}", symbol, error));
            assert_eq!(
                chord.notes().len(),
                chord.formula().tones().len(),
                "{}",
                symbol
            );
            let canonical = chord.canonical_symbol();
            let reparsed = Chord::parse(&canonical).unwrap();
            assert_eq!(reparsed.spec(), chord.spec(), "{} -> {}", symbol, canonical);
            assert_eq!(
                reparsed.notes(),
                chord.notes(),
                "{} -> {}",
                symbol,
                canonical
            );
        }
    }
}

#[test]
fn aliases_and_modifier_order_normalize_deterministically() {
    for (alias, canonical) in [
        ("C^7", "Cmaj7"),
        ("CM7", "Cmaj7"),
        ("C-7", "Cm7"),
        ("C°7", "Cdim7"),
        ("Co7", "Cdim7"),
        ("Cø", "Cm7b5"),
        ("C+7", "Caug7"),
        ("C7sus", "C7sus4"),
        ("C7omit5", "C7no5"),
        ("C♭maj7", "Cbmaj7"),
        ("F♯m7", "F#m7"),
    ] {
        assert_eq!(Chord::parse(alias).unwrap().canonical_symbol(), canonical);
    }

    assert_eq!(
        Chord::parse("C7(no5,add13,#11,b9)")
            .unwrap()
            .canonical_symbol(),
        "C7b9#11add13no5"
    );
}

#[test]
fn parse_error_categories_carry_source_positions() {
    assert!(matches!(
        Chord::parse("H7"),
        Err(ChordError::InvalidRoot { position: 0 })
    ));
    assert!(matches!(
        Chord::parse("C7wat"),
        Err(ChordError::UnexpectedToken { position: 2, .. })
    ));
    assert!(matches!(
        Chord::parse("Cadd5"),
        Err(ChordError::InvalidModifierAt { position: 1, .. })
    ));
    assert!(matches!(
        Chord::parse("C7b9#9"),
        Err(ChordError::ConflictingModifiersAt { position: 4, .. })
    ));
    assert!(matches!(
        Chord::parse("C/"),
        Err(ChordError::InvalidSlashBass { position: 2 })
    ));
    assert!(matches!(
        Chord::parse("C/9"),
        Err(ChordError::InvalidSlashBass { position: 2 })
    ));
}
