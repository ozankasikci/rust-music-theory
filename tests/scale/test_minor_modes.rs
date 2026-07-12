extern crate rust_music_theory as theory;

use theory::note::{Note, NoteLetter, Notes, Pitch};
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

#[allow(dead_code)]
fn assert_notes_are_strictly_ordered(notes: &[Note], direction: Direction) {
    for pair in notes.windows(2) {
        match direction {
            Direction::Ascending => assert!(pair[0].midi_pitch() < pair[1].midi_pitch()),
            Direction::Descending => assert!(pair[0].midi_pitch() > pair[1].midi_pitch()),
        }
    }
}
