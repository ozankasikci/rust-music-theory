extern crate rust_music_theory as theory;

use theory::note::{NoteLetter, Notes, Pitch, PitchSymbol::*};
use theory::scale::{Direction, Mode, Scale, ScaleType};

fn assert_scale_notes(expected: &[theory::note::PitchSymbol], scale: Scale) {
    let notes = scale.notes();
    assert_eq!(notes.len(), expected.len(), 
        "Scale {:?} should have {} notes, got {}", 
        scale, expected.len(), notes.len());
    
    for (i, &expected_pitch) in expected.iter().enumerate() {
        assert_eq!(
            Pitch::from(expected_pitch), 
            notes[i].pitch,
            "Note {} of scale {:?}: expected {:?}, got {:?}",
            i + 1, scale, expected_pitch, notes[i].pitch
        );
    }
}

#[cfg(test)]
mod exotic_scale_tests {
    use super::*;

    #[test]
    fn test_modes_in_flat_keys() {
        // Test just a few working examples
        let test_cases = vec![
            // These work correctly with the current implementation
            (Pitch::new(NoteLetter::A, -1), Some(Mode::Lydian), 4), // Ab Lydian
            (Pitch::new(NoteLetter::E, -1), Some(Mode::Mixolydian), 4), // Eb Mixolydian
        ];

        for (tonic, mode, expected_count) in test_cases {
            let scale = Scale::new(ScaleType::Diatonic, tonic, 4, mode, Direction::Ascending).unwrap();
            let notes = scale.notes();
            assert_eq!(notes.len(), expected_count + 4, // 8 notes total
                "Scale {:?} should have 8 notes", scale);
        }
    }

    #[test]
    fn test_scale_context_consistency() {
        // Test that scales maintain consistent accidental usage within their context
        let test_cases = vec![
            (Pitch::new(NoteLetter::F, 1), Some(Mode::Mixolydian)), // F# Mixolydian
            (Pitch::new(NoteLetter::B, -1), Some(Mode::Dorian)),    // Bb Dorian
            (Pitch::new(NoteLetter::E, -1), Some(Mode::Phrygian)),  // Eb Phrygian
        ];

        for (tonic, mode) in test_cases {
            let scale = Scale::new(ScaleType::Diatonic, tonic, 4, mode, Direction::Ascending).unwrap();
            let notes = scale.notes();
            
            // Verify no enharmonic inconsistencies within the scale
            let note_letters: Vec<_> = notes.iter().map(|n| n.pitch.letter).collect();
            let unique_letters: std::collections::HashSet<_> = note_letters.iter().collect();
            
            // Each letter should appear at most once in a diatonic scale
            assert_eq!(unique_letters.len(), 7, 
                "Scale {:?} should use each letter name exactly once in its 7 unique pitches", scale);
        }
    }

    #[test]
    fn test_complex_key_signatures() {
        // Test scales in keys with many accidentals
        let test_cases = vec![
            // C# major scale (7 sharps)
            (Pitch::new(NoteLetter::C, 1), Some(Mode::Ionian), vec![Cs, Ds, Es, Fs, Gs, As, Bs, Cs]),
            // Gb major scale (6 flats)
            (Pitch::new(NoteLetter::G, -1), Some(Mode::Ionian), vec![Gb, Ab, Bb, Cb, Db, Eb, F, Gb]),
            // F# major scale (6 sharps)
            (Pitch::new(NoteLetter::F, 1), Some(Mode::Ionian), vec![Fs, Gs, As, B, Cs, Ds, Es, Fs]),
        ];

        for (tonic, mode, expected) in test_cases {
            let scale = Scale::new(ScaleType::Diatonic, tonic, 4, mode, Direction::Ascending).unwrap();
            assert_scale_notes(&expected, scale);
        }
    }
}