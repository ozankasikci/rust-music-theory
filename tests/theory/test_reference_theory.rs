//! Broad theory conformance tests derived from the following open textbook chapters:
//! - https://viva.pressbooks.pub/openmusictheory/chapter/intervals/
//! - https://viva.pressbooks.pub/openmusictheory/chapter/triads/
//! - https://viva.pressbooks.pub/openmusictheory/chapter/seventh-chords/
//! - https://viva.pressbooks.pub/openmusictheory/chapter/chord-symbols/
//! - https://viva.pressbooks.pub/openmusictheory/chapter/major-scales/
//! - https://viva.pressbooks.pub/openmusictheory/chapter/minor-scales/

extern crate rust_music_theory as theory;

use theory::chord::{Chord, Number as ChordNumber, Quality as ChordQuality};
use theory::interval::{Interval, Number as IntervalNumber, Quality as IntervalQuality};
use theory::note::{NoteLetter, Notes, Pitch};
use theory::scale::{Direction, Mode, Scale, ScaleType};

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

fn roots_with_common_accidentals() -> Vec<Pitch> {
    let letters = [
        NoteLetter::C,
        NoteLetter::D,
        NoteLetter::E,
        NoteLetter::F,
        NoteLetter::G,
        NoteLetter::A,
        NoteLetter::B,
    ];
    letters
        .iter()
        .flat_map(|letter| (-1..=1).map(move |accidental| Pitch::new(*letter, accidental)))
        .collect()
}

fn pitch_classes(notes: &[theory::note::Note]) -> Vec<u8> {
    notes.iter().map(|note| note.pitch.into_u8()).collect()
}

#[test]
fn all_canonical_simple_intervals_and_inversions_follow_tonal_rules() {
    use IntervalNumber::*;
    use IntervalQuality::*;

    let canonical = [
        (0, Perfect, Unison),
        (1, Minor, Second),
        (2, Major, Second),
        (3, Minor, Third),
        (4, Major, Third),
        (5, Perfect, Fourth),
        (6, Diminished, Fifth),
        (7, Perfect, Fifth),
        (8, Minor, Sixth),
        (9, Major, Sixth),
        (10, Minor, Seventh),
        (11, Major, Seventh),
        (12, Perfect, Octave),
    ];

    for (semitones, quality, number) in canonical {
        let interval = Interval::from_semitone(semitones).unwrap();
        assert_eq!((interval.quality, interval.number), (quality, number));

        let inverse = Interval::invert(&interval).unwrap();
        assert_eq!(interval.semitone_count + inverse.semitone_count, 12);
        assert_eq!(Interval::invert(&inverse).unwrap(), interval);
    }

    let diminished_fifth = Interval::from_semitone(6).unwrap();
    let augmented_fourth = Interval::invert(&diminished_fifth).unwrap();
    assert_eq!(
        (augmented_fourth.quality, augmented_fourth.number),
        (Augmented, Fourth)
    );

    let default = Interval::default();
    assert_eq!((default.quality, default.number), (Perfect, Unison));
}

#[test]
fn pitch_arithmetic_is_modulo_twelve_for_large_accidentals() {
    for root in roots_with_common_accidentals() {
        for accidental in -32..=32 {
            let pitch = Pitch::new(root.letter, accidental);
            assert!(pitch.into_u8() < 12);
            assert_eq!(Pitch::from_str(&pitch.to_string()), Some(pitch));
        }
    }

    for spelling in ["C♯", "dS", "F𝄪", "gxx", "B♭♭"] {
        let (parsed, matched) = Pitch::from_regex(spelling).unwrap();
        assert_eq!(matched.as_str(), spelling);
        assert_eq!(parsed, Pitch::from_str(spelling).unwrap());
    }
}

#[test]
fn every_supported_chord_formula_is_transposed_and_spelled_by_chord_member() {
    use ChordNumber::*;
    use ChordQuality::*;

    let formulas: &[(ChordQuality, ChordNumber, &[u8], &[i16])] = &[
        (Major, Triad, &[4, 3], &[0, 2, 4]),
        (Minor, Triad, &[3, 4], &[0, 2, 4]),
        (Suspended2, Triad, &[2, 5], &[0, 1, 4]),
        (Suspended4, Triad, &[5, 2], &[0, 3, 4]),
        (Augmented, Triad, &[4, 4], &[0, 2, 4]),
        (Diminished, Triad, &[3, 3], &[0, 2, 4]),
        (Major, Seventh, &[4, 3, 4], &[0, 2, 4, 6]),
        (Minor, Seventh, &[3, 4, 3], &[0, 2, 4, 6]),
        (Augmented, Seventh, &[4, 4, 2], &[0, 2, 4, 6]),
        (Augmented, MajorSeventh, &[4, 4, 3], &[0, 2, 4, 6]),
        (Diminished, Seventh, &[3, 3, 3], &[0, 2, 4, 6]),
        (HalfDiminished, Seventh, &[3, 3, 4], &[0, 2, 4, 6]),
        (Minor, MajorSeventh, &[3, 4, 4], &[0, 2, 4, 6]),
        (Dominant, Seventh, &[4, 3, 3], &[0, 2, 4, 6]),
        (Dominant, Ninth, &[4, 3, 3, 4], &[0, 2, 4, 6, 8]),
        (Major, Ninth, &[4, 3, 4, 3], &[0, 2, 4, 6, 8]),
        (Minor, Ninth, &[3, 4, 3, 4], &[0, 2, 4, 6, 8]),
        (Dominant, Eleventh, &[4, 3, 3, 4, 3], &[0, 2, 4, 6, 8, 10]),
        (Major, Eleventh, &[4, 3, 4, 3, 3], &[0, 2, 4, 6, 8, 10]),
        (Minor, Eleventh, &[3, 4, 3, 4, 3], &[0, 2, 4, 6, 8, 10]),
        (
            Dominant,
            Thirteenth,
            &[4, 3, 3, 4, 3, 4],
            &[0, 2, 4, 6, 8, 10, 12],
        ),
        (
            Major,
            Thirteenth,
            &[4, 3, 4, 3, 3, 4],
            &[0, 2, 4, 6, 8, 10, 12],
        ),
        (
            Minor,
            Thirteenth,
            &[3, 4, 3, 4, 3, 4],
            &[0, 2, 4, 6, 8, 10, 12],
        ),
    ];

    for &(quality, number, adjacent_intervals, member_letters) in formulas {
        assert_eq!(
            Chord::chord_intervals(quality, number)
                .iter()
                .map(|interval| interval.semitone_count)
                .collect::<Vec<_>>(),
            adjacent_intervals
        );

        for root in roots_with_common_accidentals() {
            let chord = Chord::new(root, quality, number);
            let notes = chord.notes();
            assert_eq!(notes.len(), adjacent_intervals.len() + 1);

            for (window, expected) in notes.windows(2).zip(adjacent_intervals) {
                let actual = (window[1].pitch.into_u8() + 12 - window[0].pitch.into_u8()) % 12;
                assert_eq!(actual, *expected, "Wrong semitone formula for {:?}", chord);
            }

            for (note, member_offset) in notes.iter().zip(member_letters.iter()) {
                let actual =
                    (letter_index(note.pitch.letter) - letter_index(root.letter)).rem_euclid(7);
                assert_eq!(
                    actual,
                    member_offset.rem_euclid(7),
                    "Wrong spelling for {:?}",
                    chord
                );
            }

            for inversion in 0..notes.len() {
                let inverted =
                    Chord::with_inversion(root, quality, number, inversion as u8).notes();
                let expected: Vec<Pitch> = notes
                    .iter()
                    .cycle()
                    .skip(inversion)
                    .take(notes.len())
                    .map(|note| note.pitch)
                    .collect();
                assert_eq!(
                    inverted.iter().map(|note| note.pitch).collect::<Vec<_>>(),
                    expected
                );
                assert!(inverted
                    .windows(2)
                    .all(|window| window[0].octave <= window[1].octave));
            }

            let recognized = Chord::from_interval(root, adjacent_intervals).unwrap();
            assert_eq!((recognized.quality(), recognized.number()), (quality, number));
        }
    }
}

#[test]
fn all_scale_families_have_their_reference_step_patterns_in_both_directions() {
    use Direction::*;
    use ScaleType::*;

    let scales: &[(ScaleType, Option<Mode>, &[u8], &[u8])] = &[
        (
            Diatonic,
            Some(Mode::Ionian),
            &[2, 2, 1, 2, 2, 2, 1],
            &[2, 2, 1, 2, 2, 2, 1],
        ),
        (
            Diatonic,
            Some(Mode::Dorian),
            &[2, 1, 2, 2, 2, 1, 2],
            &[2, 1, 2, 2, 2, 1, 2],
        ),
        (
            Diatonic,
            Some(Mode::Phrygian),
            &[1, 2, 2, 2, 1, 2, 2],
            &[1, 2, 2, 2, 1, 2, 2],
        ),
        (
            Diatonic,
            Some(Mode::Lydian),
            &[2, 2, 2, 1, 2, 2, 1],
            &[2, 2, 2, 1, 2, 2, 1],
        ),
        (
            Diatonic,
            Some(Mode::Mixolydian),
            &[2, 2, 1, 2, 2, 1, 2],
            &[2, 2, 1, 2, 2, 1, 2],
        ),
        (
            Diatonic,
            Some(Mode::Aeolian),
            &[2, 1, 2, 2, 1, 2, 2],
            &[2, 1, 2, 2, 1, 2, 2],
        ),
        (
            Diatonic,
            Some(Mode::Locrian),
            &[1, 2, 2, 1, 2, 2, 2],
            &[1, 2, 2, 1, 2, 2, 2],
        ),
        (
            HarmonicMinor,
            Some(Mode::HarmonicMinor),
            &[2, 1, 2, 2, 1, 3, 1],
            &[2, 1, 2, 2, 1, 3, 1],
        ),
        (
            MelodicMinor,
            Some(Mode::MelodicMinor),
            &[2, 1, 2, 2, 2, 2, 1],
            &[2, 1, 2, 2, 1, 2, 2],
        ),
        (
            PentatonicMajor,
            Some(Mode::PentatonicMajor),
            &[2, 2, 3, 2, 3],
            &[2, 2, 3, 2, 3],
        ),
        (
            PentatonicMinor,
            Some(Mode::PentatonicMinor),
            &[3, 2, 2, 3, 2],
            &[3, 2, 2, 3, 2],
        ),
        (
            Blues,
            Some(Mode::Blues),
            &[3, 2, 1, 1, 3, 2],
            &[3, 2, 1, 1, 3, 2],
        ),
        (Chromatic, Some(Mode::Chromatic), &[1; 12], &[1; 12]),
        (WholeTone, Some(Mode::WholeTone), &[2; 6], &[2; 6]),
    ];

    for &(scale_type, mode, ascending_pattern, descending_pattern) in scales {
        for (direction, expected_pattern) in [
            (Ascending, ascending_pattern),
            (Descending, descending_pattern),
        ] {
            for root in roots_with_common_accidentals() {
                let scale = Scale::new(scale_type, root, 4, mode, direction).unwrap();
                let actual_pattern: Vec<u8> = scale
                    .intervals
                    .iter()
                    .map(|interval| interval.semitone_count)
                    .collect();
                assert_eq!(
                    actual_pattern, expected_pattern,
                    "Wrong pattern for {:?}",
                    scale
                );
                assert_eq!(actual_pattern.iter().sum::<u8>(), 12);

                let notes = scale.notes();
                assert_eq!(notes.len(), expected_pattern.len() + 1);
                assert_eq!(notes.first().unwrap().pitch, root);
                assert_eq!(notes.last().unwrap().pitch, root);

                let traversed: Vec<u8> = match direction {
                    Ascending => expected_pattern.to_vec(),
                    Descending => expected_pattern.iter().rev().copied().collect(),
                };
                for (window, expected) in notes.windows(2).zip(traversed) {
                    let actual = match direction {
                        Ascending => {
                            (window[1].pitch.into_u8() + 12 - window[0].pitch.into_u8()) % 12
                        }
                        Descending => {
                            (window[0].pitch.into_u8() + 12 - window[1].pitch.into_u8()) % 12
                        }
                    };
                    assert_eq!(actual, expected, "Wrong traversal for {:?}", scale);
                }
            }
        }
    }
}

#[test]
fn every_heptatonic_scale_uses_each_letter_once_before_the_octave() {
    let families = [
        (ScaleType::Diatonic, Some(Mode::Ionian)),
        (ScaleType::Diatonic, Some(Mode::Dorian)),
        (ScaleType::Diatonic, Some(Mode::Phrygian)),
        (ScaleType::Diatonic, Some(Mode::Lydian)),
        (ScaleType::Diatonic, Some(Mode::Mixolydian)),
        (ScaleType::Diatonic, Some(Mode::Aeolian)),
        (ScaleType::Diatonic, Some(Mode::Locrian)),
        (ScaleType::HarmonicMinor, Some(Mode::HarmonicMinor)),
        (ScaleType::MelodicMinor, Some(Mode::MelodicMinor)),
    ];

    for (scale_type, mode) in families {
        for direction in [Direction::Ascending, Direction::Descending] {
            for root in roots_with_common_accidentals() {
                let notes = Scale::new(scale_type, root, 4, mode, direction)
                    .unwrap()
                    .notes();
                let mut letters = notes[..7]
                    .iter()
                    .map(|note| letter_index(note.pitch.letter))
                    .collect::<Vec<_>>();
                letters.sort_unstable();
                assert_eq!(letters, vec![0, 1, 2, 3, 4, 5, 6]);
            }
        }
    }
}

#[test]
fn all_standard_major_keys_have_conventional_spellings() {
    let keys = [
        "C D E F G A B C",
        "G A B C D E Fs G",
        "D E Fs G A B Cs D",
        "A B Cs D E Fs Gs A",
        "E Fs Gs A B Cs Ds E",
        "B Cs Ds E Fs Gs As B",
        "Fs Gs As B Cs Ds Es Fs",
        "Cs Ds Es Fs Gs As Bs Cs",
        "F G A Bb C D E F",
        "Bb C D Eb F G A Bb",
        "Eb F G Ab Bb C D Eb",
        "Ab Bb C Db Eb F G Ab",
        "Db Eb F Gb Ab Bb C Db",
        "Gb Ab Bb Cb Db Eb F Gb",
        "Cb Db Eb Fb Gb Ab Bb Cb",
    ];

    for expected in keys {
        let expected: Vec<Pitch> = expected
            .split_whitespace()
            .map(|spelling| Pitch::from_str(spelling).unwrap())
            .collect();
        let scale = Scale::new(
            ScaleType::Diatonic,
            expected[0],
            4,
            Some(Mode::Ionian),
            Direction::Ascending,
        )
        .unwrap();
        assert_eq!(
            scale
                .notes()
                .iter()
                .map(|note| note.pitch)
                .collect::<Vec<_>>(),
            expected
        );
    }
}

#[test]
fn classical_minor_variants_raise_and_lower_the_correct_degrees() {
    let cases = [
        (
            ScaleType::Diatonic,
            Some(Mode::Aeolian),
            Direction::Ascending,
            "C D Eb F G Ab Bb C",
        ),
        (
            ScaleType::HarmonicMinor,
            Some(Mode::HarmonicMinor),
            Direction::Ascending,
            "C D Eb F G Ab B C",
        ),
        (
            ScaleType::MelodicMinor,
            Some(Mode::MelodicMinor),
            Direction::Ascending,
            "C D Eb F G A B C",
        ),
        (
            ScaleType::MelodicMinor,
            Some(Mode::MelodicMinor),
            Direction::Descending,
            "C Bb Ab G F Eb D C",
        ),
    ];

    for (scale_type, mode, direction, expected) in cases {
        let scale = Scale::new(
            scale_type,
            Pitch::from_str("C").unwrap(),
            4,
            mode,
            direction,
        )
        .unwrap();
        let expected: Vec<u8> = expected
            .split_whitespace()
            .map(|spelling| Pitch::from_str(spelling).unwrap().into_u8())
            .collect();
        assert_eq!(pitch_classes(&scale.notes()), expected);
    }
}

#[test]
fn interval_transposition_preserves_written_size_and_supports_negative_octaves() {
    use IntervalNumber::*;
    use IntervalQuality::*;

    let f = Pitch::from_str("F").unwrap();
    assert_eq!(
        Pitch::from_interval(f, Interval::from_semitone(3).unwrap()),
        Pitch::from_str("Ab").unwrap()
    );

    let augmented_fourth =
        Interval::new(6, Augmented, Fourth, Some(theory::interval::Step::Tritone));
    let diminished_fifth =
        Interval::new(6, Diminished, Fifth, Some(theory::interval::Step::Tritone));
    let c = Pitch::from_str("C").unwrap();
    assert_eq!(
        Pitch::from_interval(c, augmented_fourth),
        Pitch::from_str("Fs").unwrap()
    );
    assert_eq!(
        Pitch::from_interval(c, diminished_fifth),
        Pitch::from_str("Gb").unwrap()
    );

    let c_zero = theory::note::Note::new(c, 0);
    assert_eq!(
        Interval::from_semitone(12)
            .unwrap()
            .second_note_down_from(c_zero),
        theory::note::Note::new(c, -1)
    );
    let descending = Scale::new(
        ScaleType::Diatonic,
        c,
        0,
        Some(Mode::Ionian),
        Direction::Descending,
    )
    .unwrap()
    .notes();
    assert_eq!(descending.last().unwrap().octave, -1);
}

#[test]
fn scale_type_mapping_is_exhaustive() {
    let cases = [
        (Mode::Ionian, ScaleType::Diatonic),
        (Mode::Dorian, ScaleType::Diatonic),
        (Mode::Phrygian, ScaleType::Diatonic),
        (Mode::Lydian, ScaleType::Diatonic),
        (Mode::Mixolydian, ScaleType::Diatonic),
        (Mode::Aeolian, ScaleType::Diatonic),
        (Mode::Locrian, ScaleType::Diatonic),
        (Mode::HarmonicMinor, ScaleType::HarmonicMinor),
        (Mode::MelodicMinor, ScaleType::MelodicMinor),
        (Mode::PentatonicMajor, ScaleType::PentatonicMajor),
        (Mode::PentatonicMinor, ScaleType::PentatonicMinor),
        (Mode::Blues, ScaleType::Blues),
        (Mode::Chromatic, ScaleType::Chromatic),
        (Mode::WholeTone, ScaleType::WholeTone),
    ];
    for (mode, expected) in cases {
        assert_eq!(ScaleType::from_mode(mode), expected);
        assert_eq!(ScaleType::from(mode), expected);
    }
}

#[test]
fn whole_tone_transpositions_produce_exactly_two_pitch_class_collections() {
    use std::collections::HashSet;

    let collections: HashSet<Vec<u8>> = (0..12)
        .map(|root| {
            let notes = Scale::new(
                ScaleType::WholeTone,
                Pitch::from_u8(root),
                4,
                Some(Mode::WholeTone),
                Direction::Ascending,
            )
            .unwrap()
            .notes();
            let mut pitches = pitch_classes(&notes[..6]);
            pitches.sort_unstable();
            pitches
        })
        .collect();
    assert_eq!(collections.len(), 2);
}

#[test]
fn public_parsers_do_not_panic_on_arbitrary_short_input() {
    let alphabet = ['C', '#', 'b', '/', ' ', '7', 'x', '?'];
    for first in alphabet {
        for second in alphabet {
            for third in alphabet {
                let input = [first, second, third].iter().collect::<String>();
                assert!(std::panic::catch_unwind(|| Chord::parse(&input)).is_ok());
                assert!(std::panic::catch_unwind(|| Scale::from_regex(&input)).is_ok());
                assert!(std::panic::catch_unwind(|| Pitch::from_str(&input)).is_ok());
            }
        }
    }
}
