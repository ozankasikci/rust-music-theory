extern crate rust_music_theory as theory;

use theory::chord::{Chord, Number::*, Quality::*};
use theory::note::{NoteLetter, Notes, Pitch, PitchSymbol::*};

fn assert_chord_notes(expected: &[theory::note::PitchSymbol], chord: Chord) {
    let notes = chord.notes();
    assert_eq!(notes.len(), expected.len(), 
        "Chord {} should have {} notes, got {}", 
        format!("{:?}", chord), expected.len(), notes.len());
    
    for (i, &expected_pitch) in expected.iter().enumerate() {
        assert_eq!(
            Pitch::from(expected_pitch), 
            notes[i].pitch,
            "Note {} of chord {:?}: expected {:?}, got {:?}",
            i + 1, chord, expected_pitch, notes[i].pitch
        );
    }
}

fn assert_chord_spelling(expected: &str, chord: Chord) {
    let expected: Vec<Pitch> = expected
        .split_whitespace()
        .map(|pitch| Pitch::from_str(pitch).unwrap())
        .collect();
    let actual: Vec<Pitch> = chord.notes().into_iter().map(|note| note.pitch).collect();
    assert_eq!(actual, expected, "Incorrect spelling for {:?}", chord);
}

#[cfg(test)]
mod chord_enharmonic_tests {
    use super::*;

    #[test]
    fn test_major_triads_flat_keys() {
        // Test all major triads in flat keys use consistent flat spelling
        let test_cases = vec![
            (Pitch::new(NoteLetter::F, 0), vec![F, A, C]),           // F major
            (Pitch::new(NoteLetter::B, -1), vec![Bb, D, F]),         // Bb major  
            (Pitch::new(NoteLetter::E, -1), vec![Eb, G, Bb]),        // Eb major
            (Pitch::new(NoteLetter::A, -1), vec![Ab, C, Eb]),        // Ab major
            (Pitch::new(NoteLetter::D, -1), vec![Db, F, Ab]),        // Db major
            (Pitch::new(NoteLetter::G, -1), vec![Gb, Bb, Db]),       // Gb major
        ];

        for (root, expected) in test_cases {
            let chord = Chord::new(root, Major, Triad);
            assert_chord_notes(&expected, chord);
        }
    }

    #[test]
    fn test_major_triads_sharp_keys() {
        // Test all major triads in sharp keys use consistent sharp spelling
        let test_cases = vec![
            (Pitch::new(NoteLetter::G, 0), vec![G, B, D]),           // G major
            (Pitch::new(NoteLetter::D, 0), vec![D, Fs, A]),          // D major
            (Pitch::new(NoteLetter::A, 0), vec![A, Cs, E]),          // A major
            (Pitch::new(NoteLetter::E, 0), vec![E, Gs, B]),          // E major
            (Pitch::new(NoteLetter::B, 0), vec![B, Ds, Fs]),         // B major
            (Pitch::new(NoteLetter::F, 1), vec![Fs, As, Cs]),        // F# major
            (Pitch::new(NoteLetter::C, 1), vec![Cs, Es, Gs]),        // C# major
        ];

        for (root, expected) in test_cases {
            let chord = Chord::new(root, Major, Triad);
            assert_chord_notes(&expected, chord);
        }
    }

    #[test]
    fn test_minor_triads_enharmonic_spelling() {
        let test_cases = vec![
            (Pitch::new(NoteLetter::F, 0), "F Ab C"),
            (Pitch::new(NoteLetter::B, -1), "Bb Db F"),
            (Pitch::new(NoteLetter::E, -1), "Eb Gb Bb"),
            (Pitch::new(NoteLetter::A, -1), "Ab Cb Eb"),
            (Pitch::new(NoteLetter::D, -1), "Db Fb Ab"),
            (Pitch::new(NoteLetter::G, -1), "Gb Bbb Db"),
            (Pitch::new(NoteLetter::G, 0), "G Bb D"),
            (Pitch::new(NoteLetter::D, 0), "D F A"),
            (Pitch::new(NoteLetter::A, 0), "A C E"),
            (Pitch::new(NoteLetter::E, 0), "E G B"),
            (Pitch::new(NoteLetter::B, 0), "B D Fs"),
            (Pitch::new(NoteLetter::F, 1), "Fs A Cs"),
            (Pitch::new(NoteLetter::C, 1), "Cs E Gs"),
        ];

        for (root, expected) in test_cases {
            let chord = Chord::new(root, Minor, Triad);
            assert_chord_spelling(expected, chord);
        }
    }

    #[test]
    fn test_seventh_chords_enharmonic_spelling() {
        let test_cases = vec![
            // Major 7th chords in flat keys
            (Pitch::new(NoteLetter::D, -1), vec![Db, F, Ab, C]),     // Db maj7
            (Pitch::new(NoteLetter::G, -1), vec![Gb, Bb, Db, F]),    // Gb maj7
            (Pitch::new(NoteLetter::A, -1), vec![Ab, C, Eb, G]),     // Ab maj7
            
            // Major 7th chords in sharp keys  
            (Pitch::new(NoteLetter::F, 1), vec![Fs, As, Cs, Es]),    // F# maj7
            (Pitch::new(NoteLetter::C, 1), vec![Cs, Es, Gs, Bs]),    // C# maj7
            (Pitch::new(NoteLetter::B, 0), vec![B, Ds, Fs, As]),     // B maj7
            
        ];

        for (root, expected) in test_cases {
            assert_chord_notes(&expected, Chord::new(root, Major, MajorSeventh));
        }

        assert_chord_spelling("Gb Bb Db Fb", Chord::new(Pitch::new(NoteLetter::G, -1), Dominant, Seventh));
        assert_chord_spelling("Fs As Cs E", Chord::new(Pitch::new(NoteLetter::F, 1), Dominant, Seventh));
    }

    #[test]
    fn test_diminished_and_augmented_triads() {
        let test_cases = vec![
            (Pitch::new(NoteLetter::G, -1), Diminished, "Gb Bbb Dbb"),
            (Pitch::new(NoteLetter::F, 1), Diminished, "Fs A C"),
            (Pitch::new(NoteLetter::A, -1), Diminished, "Ab Cb Ebb"),
            (Pitch::new(NoteLetter::G, -1), Augmented, "Gb Bb D"),
            (Pitch::new(NoteLetter::F, 1), Augmented, "Fs As Css"),
            (Pitch::new(NoteLetter::D, -1), Augmented, "Db F A"),
        ];

        for (root, quality, expected) in test_cases {
            let chord = Chord::new(root, quality, Triad);
            assert_chord_spelling(expected, chord);
        }
    }

    #[test]
    fn test_chord_inversions_preserve_spelling() {
        // Test that inversions maintain the same accidental spelling
        let root = Pitch::new(NoteLetter::G, -1); // Gb major
        
        // Root position: Gb Bb Db
        let root_pos = Chord::with_inversion(root, Major, Triad, 0);
        assert_chord_notes(&vec![Gb, Bb, Db], root_pos);
        
        // First inversion: Bb Db Gb  
        let first_inv = Chord::with_inversion(root, Major, Triad, 1);
        let first_inv_notes = first_inv.notes();
        assert_eq!(Pitch::from(Bb), first_inv_notes[0].pitch);
        assert_eq!(Pitch::from(Db), first_inv_notes[1].pitch);
        assert_eq!(Pitch::from(Gb), first_inv_notes[2].pitch);
        
        // Second inversion: Db Gb Bb
        let second_inv = Chord::with_inversion(root, Major, Triad, 2);
        let second_inv_notes = second_inv.notes();
        assert_eq!(Pitch::from(Db), second_inv_notes[0].pitch);
        assert_eq!(Pitch::from(Gb), second_inv_notes[1].pitch);
        assert_eq!(Pitch::from(Bb), second_inv_notes[2].pitch);
    }

    #[test]
    fn test_enharmonic_chord_equivalence() {
        // Test that enharmonically equivalent chords have different spellings
        // but same semitone content
        
        // F# major vs Gb major
        let fs_major = Chord::new(Pitch::new(NoteLetter::F, 1), Major, Triad);
        let gb_major = Chord::new(Pitch::new(NoteLetter::G, -1), Major, Triad);
        
        let fs_notes = fs_major.notes();
        let gb_notes = gb_major.notes();
        
        // Same semitone content
        let fs_semitones: Vec<u8> = fs_notes.iter().map(|n| n.pitch.into_u8()).collect();
        let gb_semitones: Vec<u8> = gb_notes.iter().map(|n| n.pitch.into_u8()).collect();
        assert_eq!(fs_semitones, gb_semitones);
        
        // Different spelling
        assert_chord_notes(&vec![Fs, As, Cs], fs_major);
        assert_chord_notes(&vec![Gb, Bb, Db], gb_major);
    }

    #[test]
    fn test_complex_chord_extensions() {
        // Test that extended chords maintain consistent spelling
        let test_cases = vec![
            // 9th chords
            (Pitch::new(NoteLetter::D, -1), Major, Ninth, vec![Db, F, Ab, C, Eb]),
            (Pitch::new(NoteLetter::F, 1), Major, Ninth, vec![Fs, As, Cs, Es, Gs]),
            
            // 11th chords
            (Pitch::new(NoteLetter::A, -1), Major, Eleventh, vec![Ab, C, Eb, G, Bb, Db]),
            (Pitch::new(NoteLetter::B, 0), Major, Eleventh, vec![B, Ds, Fs, As, Cs, E]),
        ];

        for (root, quality, number, expected) in test_cases {
            let chord = Chord::new(root, quality, number);
            assert_chord_notes(&expected, chord);
        }
    }
}
