extern crate rust_music_theory as theory;

use theory::note::{NoteLetter, Notes, Pitch};
use theory::scale::{Direction, Mode, Scale, ScaleType};

fn pitches(scale: &Scale) -> Vec<String> {
    scale
        .notes()
        .iter()
        .map(|note| note.pitch.to_string())
        .collect()
}

fn steps(scale: &Scale) -> Vec<u8> {
    scale
        .intervals
        .iter()
        .map(|interval| interval.semitone_count)
        .collect()
}

#[test]
fn harmonic_minor_modes_have_reference_formulas_and_spellings() {
    let cases = [
        (
            Mode::HarmonicMinor,
            vec![2, 1, 2, 2, 1, 3, 1],
            vec!["C", "D", "Eb", "F", "G", "Ab", "B", "C"],
        ),
        (
            Mode::LocrianNatural6,
            vec![1, 2, 2, 1, 3, 1, 2],
            vec!["C", "Db", "Eb", "F", "Gb", "A", "Bb", "C"],
        ),
        (
            Mode::IonianSharp5,
            vec![2, 2, 1, 3, 1, 2, 1],
            vec!["C", "D", "E", "F", "G#", "A", "B", "C"],
        ),
        (
            Mode::DorianSharp4,
            vec![2, 1, 3, 1, 2, 1, 2],
            vec!["C", "D", "Eb", "F#", "G", "A", "Bb", "C"],
        ),
        (
            Mode::PhrygianDominant,
            vec![1, 3, 1, 2, 1, 2, 2],
            vec!["C", "Db", "E", "F", "G", "Ab", "Bb", "C"],
        ),
        (
            Mode::LydianSharp2,
            vec![3, 1, 2, 1, 2, 2, 1],
            vec!["C", "D#", "E", "F#", "G", "A", "B", "C"],
        ),
        (
            Mode::UltraLocrian,
            vec![1, 2, 1, 2, 2, 1, 3],
            vec!["C", "Db", "Eb", "Fb", "Gb", "Ab", "Bbb", "C"],
        ),
    ];

    for (mode, expected_steps, expected_pitches) in cases {
        let scale = Scale::new(
            ScaleType::HarmonicMinor,
            Pitch::new(NoteLetter::C, 0),
            4,
            Some(mode),
            Direction::Ascending,
        )
        .unwrap();
        assert_eq!(
            steps(&scale),
            expected_steps,
            "wrong formula for {:?}",
            mode
        );
        assert_eq!(
            pitches(&scale),
            expected_pitches,
            "wrong spelling for {:?}",
            mode
        );
    }
}

#[test]
fn harmonic_minor_metadata_and_aliases_are_normalized() {
    let cases = [
        (
            Mode::LocrianNatural6,
            "Locrian natural 6",
            "locrian_natural_6",
        ),
        (Mode::IonianSharp5, "Ionian #5", "ionian_sharp_5"),
        (Mode::DorianSharp4, "Dorian #4", "dorian_sharp_4"),
        (
            Mode::PhrygianDominant,
            "Phrygian Dominant",
            "phrygian_dominant",
        ),
        (Mode::LydianSharp2, "Lydian #2", "lydian_sharp_2"),
        (Mode::UltraLocrian, "Ultralocrian", "ultralocrian"),
    ];

    for (mode, canonical, api) in cases {
        assert_eq!(mode.scale_type(), ScaleType::HarmonicMinor);
        assert_eq!(mode.canonical_name(), canonical);
        assert_eq!(mode.api_name(), api);
        assert_eq!(api.parse::<Mode>().unwrap(), mode);
        assert_eq!(canonical.parse::<Mode>().unwrap(), mode);
    }

    for (alias, expected) in [
        ("Locrian ♮ 6", Mode::LocrianNatural6),
        ("major sharp 5", Mode::IonianSharp5),
        ("Ukrainian-Dorian", Mode::DorianSharp4),
        ("Spanish", Mode::PhrygianDominant),
        ("Lydian ♯9", Mode::LydianSharp2),
        ("super_locrian diminished", Mode::UltraLocrian),
    ] {
        assert_eq!(alias.parse::<Mode>().unwrap(), expected, "alias {alias}");
    }
}

#[test]
fn harmonic_minor_modes_descend_through_the_same_pitch_collection() {
    for mode in [
        Mode::HarmonicMinor,
        Mode::LocrianNatural6,
        Mode::IonianSharp5,
        Mode::DorianSharp4,
        Mode::PhrygianDominant,
        Mode::LydianSharp2,
        Mode::UltraLocrian,
    ] {
        let ascending = Scale::new(
            ScaleType::HarmonicMinor,
            Pitch::new(NoteLetter::C, 0),
            4,
            Some(mode),
            Direction::Ascending,
        )
        .unwrap();
        let descending = Scale::new(
            ScaleType::HarmonicMinor,
            Pitch::new(NoteLetter::C, 0),
            4,
            Some(mode),
            Direction::Descending,
        )
        .unwrap();

        let mut ascending_classes = ascending
            .notes()
            .into_iter()
            .take(7)
            .map(|note| note.pitch.into_u8())
            .collect::<Vec<_>>();
        let mut descending_classes = descending
            .notes()
            .into_iter()
            .take(7)
            .map(|note| note.pitch.into_u8())
            .collect::<Vec<_>>();
        ascending_classes.sort_unstable();
        descending_classes.sort_unstable();
        assert_eq!(ascending_classes, descending_classes, "mode {:?}", mode);
    }
}

#[test]
fn melodic_minor_modes_have_reference_formulas_and_spellings() {
    let cases = [
        (
            Mode::MelodicMinor,
            vec![2, 1, 2, 2, 2, 2, 1],
            vec!["C", "D", "Eb", "F", "G", "A", "B", "C"],
        ),
        (
            Mode::DorianFlat2,
            vec![1, 2, 2, 2, 2, 1, 2],
            vec!["C", "Db", "Eb", "F", "G", "A", "Bb", "C"],
        ),
        (
            Mode::LydianAugmented,
            vec![2, 2, 2, 2, 1, 2, 1],
            vec!["C", "D", "E", "F#", "G#", "A", "B", "C"],
        ),
        (
            Mode::LydianDominant,
            vec![2, 2, 2, 1, 2, 1, 2],
            vec!["C", "D", "E", "F#", "G", "A", "Bb", "C"],
        ),
        (
            Mode::MixolydianFlat6,
            vec![2, 2, 1, 2, 1, 2, 2],
            vec!["C", "D", "E", "F", "G", "Ab", "Bb", "C"],
        ),
        (
            Mode::LocrianSharp2,
            vec![2, 1, 2, 1, 2, 2, 2],
            vec!["C", "D", "Eb", "F", "Gb", "Ab", "Bb", "C"],
        ),
        (
            Mode::Altered,
            vec![1, 2, 1, 2, 2, 2, 2],
            vec!["C", "Db", "Eb", "Fb", "Gb", "Ab", "Bb", "C"],
        ),
    ];

    for (mode, expected_steps, expected_pitches) in cases {
        let scale = Scale::new(
            ScaleType::MelodicMinor,
            Pitch::new(NoteLetter::C, 0),
            4,
            Some(mode),
            Direction::Ascending,
        )
        .unwrap();
        assert_eq!(
            steps(&scale),
            expected_steps,
            "wrong formula for {:?}",
            mode
        );
        assert_eq!(
            pitches(&scale),
            expected_pitches,
            "wrong spelling for {:?}",
            mode
        );
    }
}

#[test]
fn melodic_minor_metadata_and_aliases_are_normalized() {
    let cases = [
        (Mode::DorianFlat2, "Dorian b2", "dorian_flat_2"),
        (
            Mode::LydianAugmented,
            "Lydian Augmented",
            "lydian_augmented",
        ),
        (Mode::LydianDominant, "Lydian Dominant", "lydian_dominant"),
        (Mode::MixolydianFlat6, "Mixolydian b6", "mixolydian_flat_6"),
        (Mode::LocrianSharp2, "Locrian #2", "locrian_sharp_2"),
        (Mode::Altered, "Altered", "altered"),
    ];

    for (mode, canonical, api) in cases {
        assert_eq!(mode.scale_type(), ScaleType::MelodicMinor);
        assert_eq!(mode.canonical_name(), canonical);
        assert_eq!(mode.api_name(), api);
        assert_eq!(api.parse::<Mode>().unwrap(), mode);
        assert_eq!(canonical.parse::<Mode>().unwrap(), mode);
    }

    for (alias, expected) in [
        ("Dorian ♭2", Mode::DorianFlat2),
        ("Phrygian sharp 6", Mode::DorianFlat2),
        ("Lydian ♯5", Mode::LydianAugmented),
        ("Overtone", Mode::LydianDominant),
        ("Hindu", Mode::MixolydianFlat6),
        ("Half-Diminished", Mode::LocrianSharp2),
        ("diminished_whole_tone", Mode::Altered),
    ] {
        assert_eq!(alias.parse::<Mode>().unwrap(), expected, "alias {alias}");
    }
}

#[test]
fn classical_base_descent_and_jazz_derived_modes_are_distinct() {
    let base = Scale::new(
        ScaleType::MelodicMinor,
        Pitch::new(NoteLetter::C, 0),
        4,
        Some(Mode::MelodicMinor),
        Direction::Descending,
    )
    .unwrap();
    assert_eq!(
        pitches(&base),
        vec!["C", "Bb", "Ab", "G", "F", "Eb", "D", "C"]
    );

    for mode in [
        Mode::DorianFlat2,
        Mode::LydianAugmented,
        Mode::LydianDominant,
        Mode::MixolydianFlat6,
        Mode::LocrianSharp2,
        Mode::Altered,
    ] {
        let ascending = Scale::new(
            ScaleType::MelodicMinor,
            Pitch::new(NoteLetter::C, 0),
            4,
            Some(mode),
            Direction::Ascending,
        )
        .unwrap();
        let descending = Scale::new(
            ScaleType::MelodicMinor,
            Pitch::new(NoteLetter::C, 0),
            4,
            Some(mode),
            Direction::Descending,
        )
        .unwrap();
        let mut ascending_classes = ascending
            .notes()
            .into_iter()
            .take(7)
            .map(|note| note.pitch.into_u8())
            .collect::<Vec<_>>();
        let mut descending_classes = descending
            .notes()
            .into_iter()
            .take(7)
            .map(|note| note.pitch.into_u8())
            .collect::<Vec<_>>();
        ascending_classes.sort_unstable();
        descending_classes.sort_unstable();
        assert_eq!(ascending_classes, descending_classes, "mode {:?}", mode);
    }
}

#[test]
fn incompatible_mode_families_return_a_structured_error() {
    let error = Scale::new(
        ScaleType::Diatonic,
        Pitch::new(NoteLetter::C, 0),
        4,
        Some(Mode::Altered),
        Direction::Ascending,
    )
    .unwrap_err();
    assert_eq!(
        error,
        theory::scale::ScaleError::IncompatibleMode {
            scale_type: ScaleType::Diatonic,
            mode: Mode::Altered,
        }
    );
    assert_eq!(
        error.to_string(),
        "Mode Altered belongs to MelodicMinor, not Diatonic"
    );
}

#[test]
fn every_minor_mode_preserves_letters_octaves_and_midi_ordering() {
    let modes = [
        Mode::HarmonicMinor,
        Mode::LocrianNatural6,
        Mode::IonianSharp5,
        Mode::DorianSharp4,
        Mode::PhrygianDominant,
        Mode::LydianSharp2,
        Mode::UltraLocrian,
        Mode::MelodicMinor,
        Mode::DorianFlat2,
        Mode::LydianAugmented,
        Mode::LydianDominant,
        Mode::MixolydianFlat6,
        Mode::LocrianSharp2,
        Mode::Altered,
    ];
    let roots = [
        Pitch::new(NoteLetter::C, 0),
        Pitch::new(NoteLetter::F, 1),
        Pitch::new(NoteLetter::B, -1),
        Pitch::new(NoteLetter::C, 2),
        Pitch::new(NoteLetter::F, -2),
    ];

    for mode in modes {
        for root in roots {
            for direction in [Direction::Ascending, Direction::Descending] {
                let scale = Scale::new(mode.scale_type(), root, 4, Some(mode), direction).unwrap();
                let notes = scale.notes();
                let mut letters = notes[..7]
                    .iter()
                    .map(|note| note.pitch.letter as u8)
                    .collect::<Vec<_>>();
                letters.sort_unstable();
                letters.dedup();
                assert_eq!(letters.len(), 7, "letters for {:?} from {:?}", mode, root);
                for pair in notes.windows(2) {
                    let ordered = match direction {
                        Direction::Ascending => pair[0].midi_pitch() < pair[1].midi_pitch(),
                        Direction::Descending => pair[0].midi_pitch() > pair[1].midi_pitch(),
                    };
                    assert!(
                        ordered,
                        "MIDI order for {:?} from {:?} {:?}: {:?}",
                        mode, root, direction, notes
                    );
                }
            }
        }
    }
}

#[test]
fn all_mode_names_are_unique_and_round_trip() {
    let modes = [
        Mode::Ionian,
        Mode::Dorian,
        Mode::Phrygian,
        Mode::Lydian,
        Mode::Mixolydian,
        Mode::Aeolian,
        Mode::Locrian,
        Mode::HarmonicMinor,
        Mode::LocrianNatural6,
        Mode::IonianSharp5,
        Mode::DorianSharp4,
        Mode::PhrygianDominant,
        Mode::LydianSharp2,
        Mode::UltraLocrian,
        Mode::MelodicMinor,
        Mode::DorianFlat2,
        Mode::LydianAugmented,
        Mode::LydianDominant,
        Mode::MixolydianFlat6,
        Mode::LocrianSharp2,
        Mode::Altered,
    ];
    let mut canonical = std::collections::HashSet::new();
    let mut api = std::collections::HashSet::new();
    for mode in modes {
        assert!(canonical.insert(mode.canonical_name()));
        assert!(api.insert(mode.api_name()));
        assert_eq!(mode.canonical_name().parse::<Mode>().unwrap(), mode);
        assert_eq!(mode.api_name().parse::<Mode>().unwrap(), mode);
    }
}
