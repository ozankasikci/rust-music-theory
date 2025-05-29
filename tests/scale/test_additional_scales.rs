extern crate rust_music_theory as theory;
use theory::note::{Notes, Pitch, PitchSymbol::*};
use theory::scale::{Direction, Mode, Scale, ScaleType};

fn assert_scale_notes(expected: &[theory::note::PitchSymbol], scale: Scale) {
    let notes = scale.notes();
    assert_eq!(notes.len(), expected.len(), "Scale length mismatch");
    for (i, expected_pitch) in expected.iter().enumerate() {
        assert_eq!(
            Pitch::from(*expected_pitch),
            notes[i].pitch,
            "Mismatch at position {} in scale: expected {:?}, got {:?}",
            i,
            expected_pitch,
            notes[i].pitch
        );
    }
}

#[cfg(test)]
mod additional_scale_tests {
    use super::*;

    #[test]
    fn test_pentatonic_major_scales() {
        // C Major Pentatonic: C D E G A
        let scale = Scale::new(
            ScaleType::PentatonicMajor,
            Pitch::from(C),
            4,
            Some(Mode::PentatonicMajor),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[C, D, E, G, A, C], scale);

        // G Major Pentatonic: G A B D E
        let scale = Scale::new(
            ScaleType::PentatonicMajor,
            Pitch::from(G),
            4,
            Some(Mode::PentatonicMajor),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[G, A, B, D, E, G], scale);

        // F Major Pentatonic: F G A C D
        let scale = Scale::new(
            ScaleType::PentatonicMajor,
            Pitch::from(F),
            4,
            Some(Mode::PentatonicMajor),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[F, G, A, C, D, F], scale);
    }

    #[test]
    fn test_pentatonic_minor_scales() {
        // A Minor Pentatonic: A C D E G
        let scale = Scale::new(
            ScaleType::PentatonicMinor,
            Pitch::from(A),
            4,
            Some(Mode::PentatonicMinor),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[A, C, D, E, G, A], scale);

        // E Minor Pentatonic: E G A B D
        let scale = Scale::new(
            ScaleType::PentatonicMinor,
            Pitch::from(E),
            4,
            Some(Mode::PentatonicMinor),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[E, G, A, B, D, E], scale);

        // D Minor Pentatonic: D F G A C
        let scale = Scale::new(
            ScaleType::PentatonicMinor,
            Pitch::from(D),
            4,
            Some(Mode::PentatonicMinor),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[D, F, G, A, C, D], scale);
    }

    #[test]
    fn test_blues_scales() {
        // C Blues: C D# F F# G A# (using sharps for now)
        let scale = Scale::new(
            ScaleType::Blues,
            Pitch::from(C),
            4,
            Some(Mode::Blues),
            Direction::Ascending,
        )
        .unwrap();
        
        // C Blues uses sharp notation in current implementation
        assert_scale_notes(&[C, Ds, F, Fs, G, As, C], scale);

        // A Blues: A C D D# E G
        let scale = Scale::new(
            ScaleType::Blues,
            Pitch::from(A),
            4,
            Some(Mode::Blues),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[A, C, D, Ds, E, G, A], scale);

        // E Blues: E G A A# B D
        let scale = Scale::new(
            ScaleType::Blues,
            Pitch::from(E),
            4,
            Some(Mode::Blues),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[E, G, A, As, B, D, E], scale);
    }

    #[test]
    fn test_chromatic_scale() {
        // C Chromatic: C C# D D# E F F# G G# A A# B
        let scale = Scale::new(
            ScaleType::Chromatic,
            Pitch::from(C),
            4,
            Some(Mode::Chromatic),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B, C], scale);

        // F Chromatic should also have 13 notes (including octave)
        let scale = Scale::new(
            ScaleType::Chromatic,
            Pitch::from(F),
            4,
            Some(Mode::Chromatic),
            Direction::Ascending,
        )
        .unwrap();
        let notes = scale.notes();
        assert_eq!(notes.len(), 13, "Chromatic scale should have 13 notes including octave");
        assert_eq!(notes[0].pitch, Pitch::from(F));
        assert_eq!(notes[12].pitch, Pitch::from(F)); // Octave
    }

    #[test]
    fn test_whole_tone_scale() {
        // C Whole Tone: C D E F# G# A#
        let scale = Scale::new(
            ScaleType::WholeTone,
            Pitch::from(C),
            4,
            Some(Mode::WholeTone),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[C, D, E, Fs, Gs, As, C], scale);

        // D Whole Tone: D E F# G# A# C
        let scale = Scale::new(
            ScaleType::WholeTone,
            Pitch::from(D),
            4,
            Some(Mode::WholeTone),
            Direction::Ascending,
        )
        .unwrap();
        assert_scale_notes(&[D, E, Fs, Gs, As, C, D], scale);

        // Whole tone scales should have exactly 7 notes (including octave)
        let scale = Scale::new(
            ScaleType::WholeTone,
            Pitch::from(G),
            4,
            Some(Mode::WholeTone),
            Direction::Ascending,
        )
        .unwrap();
        let notes = scale.notes();
        assert_eq!(notes.len(), 7, "Whole tone scale should have 7 notes including octave");
    }

    #[test]
    fn test_scale_from_regex_new_types() {
        // Test pentatonic major parsing
        let scale = Scale::from_regex("C pentatonic major").unwrap();
        assert_eq!(scale.scale_type, ScaleType::PentatonicMajor);
        assert_eq!(scale.mode, Some(Mode::PentatonicMajor));
        assert_scale_notes(&[C, D, E, G, A, C], scale);

        // Test pentatonic minor parsing
        let scale = Scale::from_regex("A pentatonic minor").unwrap();
        assert_eq!(scale.scale_type, ScaleType::PentatonicMinor);
        assert_eq!(scale.mode, Some(Mode::PentatonicMinor));
        assert_scale_notes(&[A, C, D, E, G, A], scale);

        // Test blues parsing
        let scale = Scale::from_regex("E blues").unwrap();
        assert_eq!(scale.scale_type, ScaleType::Blues);
        assert_eq!(scale.mode, Some(Mode::Blues));
        assert_scale_notes(&[E, G, A, As, B, D, E], scale);

        // Test chromatic parsing
        let scale = Scale::from_regex("F chromatic").unwrap();
        assert_eq!(scale.scale_type, ScaleType::Chromatic);
        assert_eq!(scale.mode, Some(Mode::Chromatic));

        // Test whole tone parsing
        let scale = Scale::from_regex("G whole tone").unwrap();
        assert_eq!(scale.scale_type, ScaleType::WholeTone);
        assert_eq!(scale.mode, Some(Mode::WholeTone));
    }

    #[test]
    fn test_scale_abbreviations() {
        // Test abbreviated forms
        let scale1 = Scale::from_regex("C pent maj").unwrap();
        let scale2 = Scale::from_regex("C pentatonic major").unwrap();
        assert_eq!(scale1.scale_type, scale2.scale_type);
        assert_eq!(scale1.mode, scale2.mode);

        let scale1 = Scale::from_regex("A pent min").unwrap();
        let scale2 = Scale::from_regex("A pentatonic minor").unwrap();
        assert_eq!(scale1.scale_type, scale2.scale_type);
        assert_eq!(scale1.mode, scale2.mode);

        let scale1 = Scale::from_regex("D wholetone").unwrap();
        let scale2 = Scale::from_regex("D whole tone").unwrap();
        assert_eq!(scale1.scale_type, scale2.scale_type);
        assert_eq!(scale1.mode, scale2.mode);
    }

    #[test]
    fn test_descending_new_scales() {
        // Test that descending scales work correctly
        let scale = Scale::new(
            ScaleType::PentatonicMajor,
            Pitch::from(C),
            4,
            Some(Mode::PentatonicMajor),
            Direction::Descending,
        )
        .unwrap();
        
        let notes = scale.notes();
        assert_eq!(notes.len(), 6); // Should include the octave
        
        // First and last notes should be the same pitch (C)
        assert_eq!(notes[0].pitch, Pitch::from(C));
        assert_eq!(notes[5].pitch, Pitch::from(C));
    }

    #[test]
    fn test_sharp_key_enharmonics() {
        // Test that scales in sharp keys use proper enharmonic spelling
        let scale = Scale::new(
            ScaleType::PentatonicMajor,
            Pitch::from(Fs),
            4,
            Some(Mode::PentatonicMajor),
            Direction::Ascending,
        )
        .unwrap();
        
        let notes = scale.notes();
        // F# Major Pentatonic should use sharps, not flats
        // F# G# A# C# D#
        assert_eq!(notes[0].pitch, Pitch::from(Fs));
        assert_eq!(notes[1].pitch, Pitch::from(Gs));
        assert_eq!(notes[2].pitch, Pitch::from(As));
        assert_eq!(notes[3].pitch, Pitch::from(Cs));
        assert_eq!(notes[4].pitch, Pitch::from(Ds));
    }
}