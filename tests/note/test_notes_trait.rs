extern crate rust_music_theory as theory;
use theory::note::{Notes, Pitch, NoteLetter};
use theory::chord::Chord;
use theory::scale::Scale;

#[cfg(test)]
mod test_notes_trait {
    use super::*;

    #[test]
    fn test_chord_notes_trait() {
        // Test that Chord implements Notes trait
        let c_major = Chord::from_regex("C major").unwrap();
        let notes = c_major.notes();

        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::C, 0));
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::E, 0));
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::G, 0));
    }

    #[test]
    fn test_scale_notes_trait() {
        // Test that Scale implements Notes trait
        let c_major_scale = Scale::from_regex("C major").unwrap();
        let notes = c_major_scale.notes();

        assert_eq!(notes.len(), 8); // Including the octave
        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::C, 0));
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::D, 0));
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::E, 0));
        assert_eq!(notes[3].pitch, Pitch::new(NoteLetter::F, 0));
        assert_eq!(notes[4].pitch, Pitch::new(NoteLetter::G, 0));
        assert_eq!(notes[5].pitch, Pitch::new(NoteLetter::A, 0));
        assert_eq!(notes[6].pitch, Pitch::new(NoteLetter::B, 0));
        assert_eq!(notes[7].pitch, Pitch::new(NoteLetter::C, 0));
    }

    #[test]
    fn test_notes_trait_with_different_roots() {
        // Test with G major scale
        let g_major_scale = Scale::from_regex("G major").unwrap();
        let notes = g_major_scale.notes();

        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::G, 0));
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::A, 0));
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::B, 0));
        assert_eq!(notes[3].pitch, Pitch::new(NoteLetter::C, 0));
        assert_eq!(notes[4].pitch, Pitch::new(NoteLetter::D, 0));
        assert_eq!(notes[5].pitch, Pitch::new(NoteLetter::E, 0));
        assert_eq!(notes[6].pitch, Pitch::new(NoteLetter::F, 1)); // F#
        assert_eq!(notes[7].pitch, Pitch::new(NoteLetter::G, 0));
    }

    #[test]
    fn test_notes_trait_with_flats() {
        // Test with F major scale (has Bb)
        let f_major_scale = Scale::from_regex("F major").unwrap();
        let notes = f_major_scale.notes();

        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::F, 0));
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::G, 0));
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::A, 0));
        assert_eq!(notes[3].pitch, Pitch::new(NoteLetter::B, -1)); // Bb
        assert_eq!(notes[4].pitch, Pitch::new(NoteLetter::C, 0));
        assert_eq!(notes[5].pitch, Pitch::new(NoteLetter::D, 0));
        assert_eq!(notes[6].pitch, Pitch::new(NoteLetter::E, 0));
        assert_eq!(notes[7].pitch, Pitch::new(NoteLetter::F, 0));
    }

    #[test]
    fn test_chord_notes_with_sharps_and_flats() {
        // Test D minor chord
        let d_minor = Chord::from_regex("D minor").unwrap();
        let notes = d_minor.notes();

        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::D, 0));
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::F, 0));
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::A, 0));

        // Test F# major chord
        let fs_major = Chord::from_regex("F# major").unwrap();
        let notes = fs_major.notes();

        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::F, 1)); // F#
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::A, 1)); // A#
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::C, 1)); // C#
    }

    #[test]
    fn test_notes_octave_progression() {
        // Create a C major scale (default octave is 4)
        let c_major = Scale::from_regex("C major").unwrap();
        let notes = c_major.notes();

        // First note should be at octave 4 (default)
        assert_eq!(notes[0].octave, 4);

        // Notes should stay in the same octave until the tonic repeats
        for i in 1..7 {
            assert_eq!(notes[i].octave, 4);
        }

        // The octave should increment for notes that wrap around
        assert_eq!(notes[7].octave, 5);
    }

    #[test]
    fn test_seventh_chord_notes() {
        // Test a seventh chord
        let c_maj7 = Chord::from_regex("C major seventh").unwrap();
        let notes = c_maj7.notes();

        assert_eq!(notes.len(), 4);
        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::C, 0));
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::E, 0));
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::G, 0));
        assert_eq!(notes[3].pitch, Pitch::new(NoteLetter::B, 0));
    }

    #[test]
    fn test_diminished_chord_notes() {
        // Test a diminished chord
        let b_dim = Chord::from_regex("B diminished").unwrap();
        let notes = b_dim.notes();

        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::B, 0));
        assert_eq!(notes[1].pitch, Pitch::new(NoteLetter::D, 0));
        assert_eq!(notes[2].pitch, Pitch::new(NoteLetter::F, 0));
    }

    #[test]
    fn test_chromatic_scale_notes() {
        // Test chromatic scale
        let c_chromatic = Scale::from_regex("C chromatic").unwrap();
        let notes = c_chromatic.notes();

        // Chromatic scale has 13 notes (12 semitones + octave)
        assert_eq!(notes.len(), 13);

        // First and last notes should be C
        assert_eq!(notes[0].pitch, Pitch::new(NoteLetter::C, 0));
        assert_eq!(notes[12].pitch, Pitch::new(NoteLetter::C, 0));

        // Last note should be an octave higher
        assert_eq!(notes[12].octave, notes[0].octave + 1);
    }
}