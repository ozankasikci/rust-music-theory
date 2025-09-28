extern crate rust_music_theory as theory;
use theory::note::{Pitch, NoteLetter, Note};
use theory::scale::Direction;
use theory::interval::Interval;
use NoteLetter::*;

#[cfg(test)]
mod test_note {
    use super::*;

    #[test]
    fn test_pitch_from_str() {
        let table = vec![
            ("Cb", Pitch::new(C, -1)),
            ("C#", Pitch::new(C, 1)),
            ("C##", Pitch::new(C, 2)),
            ("D", Pitch::new(D, 0)),
            ("Db", Pitch::new(D, -1)),
            ("Dbb", Pitch::new(D, -2)),
            ("Ds", Pitch::new(D, 1)),
            ("E", Pitch::new(E, 0)),
            ("Es", Pitch::new(E, 1)),
            ("Eb", Pitch::new(E, -1)),
            ("F", Pitch::new(F, 0)),
            ("f", Pitch::new(F, 0)),
            ("Fb", Pitch::new(F, -1)),
            ("G", Pitch::new(G, 0)),
            ("Gb", Pitch::new(G, -1)),
            ("Gbb", Pitch::new(G, -2)),
            ("Gs", Pitch::new(G, 1)),
            ("Gs##s𝄪", Pitch::new(G, 6)),
            ("Gs#♯", Pitch::new(G, 3)),
            ("A", Pitch::new(A, 0)),
            ("As", Pitch::new(A, 1)),
            ("Ab", Pitch::new(A, -1)),
            ("B", Pitch::new(B, 0)),
            ("B♯", Pitch::new(B, 1)),
            ("Bb", Pitch::new(B, -1)),
        ];

        for (string, pitch) in table {
            let p = Pitch::from_str(string).unwrap();
            assert_eq!(p, pitch);
            assert_eq!(string.parse::<Pitch>().unwrap(), pitch);
        }
    }

    #[test]
    fn test_pitch_from_str_err() {
        for string in vec!["Ca", "Q", "Cb#", "B♯b#"] {
            assert!(Pitch::from_str(string).is_none());
        }
    }

    #[test]
    fn test_pitch_into_u8() {
        let table = vec![
            (Pitch::new(C, 0), 0),
            (Pitch::new(C, 1), 1),
            (Pitch::new(D, 0), 2),
            (Pitch::new(D, 1), 3),
            (Pitch::new(E, 0), 4),
            (Pitch::new(F, 0), 5),
            (Pitch::new(F, 1), 6),
            (Pitch::new(G, 0), 7),
            (Pitch::new(G, 1), 8),
            (Pitch::new(A, 0), 9),
            (Pitch::new(A, 1), 10),
            (Pitch::new(B, 0), 11),
        ];

        for (pitch, number) in table {
            let n = pitch.into_u8();
            assert_eq!(n, number);
        }
    }

    #[test]
    fn test_pitch_format() {
        assert_eq!(format!("{}", Pitch::new(C,2)), "C##");
        assert_eq!(format!("{}", Pitch::new(C,-2)), "Cbb");
        assert_eq!(format!("{}", Pitch::new(C,0)), "C");
    }

    #[test]
    fn test_enharmonic_ascending() {
        let table = vec![
            (0, NoteLetter::C),    // C
            (1, NoteLetter::C),    // C♯
            (2, NoteLetter::D),    // D
            (3, NoteLetter::D),    // D♯
            (4, NoteLetter::E),    // E
            (5, NoteLetter::F),    // F
            (6, NoteLetter::F),    // F♯
            (7, NoteLetter::G),    // G
            (8, NoteLetter::G),    // G♯
            (9, NoteLetter::A),    // A
            (10, NoteLetter::A),   // A♯
            (11, NoteLetter::B),   // B
        ];

        for (val, expected) in table {
            let pitch = Pitch::from_u8_with_direction(val, Direction::Ascending);
            let expected_pitch = match val % 12 {
                1 => Pitch::new(expected, 1),  // Sharp
                3 => Pitch::new(expected, 1),  // Sharp
                6 => Pitch::new(expected, 1),  // Sharp
                8 => Pitch::new(expected, 1),  // Sharp
                10 => Pitch::new(expected, 1), // Sharp
                _ => Pitch::new(expected, 0),  // Natural
            };
            assert_eq!(pitch, expected_pitch,
                "Expected {} but got {} for value {}",
                expected_pitch, pitch, val);
        }
    }

    #[test]
    fn test_enharmonic_descending() {
        let table = vec![
            (0, NoteLetter::C),    // C
            (1, NoteLetter::D),    // D♭
            (2, NoteLetter::D),    // D
            (3, NoteLetter::E),    // E♭
            (4, NoteLetter::E),    // E
            (5, NoteLetter::F),    // F
            (6, NoteLetter::G),    // G♭
            (7, NoteLetter::G),    // G
            (8, NoteLetter::A),    // A♭
            (9, NoteLetter::A),    // A
            (10, NoteLetter::B),   // B♭
            (11, NoteLetter::B),   // B
        ];

        for (val, expected) in table {
            let pitch = Pitch::from_u8_with_direction(val, Direction::Descending);
            let expected_pitch = match val % 12 {
                1 => Pitch::new(expected, -1),  // Flat
                3 => Pitch::new(expected, -1),  // Flat
                6 => Pitch::new(expected, -1),  // Flat
                8 => Pitch::new(expected, -1),  // Flat
                10 => Pitch::new(expected, -1), // Flat
                _ => Pitch::new(expected, 0),   // Natural
            };
            assert_eq!(pitch, expected_pitch,
                "Expected {} but got {} for value {}",
                expected_pitch, pitch, val);
        }
    }

    #[test]
    fn test_enharmonic_edge_cases() {
        // Test that B♯ and C are enharmonic
        assert_eq!(
            Pitch::new(NoteLetter::B, 1).into_u8() % 12,
            Pitch::new(NoteLetter::C, 0).into_u8()
        );

        // Test that C♭ and B are enharmonic
        assert_eq!(
            Pitch::new(NoteLetter::C, -1).into_u8() % 12,
            Pitch::new(NoteLetter::B, 0).into_u8()
        );

        // Test that E♯ and F are enharmonic
        assert_eq!(
            Pitch::new(NoteLetter::E, 1).into_u8() % 12,
            Pitch::new(NoteLetter::F, 0).into_u8()
        );

        // Test that F♭ and E are enharmonic
        assert_eq!(
            Pitch::new(NoteLetter::F, -1).into_u8() % 12,
            Pitch::new(NoteLetter::E, 0).into_u8()
        );
    }

    #[test]
    fn test_pitch_double_accidentals() {
        // Test that double sharps and double flats work correctly
        let c_double_sharp = Pitch::new(NoteLetter::C, 2);
        assert_eq!(c_double_sharp.into_u8(), 2); // C## = D

        let d_double_flat = Pitch::new(NoteLetter::D, -2);
        assert_eq!(d_double_flat.into_u8(), 0); // Dbb = C

        // Display formatting
        assert_eq!(format!("{}", c_double_sharp), "C##");
        assert_eq!(format!("{}", d_double_flat), "Dbb");
    }

    #[test]
    fn test_extreme_octaves() {
        // Test notes at octave boundaries
        let very_low = Note::new(Pitch::new(NoteLetter::C, 0), 0);
        assert_eq!(very_low.octave, 0);

        let very_high = Note::new(Pitch::new(NoteLetter::C, 0), 127);
        assert_eq!(very_high.octave, 127);

        // Test that intervals work across extreme octaves
        let major_third = Interval::from_semitone(4).unwrap();
        let high_e = major_third.second_note_from(very_high.clone());
        assert_eq!(high_e.pitch, Pitch::new(NoteLetter::E, 0));
    }

    #[test]
    fn test_multiple_accidentals() {
        // Test triple and quadruple accidentals
        let c_triple_sharp = Pitch::new(NoteLetter::C, 3);
        assert_eq!(c_triple_sharp.into_u8(), 3); // C### = Eb

        let g_triple_flat = Pitch::new(NoteLetter::G, -3);
        assert_eq!(g_triple_flat.into_u8(), 4); // Gbbb = E

        // Test extreme accidentals
        let f_quintuple_sharp = Pitch::new(NoteLetter::F, 5);
        assert_eq!(f_quintuple_sharp.into_u8(), 10); // F##### = Bb
    }
}
