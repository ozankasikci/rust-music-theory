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
            ((Diatonic, Some(Ionian)), vec![C, D, E, F, G, A, B, C]),
            ((Diatonic, Some(Dorian)), vec![C, D, Eb, F, G, A, Bb, C]),  // Uses Bb major key signature (2 flats)
            ((Diatonic, Some(Phrygian)), vec![C, Db, Eb, F, G, Ab, Bb, C]),  // Uses Ab major key signature (4 flats)
            ((Diatonic, Some(Lydian)), vec![C, D, E, Fs, G, A, B, C]),  // Uses G major key signature (1 sharp)
            ((Diatonic, Some(Mixolydian)), vec![C, D, E, F, G, A, Bb, C]),  // Uses F major key signature (1 flat)
            ((Diatonic, Some(Aeolian)), vec![C, D, Eb, F, G, Ab, Bb, C]),  // Uses Eb major key signature (3 flats)
            ((Diatonic, Some(Locrian)), vec![C, Db, Eb, F, Gb, Ab, Bb, C]),  // Uses Db major key signature (5 flats)
            (
                (ScaleType::HarmonicMinor, None),
                vec![C, D, Ds, F, G, Gs, B, C],  // Uses C major sharp preference
            ),
            (
                (ScaleType::MelodicMinor, None),
                vec![C, D, Ds, F, G, A, B, C],  // Uses C major sharp preference
            )
        ];

        for (scale_tuple, pitches) in scale_tuples.iter() {
            let (scale_type, mode) = scale_tuple;
            let scale_ascending =
                Scale::new(*scale_type, Pitch::from(C), 4, *mode, Direction::Ascending).unwrap();
            assert_notes(pitches, scale_ascending.notes());

            let scale_descending =
                Scale::new(*scale_type, Pitch::from(C), 4, *mode, Direction::Descending).unwrap();
            let mut pitches_descending = pitches.clone();
            pitches_descending.reverse();
            assert_notes(&pitches_descending, scale_descending.notes());

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
}
