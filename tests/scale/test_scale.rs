extern crate rust_music_theory as theory;
use theory::note::{PitchSymbol::*, *};
use theory::scale::Mode::*;
use theory::scale::ScaleType::*;
use theory::scale::*;
use theory::interval::Interval;

fn assert_notes(symbols: &[PitchSymbol], notes: Vec<Note>) {
    for (i, symbol) in symbols.iter().enumerate() {
        assert_eq!(Pitch::from(*symbol), notes[i].pitch);
    }
}

#[cfg(test)]
mod scale_tests {
    use super::*;

    #[test]
    fn test_all_scales_in_c() {
        let scale_tuples = [
            ((Diatonic, Some(Ionian)), vec![C, D, E, F, G, A, B, C], vec![C, B, A, G, F, E, D, C]),
            ((Diatonic, Some(Dorian)), vec![C, D, Eb, F, G, A, Bb, C], vec![C, Bb, A, G, F, Eb, D, C]),
            ((Diatonic, Some(Phrygian)), vec![C, Db, Eb, F, G, Ab, Bb, C], vec![C, Bb, Ab, G, F, Eb, Db, C]),
            ((Diatonic, Some(Lydian)), vec![C, D, E, Fs, G, A, B, C], vec![C, B, A, G, Fs, E, D, C]),
            ((Diatonic, Some(Mixolydian)), vec![C, D, E, F, G, A, Bb, C], vec![C, Bb, A, G, F, E, D, C]),
            ((Diatonic, Some(Aeolian)), vec![C, D, Eb, F, G, Ab, Bb, C], vec![C, Bb, Ab, G, F, Eb, D, C]),
            ((Diatonic, Some(Locrian)), vec![C, Db, Eb, F, Gb, Ab, Bb, C], vec![C, Bb, Ab, Gb, F, Eb, Db, C]),
            (
                (ScaleType::HarmonicMinor, None),
                vec![C, D, Eb, F, G, Ab, B, C],
                vec![C, B, Ab, G, F, Eb, D, C],
            ),
            (
                (ScaleType::MelodicMinor, None),
                vec![C, D, Eb, F, G, A, B, C],
                vec![C, Bb, Ab, G, F, Eb, D, C],
            )
        ];

        for (scale_tuple, ascending_pitches, descending_pitches) in scale_tuples.iter() {
            let (scale_type, mode) = scale_tuple;
            let scale_ascending =
                Scale::new(*scale_type, Pitch::from(C), 4, *mode, Direction::Ascending).unwrap();
            assert_notes(ascending_pitches, scale_ascending.notes());

            let scale_descending =
                Scale::new(*scale_type, Pitch::from(C), 4, *mode, Direction::Descending).unwrap();
            assert_notes(descending_pitches, scale_descending.notes());

            if scale_ascending.scale_type == Diatonic {
                if let Some(mode) = scale_ascending.mode {
                    assert!(mode.is_diatonic());
                }
            }
        }
    }

    #[test]
    fn test_octave_increment() {
        let scale = Scale::new(
            ScaleType::Diatonic,
            Pitch::new(NoteLetter::G, 0),
            5,
            Some(Mode::Mixolydian),
            Direction::Ascending,
        )
        .unwrap();

        for (i, note) in scale.notes().iter().enumerate() {
            assert_eq!(note.octave, if i <= 2 { 5 } else { 6 });
        }
    }

    #[test]
    fn test_absolute_intervals() {
        let scale = Scale::new(
            Diatonic,
            Pitch::new(NoteLetter::C, 0),
            4,
            Some(Ionian),
            Direction::Ascending,
        )
        .unwrap();
        let intervals = scale.absolute_intervals();
        assert_eq!(
            intervals,
            vec![
                Interval::from_semitone(0).unwrap(),
                Interval::from_semitone(2).unwrap(),
                Interval::from_semitone(4).unwrap(),
                Interval::from_semitone(5).unwrap(),
                Interval::from_semitone(7).unwrap(),
                Interval::from_semitone(9).unwrap(),
                Interval::from_semitone(11).unwrap(),
            ]
        );
    }

    #[test]
    fn test_enharmonic_scales() {
        // Structure: (note1_letter, note1_acc, note2_letter, note2_acc, mode, scale_type, description)
        let test_cases = vec![
            // Major scale (Ionian) enharmonic pairs
            (NoteLetter::C, 1, NoteLetter::D, -1, Some(Ionian), Diatonic, "C♯ major and D♭ major"),
            (NoteLetter::F, 1, NoteLetter::G, -1, Some(Ionian), Diatonic, "F♯ major and G♭ major"),
            (NoteLetter::B, 1, NoteLetter::C, 0, Some(Ionian), Diatonic, "B♯ major and C major"),
            (NoteLetter::E, 1, NoteLetter::F, 0, Some(Ionian), Diatonic, "E♯ major and F major"),
            
            // Minor scale (Aeolian) enharmonic pairs
            (NoteLetter::A, 1, NoteLetter::B, -1, Some(Aeolian), Diatonic, "A♯ minor and B♭ minor"),
            (NoteLetter::D, 1, NoteLetter::E, -1, Some(Aeolian), Diatonic, "D♯ minor and E♭ minor"),
            (NoteLetter::G, 1, NoteLetter::A, -1, Some(Aeolian), Diatonic, "G♯ minor and A♭ minor"),
            
            // Double accidentals
            (NoteLetter::C, 2, NoteLetter::D, 0, Some(Ionian), Diatonic, "C𝄪 major and D major"),
            (NoteLetter::F, -2, NoteLetter::E, -1, Some(Ionian), Diatonic, "F𝄫 major and E♭ major"),
            
            // Other modes
            (NoteLetter::D, 1, NoteLetter::E, -1, Some(Dorian), Diatonic, "D♯ dorian and E♭ dorian"),
            (NoteLetter::E, 1, NoteLetter::F, 0, Some(Phrygian), Diatonic, "E♯ phrygian and F phrygian"),
            (NoteLetter::F, 1, NoteLetter::G, -1, Some(Lydian), Diatonic, "F♯ lydian and G♭ lydian"),
            (NoteLetter::G, 1, NoteLetter::A, -1, Some(Mixolydian), Diatonic, "G♯ mixolydian and A♭ mixolydian"),
            (NoteLetter::B, 1, NoteLetter::C, 0, Some(Locrian), Diatonic, "B♯ locrian and C locrian"),
            
            // Melodic minor enharmonic pairs
            (NoteLetter::C, 1, NoteLetter::D, -1, None, ScaleType::MelodicMinor, "C♯ melodic minor and D♭ melodic minor"),
            (NoteLetter::F, 1, NoteLetter::G, -1, None, ScaleType::MelodicMinor, "F♯ melodic minor and G♭ melodic minor"),
        ];

        for (note1_letter, note1_acc, note2_letter, note2_acc, mode, scale_type, description) in test_cases {
            let scale1 = Scale::new(
                scale_type,
                Pitch::new(note1_letter, note1_acc),
                4,
                mode,
                Direction::Ascending,
            ).unwrap();

            let scale2 = Scale::new(
                scale_type,
                Pitch::new(note2_letter, note2_acc),
                4,
                mode,
                Direction::Ascending,
            ).unwrap();

            // Verify that the semitone values are the same
            let notes1: Vec<u8> = scale1.notes().iter().map(|n| n.pitch.into_u8()).collect();
            let notes2: Vec<u8> = scale2.notes().iter().map(|n| n.pitch.into_u8()).collect();
            assert_eq!(notes1, notes2, "{} should be enharmonically equivalent", description);

            // Verify intervals are consistent
            assert_eq!(
                scale1.absolute_intervals(),
                scale2.absolute_intervals(),
                "Intervals should be the same for {}",
                description
            );
        }
    }

    #[test]
    fn test_invalid_scale_regex() {
        // Test invalid scale strings
        let invalid_scales = vec![
            "",
            "invalid scale",
            "XYZ major",
            "C invalid mode",
            "123 scale",
            "!@#$%",
        ];

        for invalid_scale in invalid_scales {
            assert!(
                Scale::from_regex(invalid_scale).is_err(),
                "Expected error for: {}",
                invalid_scale
            );
        }
    }

    #[test]
    fn test_scale_default() {
        let default_scale = Scale::default();
        assert_eq!(default_scale.tonic, Pitch::new(NoteLetter::C, 0));
        assert_eq!(default_scale.octave, 0);
        assert_eq!(default_scale.scale_type, ScaleType::Diatonic);
        assert_eq!(default_scale.mode, Some(Mode::Ionian));
        assert_eq!(default_scale.direction, Direction::Ascending);
    }

    #[test]
    fn test_scale_descending() {
        let c_major_desc = Scale::from_regex_in_direction("C major", Direction::Descending).unwrap();
        let notes = c_major_desc.notes();

        // In descending order, starting from C going down
        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::C, 0));
        assert_eq!(notes[0].octave, 4);

        // Should go down to B in the lower octave
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::B, 0));
        assert_eq!(notes[1].octave, 3);
    }

    #[test]
    fn test_scale_new_with_modes() {
        // Test creating scales with different modes
        let root = Pitch::new(NoteLetter::D, 0);

        let dorian = Scale::new(
            ScaleType::Diatonic,
            root,
            4,
            Some(Mode::Dorian),
            Direction::Ascending,
        )
        .unwrap();

        let notes = dorian.notes();
        assert_eq!(notes[0].pitch, root);

        // D Dorian has the same notes as C major
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::F, 0)); // Natural F, not F#
    }
}
