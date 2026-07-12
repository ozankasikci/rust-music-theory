extern crate rust_music_theory as theory;

use std::panic;

use theory::chord::{
    Chord, ChordError, ChordExtension, ChordSpec, Number, Quality, SeventhQuality, Suspension,
    TriadQuality,
};
use theory::note::{Note, NoteLetter, Notes, Pitch};

fn formula(symbol: &str) -> Vec<(u8, i8)> {
    Chord::parse(symbol)
        .unwrap_or_else(|error| panic!("{} should parse: {}", symbol, error))
        .formula()
        .tones()
        .iter()
        .map(|tone| (tone.degree(), tone.alteration()))
        .collect()
}

fn absolute(note: &Note) -> i32 {
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

fn letter_index(letter: NoteLetter) -> i16 {
    match letter {
        NoteLetter::C => 0,
        NoteLetter::D => 1,
        NoteLetter::E => 2,
        NoteLetter::F => 3,
        NoteLetter::G => 4,
        NoteLetter::A => 5,
        NoteLetter::B => 6,
    }
}

// MusicXML 4.0 kind-value defines these base chord formulas. The additional
// six-nine, minor-major extensions, and deterministic alt formula are explicit
// requirements of this crate's v0.5 grammar.
// https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/kind-value/
#[test]
fn source_backed_core_formula_matrix_is_exact() {
    let cases: &[(&str, &[(u8, i8)])] = &[
        ("C", &[(1, 0), (3, 0), (5, 0)]),
        ("Cm", &[(1, 0), (3, -1), (5, 0)]),
        ("Cdim", &[(1, 0), (3, -1), (5, -1)]),
        ("Caug", &[(1, 0), (3, 0), (5, 1)]),
        ("C5", &[(1, 0), (5, 0)]),
        ("C6", &[(1, 0), (3, 0), (5, 0), (6, 0)]),
        ("Cm6", &[(1, 0), (3, -1), (5, 0), (6, 0)]),
        ("C6/9", &[(1, 0), (3, 0), (5, 0), (6, 0), (9, 0)]),
        ("Cm6/9", &[(1, 0), (3, -1), (5, 0), (6, 0), (9, 0)]),
        ("C7", &[(1, 0), (3, 0), (5, 0), (7, -1)]),
        ("Cmaj7", &[(1, 0), (3, 0), (5, 0), (7, 0)]),
        ("Cm7", &[(1, 0), (3, -1), (5, 0), (7, -1)]),
        ("CmMaj7", &[(1, 0), (3, -1), (5, 0), (7, 0)]),
        ("Cm7b5", &[(1, 0), (3, -1), (5, -1), (7, -1)]),
        ("Cdim7", &[(1, 0), (3, -1), (5, -1), (7, -2)]),
        ("Caug7", &[(1, 0), (3, 0), (5, 1), (7, -1)]),
        ("CaugMaj7", &[(1, 0), (3, 0), (5, 1), (7, 0)]),
        ("C9", &[(1, 0), (3, 0), (5, 0), (7, -1), (9, 0)]),
        ("Cmaj9", &[(1, 0), (3, 0), (5, 0), (7, 0), (9, 0)]),
        ("Cm9", &[(1, 0), (3, -1), (5, 0), (7, -1), (9, 0)]),
        ("CmMaj9", &[(1, 0), (3, -1), (5, 0), (7, 0), (9, 0)]),
        ("C11", &[(1, 0), (3, 0), (5, 0), (7, -1), (9, 0), (11, 0)]),
        ("Cmaj11", &[(1, 0), (3, 0), (5, 0), (7, 0), (9, 0), (11, 0)]),
        ("Cm11", &[(1, 0), (3, -1), (5, 0), (7, -1), (9, 0), (11, 0)]),
        (
            "CmMaj11",
            &[(1, 0), (3, -1), (5, 0), (7, 0), (9, 0), (11, 0)],
        ),
        (
            "C13",
            &[(1, 0), (3, 0), (5, 0), (7, -1), (9, 0), (11, 0), (13, 0)],
        ),
        (
            "Cmaj13",
            &[(1, 0), (3, 0), (5, 0), (7, 0), (9, 0), (11, 0), (13, 0)],
        ),
        (
            "Cm13",
            &[(1, 0), (3, -1), (5, 0), (7, -1), (9, 0), (11, 0), (13, 0)],
        ),
        (
            "CmMaj13",
            &[(1, 0), (3, -1), (5, 0), (7, 0), (9, 0), (11, 0), (13, 0)],
        ),
        ("Csus2", &[(1, 0), (2, 0), (5, 0)]),
        ("Csus4", &[(1, 0), (4, 0), (5, 0)]),
        ("C7sus4", &[(1, 0), (4, 0), (5, 0), (7, -1)]),
        (
            "C7alt",
            &[(1, 0), (3, 0), (5, -1), (5, 1), (7, -1), (9, -1), (9, 1)],
        ),
    ];

    for (symbol, expected) in cases {
        assert_eq!(formula(symbol), *expected, "{}", symbol);
    }
}

// These aliases are present in Tonal's current chord dictionary and/or the
// ChordPro reference implementation's built-in extension list.
// https://github.com/tonaljs/tonal/blob/main/packages/chord-type/data.ts
// https://www.chordpro.org/chordpro/chordpro-chords/
#[test]
fn common_tonal_and_chordpro_aliases_normalize_portably() {
    for (input, canonical) in [
        ("Cma7", "Cmaj7"),
        ("CMaj7", "Cmaj7"),
        ("CMadd9", "Cadd9"),
        ("Cmi7", "Cm7"),
        ("Cmin7", "Cm7"),
        ("Cmadd9", "Cmadd9"),
        ("C69", "C6/9"),
        ("C6add9", "C6/9"),
        ("CM69", "C6/9"),
        ("Cm69", "Cm6/9"),
        ("Cm6add9", "Cm6/9"),
        ("C-69", "Cm6/9"),
        ("Ch", "Cm7b5"),
        ("Ch7", "Cm7b5"),
        ("C0", "Cdim"),
        ("C07", "Cdim7"),
        ("Calt", "C7alt"),
        ("Calt7", "C7alt"),
        ("C7+", "Caug7"),
        ("C7aug", "Caug7"),
        ("C+M7", "CaugMaj7"),
        ("CaugM7", "CaugMaj7"),
        ("CmM7", "CmMaj7"),
        ("Cm/M7", "CmMaj7"),
        ("Cm/ma7", "CmMaj7"),
        ("C-Δ7", "CmMaj7"),
        ("C2", "Cadd2"),
        ("C4", "Cadd4"),
        ("C69/E", "C6/9/E"),
        ("Cm7sus4", "Cm7sus4"),
    ] {
        let chord = Chord::parse(input).unwrap_or_else(|error| panic!("{}: {}", input, error));
        assert_eq!(chord.canonical_symbol(), canonical, "{}", input);
        assert_eq!(
            Chord::parse(canonical).unwrap().spec(),
            chord.spec(),
            "{}",
            input
        );
    }
    assert_eq!(formula("Cm7sus4"), [(1, 0), (4, 0), (5, 0), (7, -1)]);
}

#[test]
fn degree_operations_resolve_in_base_suspend_alter_add_omit_order() {
    assert_eq!(
        formula("C7sus4add3#11no5"),
        [(1, 0), (3, 0), (4, 0), (7, -1), (11, 1)]
    );
    assert_eq!(
        Chord::parse("C7sus4add3#11no5").unwrap().canonical_symbol(),
        "C7sus4#11add3no5"
    );
    assert_eq!(
        formula("Cm7sus4addb3"),
        [(1, 0), (3, -1), (4, 0), (5, 0), (7, -1)]
    );
    assert_eq!(
        formula("C9b9add11no5"),
        [(1, 0), (3, 0), (7, -1), (9, -1), (11, 0)]
    );
}

#[test]
fn simple_and_compound_additions_keep_distinct_octave_intent() {
    let cases = [
        ("Cadd2", vec![("C", 4), ("D", 4), ("E", 4), ("G", 4)]),
        ("Cadd9", vec![("C", 4), ("E", 4), ("G", 4), ("D", 5)]),
        ("Cadd6", vec![("C", 4), ("E", 4), ("G", 4), ("A", 4)]),
        ("Cadd13", vec![("C", 4), ("E", 4), ("G", 4), ("A", 5)]),
        (
            "C6/9",
            vec![("C", 4), ("E", 4), ("G", 4), ("A", 4), ("D", 5)],
        ),
    ];

    for (symbol, expected) in cases {
        let actual: Vec<(String, i16)> = Chord::parse(symbol)
            .unwrap()
            .notes()
            .iter()
            .map(|note| (note.pitch.to_string(), note.octave))
            .collect();
        let expected: Vec<(String, i16)> = expected
            .into_iter()
            .map(|(pitch, octave)| (pitch.to_string(), octave))
            .collect();
        assert_eq!(actual, expected, "{}", symbol);
    }
}

fn permutations(items: &[&str]) -> Vec<Vec<String>> {
    fn visit(items: &mut Vec<String>, index: usize, output: &mut Vec<Vec<String>>) {
        if index == items.len() {
            output.push(items.clone());
            return;
        }
        for next in index..items.len() {
            items.swap(index, next);
            visit(items, index + 1, output);
            items.swap(index, next);
        }
    }

    let mut items: Vec<String> = items.iter().map(|item| item.to_string()).collect();
    let mut output = Vec::new();
    visit(&mut items, 0, &mut output);
    output
}

#[test]
fn every_modifier_permutation_has_one_canonical_form() {
    let expected = Chord::parse("C7b9#11add13no5").unwrap();
    for permutation in permutations(&["b9", "#11", "add13", "no5"]) {
        let symbol = format!("C7{}", permutation.join(""));
        let chord = Chord::parse(&symbol).unwrap_or_else(|error| panic!("{}: {}", symbol, error));
        assert_eq!(chord.canonical_symbol(), "C7b9#11add13no5", "{}", symbol);
        assert_eq!(chord.spec(), expected.spec(), "{}", symbol);
        assert_eq!(chord.notes(), expected.notes(), "{}", symbol);
    }
}

#[test]
fn altered_major_triads_have_unambiguous_canonical_round_trips() {
    for (input, canonical) in [
        ("C(b5)", "Cmajb5"),
        ("C(#5)", "Cmaj#5"),
        ("C(#11)", "Cmaj#11"),
        ("C#(b5)", "C#majb5"),
        ("Cb(#11)", "Cbmaj#11"),
    ] {
        let chord = Chord::parse(input).unwrap();
        assert_eq!(chord.canonical_symbol(), canonical, "{}", input);
        let reparsed = Chord::parse(canonical).unwrap();
        assert_eq!(reparsed.spec(), chord.spec(), "{}", input);
        assert_eq!(reparsed.notes(), chord.notes(), "{}", input);
    }

    assert_eq!(Chord::parse("Cb5").unwrap().canonical_symbol(), "Cb5");
    assert_eq!(Chord::parse("C#11").unwrap().canonical_symbol(), "C#11");
}

#[test]
fn generated_notes_match_every_formula_tone_for_theoretical_roots() {
    let suffixes = [
        "", "m", "dim", "aug", "5", "6/9", "m6/9", "7", "maj7", "mMaj7", "m7b5", "dim7", "augMaj7",
        "13", "maj13", "m13", "mMaj13", "7sus4", "add2", "add9", "add13", "7b9#11", "7no5", "7alt",
    ];
    let letters = [
        NoteLetter::C,
        NoteLetter::D,
        NoteLetter::E,
        NoteLetter::F,
        NoteLetter::G,
        NoteLetter::A,
        NoteLetter::B,
    ];

    for root_letter in letters {
        for root_alteration in -2..=2 {
            let root = Pitch::new(root_letter, root_alteration);
            for suffix in suffixes {
                let symbol = format!("{}{}", root, suffix);
                let chord = Chord::parse(&symbol).unwrap();
                let notes = chord.notes();
                assert_eq!(notes.len(), chord.formula().tones().len(), "{}", symbol);
                let root_absolute = absolute(&Note::new(root, 4));
                for (note, tone) in notes.iter().zip(chord.formula().tones()) {
                    assert_eq!(
                        absolute(note) - root_absolute,
                        tone.semitones() as i32,
                        "{} degree {}",
                        symbol,
                        tone.degree()
                    );
                    assert_eq!(
                        (letter_index(note.pitch.letter) - letter_index(root_letter)).rem_euclid(7),
                        (tone.degree() as i16 - 1).rem_euclid(7),
                        "{} degree {} spelling",
                        symbol,
                        tone.degree()
                    );
                }
            }
        }
    }
}

#[test]
fn every_numeric_inversion_is_ordered_and_formats_its_actual_bass() {
    for base in ["C", "Cm7", "Cdim7", "C6/9", "Cmaj9#11", "C13", "C7alt"] {
        let root_position = Chord::parse(base).unwrap();
        for inversion in 0..root_position.formula().tones().len() {
            let symbol = format!("{}/{}", base, inversion);
            let chord = Chord::parse(&symbol).unwrap();
            let notes = chord.notes();
            assert_eq!(chord.inversion(), inversion as u8, "{}", symbol);
            assert!(
                notes
                    .windows(2)
                    .all(|window| absolute(&window[0]) < absolute(&window[1])),
                "{} is not strictly ascending: {:?}",
                symbol,
                notes
            );
            if inversion == 0 {
                assert_eq!(chord.canonical_symbol(), base, "{}", symbol);
            } else {
                assert_eq!(chord.bass(), Some(notes[0].pitch), "{}", symbol);
                assert!(
                    chord
                        .canonical_symbol()
                        .ends_with(&format!("/{}", notes[0].pitch)),
                    "{}",
                    symbol
                );
            }
        }
        let invalid = format!("{}/{}", base, root_position.formula().tones().len());
        assert!(matches!(
            Chord::parse(&invalid),
            Err(ChordError::InvalidSlashBass { .. })
        ));
    }
}

#[test]
fn chord_member_and_non_chord_slash_basses_have_distinct_behavior() {
    let member = Chord::parse("Cmaj7/D##").unwrap();
    assert_eq!(member.inversion(), 1);
    assert_eq!(member.canonical_symbol(), "Cmaj7/E");
    assert_eq!(member.notes()[0].pitch.to_string(), "E");

    for bass in ["Db", "D", "F", "Gb", "Ab", "A", "Bb"] {
        let symbol = format!("Cmaj7/{}", bass);
        let chord = Chord::parse(&symbol).unwrap();
        let notes = chord.notes();
        assert_eq!(chord.inversion(), 0, "{}", symbol);
        assert_eq!(chord.bass().unwrap().to_string(), bass, "{}", symbol);
        assert_eq!(notes[0].pitch.to_string(), bass, "{}", symbol);
        assert!(absolute(&notes[0]) < absolute(&notes[1]), "{}", symbol);
    }
}

#[test]
fn malformed_and_unicode_input_never_panics() {
    let mut invalid = vec![
        "".to_string(),
        " ".to_string(),
        "🎵".to_string(),
        "N.C.".to_string(),
        "[C7]".to_string(),
        "C7()".to_string(),
        "C7(,)".to_string(),
        "C7((b9))".to_string(),
        "C7(b9,,#11)".to_string(),
        "C7(b9,#11".to_string(),
        "C7b9,#11)".to_string(),
        "C///E".to_string(),
        "C6/9/".to_string(),
        "C/999999999999999999999999".to_string(),
        "C/H".to_string(),
        "C7add8".to_string(),
        "C7omit8".to_string(),
        "C7b8".to_string(),
        "C7###9".to_string(),
        "C7bbb9".to_string(),
        "C7altadd9".to_string(),
        "C7sus2sus4".to_string(),
        "Cno3no5".to_string(),
        "C7no3no5no7".to_string(),
        "C♯♭7".to_string(),
        "C𝄪♭7".to_string(),
        "C\0maj7".to_string(),
    ];
    invalid.push(format!("C{}7", "#".repeat(512)));
    invalid.push(format!("C7{}9", "#".repeat(512)));
    invalid.push(format!("C/E{}", "b".repeat(512)));

    for symbol in invalid {
        let result = panic::catch_unwind(|| Chord::parse(&symbol));
        assert!(result.is_ok(), "{} panicked", symbol.escape_debug());
        assert!(
            result.unwrap().is_err(),
            "{} should be rejected",
            symbol.escape_debug()
        );
    }
}

#[test]
fn builder_and_spec_validation_reject_every_incompatible_construction() {
    let root = Pitch::new(NoteLetter::C, 0);
    assert!(ChordSpec::new(TriadQuality::Major, None, ChordExtension::Seventh).is_err());
    assert!(ChordSpec::new(
        TriadQuality::Major,
        Some(SeventhQuality::Minor),
        ChordExtension::Triad
    )
    .is_err());
    assert!(ChordSpec::new(TriadQuality::Power, None, ChordExtension::Sixth).is_err());
    assert!(ChordSpec::new(
        TriadQuality::Diminished,
        Some(SeventhQuality::Diminished),
        ChordExtension::Ninth
    )
    .is_err());
    assert!(Chord::builder(root)
        .triad_quality(TriadQuality::Augmented)
        .suspension(Suspension::Fourth)
        .build()
        .is_err());
    assert!(Chord::builder(root)
        .extension(ChordExtension::Seventh)
        .seventh_quality(SeventhQuality::Minor)
        .altered()
        .add(9, -1)
        .unwrap()
        .build()
        .is_err());
    assert!(Chord::builder(root)
        .add(9, 0)
        .unwrap()
        .add(9, 0)
        .unwrap()
        .build()
        .is_err());
    assert!(Chord::builder(root).omit(3).omit(5).build().is_err());
}

#[test]
fn every_legacy_quality_number_pair_is_fallible_without_panicking() {
    let qualities = [
        Quality::Major,
        Quality::Minor,
        Quality::Diminished,
        Quality::Augmented,
        Quality::HalfDiminished,
        Quality::Dominant,
        Quality::Suspended2,
        Quality::Suspended4,
        Quality::Power,
    ];
    let numbers = [
        Number::Triad,
        Number::Fifth,
        Number::Sixth,
        Number::SixNine,
        Number::Seventh,
        Number::MajorSeventh,
        Number::Ninth,
        Number::Eleventh,
        Number::Thirteenth,
    ];
    let root = Pitch::new(NoteLetter::F, 1);

    for quality in qualities {
        for number in numbers {
            let result = panic::catch_unwind(|| Chord::try_new(root, quality, number));
            assert!(result.is_ok(), "{:?} {:?} panicked", quality, number);
            if let Ok(chord) = result.unwrap() {
                assert_eq!(chord.root(), root);
                assert_eq!(chord.quality(), quality);
                assert_eq!(chord.number(), number);
                assert_eq!(chord.intervals().len() + 1, chord.formula().tones().len());
                let reparsed = Chord::parse(&chord.canonical_symbol()).unwrap();
                assert_eq!(reparsed.spec(), chord.spec());
                assert_eq!(reparsed.notes(), chord.notes());
            }
        }
    }
}
