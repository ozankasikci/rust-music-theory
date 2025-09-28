extern crate rust_music_theory as theory;
use theory::chord::{Number::*, Quality::*, *};
use theory::note::{PitchSymbol::*, *};

fn assert_notes(symbols: &[PitchSymbol], notes: Vec<Note>) {
    for (i, symbol) in symbols.iter().enumerate() {
        assert_eq!(Pitch::from(*symbol), notes[i].pitch);
    }
}

#[cfg(test)]
mod chord_tests {
    use super::*;

    #[test]
    fn test_all_chords_in_c() {
        let chord_tuples = [
            ((C, Major, Triad), vec![C, E, G]),
            ((C, Minor, Triad), vec![C, Ds, G]),
            ((C, Augmented, Triad), vec![C, E, Gs]),
            ((C, Diminished, Triad), vec![C, Ds, Fs]),
            ((C, Suspended2, Triad), vec![C, D, G]),
            ((C, Suspended4, Triad), vec![C, F, G]),
            ((C, Major, Seventh), vec![C, E, G, B]),
            ((C, Minor, Seventh), vec![C, Ds, G, As]),
            ((C, Augmented, Seventh), vec![C, E, Gs, As]),
            ((C, Augmented, MajorSeventh), vec![C, E, Gs, B]),
            ((C, Diminished, Seventh), vec![C, Ds, Fs, A]),
            ((C, HalfDiminished, Seventh), vec![C, Ds, Fs, As]),
            ((C, Minor, MajorSeventh), vec![C, Ds, G, B]),
            ((C, Dominant, Seventh), vec![C, E, G, As]),
        ];

        for (chord, pitches) in chord_tuples.iter() {
            let symbols = &mut pitches.clone();
            for inversion in 0..pitches.len() {
                assert_notes(
                    &symbols,
                    Chord::with_inversion(Pitch::from(chord.0), chord.1, chord.2, inversion as u8).notes(),
                );
                symbols.rotate_left(1);
            }
        }
    }

    #[test]
    fn test_inversion_octaves() {
        let chord_desc = (G, Major, Ninth);
        let octaves = [
            [4u8, 4, 5, 5, 5],
            [4, 5, 5, 5, 6],
            [4, 4, 4, 5, 5],
            [4, 4, 5, 5, 6],
            [4, 5, 5, 6, 6],
        ];
        for inversion in 0..octaves[0].len() {
            let notes =
                Chord::with_inversion(Pitch::from(chord_desc.0), chord_desc.1, chord_desc.2, inversion as u8)
                    .notes();
            assert_eq!(
                notes
                    .into_iter()
                    .map(|note| note.octave)
                    .collect::<Vec<u8>>(),
                octaves[inversion]
            );
        }
    }

    #[test]
    fn test_regex() {
        let chord = Chord::from_regex("F major");
        assert!(chord.is_ok());
        let chord = chord.unwrap();
        assert_notes(&vec![F, A, C], chord.notes());
        assert_eq!(chord.inversion, 0);
    }

    #[test]
    fn test_inversion_regex() {
        let chord = Chord::from_regex("F/C");
        let chord_num = Chord::from_regex("F/2");
        assert!(chord.is_ok());
        assert!(chord_num.is_ok());
        let chord = chord.unwrap();
        let chord_num = chord_num.unwrap();
        assert_notes(&vec![C, F, A], chord.notes());
        assert_notes(&vec![C, F, A], chord_num.notes());
        assert_eq!(chord.inversion, 2);
        assert_eq!(chord_num.inversion, 2);
    }

    #[test]
    fn test_chord_from_string() {
        let c = Pitch::from_str("C").unwrap();
        let chord_tuples = [
            ((c, Major, Triad), "C E G"),
            ((c, Minor, Triad), "C Ds G"),
            ((c, Augmented, Triad), "C E Gs"),
            ((c, Diminished, Triad), "C Ds Fs"),
            ((c, Suspended2, Triad), "C D G"),
            ((c, Suspended4, Triad), "C F G"),
            ((c, Major, Seventh), "C E G B"),
            ((c, Minor, Seventh), "C Ds G As"),
            ((c, Augmented, Seventh), "C E Gs As"),
            ((c, Augmented, MajorSeventh), "C, E, Gs, B"),
            ((c, Diminished, Seventh), "C, Ds, Fs, A"),
            ((c, HalfDiminished, Seventh), "C, Ds, Fs, As"),
            ((c, Minor, MajorSeventh), "C, Ds, G, B"),
            ((c, Dominant, Seventh), "C, E, G, As"),
        ];

        for chord_pair in chord_tuples.iter() {
            let chord = Chord::from_string(chord_pair.1).unwrap();
            let (root, quality, number) = (chord.root, chord.quality, chord.number);
            assert_eq!((root, quality, number), (chord_pair.0));
        }
    }

    #[test]
    fn test_invalid_chord_regex() {
        // Test definitely invalid chord strings
        let invalid_chords = vec![
            "",
            "123",
            "!@#$%",
        ];

        for invalid_chord in invalid_chords {
            assert!(
                Chord::from_regex(invalid_chord).is_err(),
                "Expected error for: {}",
                invalid_chord
            );
        }
    }

    #[test]
    fn test_chord_from_interval_errors() {
        // Test unknown interval patterns
        let root = Pitch::new(NoteLetter::C, 0);

        // Test with invalid interval patterns
        let invalid_patterns = vec![
            vec![1, 2],           // Too few intervals for any chord
            vec![5, 5, 5, 5, 5],  // Nonsensical pattern
            vec![13, 14, 15],     // Intervals too large
        ];

        for pattern in invalid_patterns {
            let result = Chord::from_interval(root, &pattern);
            assert!(
                result.is_err(),
                "Expected error for interval pattern: {:?}",
                pattern
            );
        }
    }

    #[test]
    fn test_chord_default() {
        let default_chord = Chord::default();

        // Verify default chord properties
        assert_eq!(default_chord.root, Pitch::new(NoteLetter::C, 0));
        assert_eq!(default_chord.octave, 4);
        assert_eq!(default_chord.quality, Quality::Major);
        assert_eq!(default_chord.number, Number::Triad);
        assert_eq!(default_chord.inversion, 0);

        // Default chord has empty intervals, so we need to create one properly
        let c_major = Chord::new(
            Pitch::new(NoteLetter::C, 0),
            Quality::Major,
            Number::Triad
        );
        let notes = c_major.notes();
        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::C, 0));
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::E, 0));
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::G, 0));
    }

    #[test]
    fn test_chord_from_interval() {
        let root = Pitch::new(NoteLetter::C, 0);

        // Major triad intervals: [4, 3]
        let major_triad = Chord::from_interval(root, &[4, 3]).unwrap();
        let notes = major_triad.notes();
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].pitch, root);
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::E, 0));
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::G, 0));

        // Minor triad intervals: [3, 4]
        let minor_triad = Chord::from_interval(root, &[3, 4]).unwrap();
        let notes = minor_triad.notes();
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].pitch, root);
        assert_eq!(notes[1].pitch.into_u8(), 3); // Eb
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::G, 0));

        // Dominant seventh: [4, 3, 3]
        let dom7 = Chord::from_interval(root, &[4, 3, 3]).unwrap();
        let notes = dom7.notes();
        assert_eq!(notes.len(), 4);
        assert_eq!(notes[3].pitch.into_u8(), 10); // Bb
    }

    #[test]
    fn test_chord_intervals() {
        // Test that chord_intervals returns correct intervals for various chord types
        let major_triad_intervals = Chord::chord_intervals(Quality::Major, Number::Triad);
        assert_eq!(major_triad_intervals.len(), 2);
        assert_eq!(major_triad_intervals[0].semitone_count, 4);
        assert_eq!(major_triad_intervals[1].semitone_count, 3);

        let minor_seventh_intervals = Chord::chord_intervals(Quality::Minor, Number::Seventh);
        assert_eq!(minor_seventh_intervals.len(), 3);
        assert_eq!(minor_seventh_intervals[0].semitone_count, 3);
        assert_eq!(minor_seventh_intervals[1].semitone_count, 4);
        assert_eq!(minor_seventh_intervals[2].semitone_count, 3);

        let dim_triad_intervals = Chord::chord_intervals(Quality::Diminished, Number::Triad);
        assert_eq!(dim_triad_intervals.len(), 2);
        assert_eq!(dim_triad_intervals[0].semitone_count, 3);
        assert_eq!(dim_triad_intervals[1].semitone_count, 3);
    }
}
