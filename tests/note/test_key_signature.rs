extern crate rust_music_theory as theory;

use theory::note::{KeySignature, NoteLetter, Pitch, PitchSymbol};
use theory::scale::Mode;

#[cfg(test)]
mod key_signature_tests {
    use super::*; // Imports items from the outer module (key_signature_tests)

    #[test]
    fn test_preferred_spelling_c_major() {
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::C, 0));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(0)), PitchSymbol::C, "C in C Major"); 
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Cs, "C# in C Major (should be sharp)");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Ds, "D# in C Major (should be sharp)");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::As, "A# in C Major (should be sharp, not Bb)");
    }

    #[test]
    fn test_preferred_spelling_g_major() {
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::G, 0));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Fs, "F# in G Major (diatonic)"); 
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Cs, "C# in G Major (chromatic, sharp key context)");
    }

    #[test]
    fn test_preferred_spelling_f_major() {
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::F, 0));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::Bb, "Bb in F Major (diatonic)");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Db,  "Db in F Major (chromatic, C#/Db should be Db)");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Eb,  "Eb in F Major (chromatic, D#/Eb should be Eb)");
    }

    #[test]
    fn test_preferred_spelling_db_major() {
        // D♭ Major: Db, Eb, F, Gb, Ab, Bb, C
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::D, -1));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Db, "Db in Db Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Eb, "Eb in Db Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(5)), PitchSymbol::F,  "F in Db Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Gb, "Gb in Db Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Ab, "Ab in Db Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::Bb, "Bb in Db Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(0)), PitchSymbol::C,  "C in Db Major");
    }

    #[test]
    fn test_preferred_spelling_fs_major_challenging_case() {
        // F♯ Major: F♯, G♯, A♯, B, C♯, D♯, E♯
        // This relies on the KEY_SIGNATURE_SPELLINGS being comprehensive or the fallback logic being very smart.
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::F, 1)); 
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Fs, "F# in F# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Gs, "G# in F# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::As, "A# in F# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(11)), PitchSymbol::B,  "B in F# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Cs, "C# in F# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Ds, "D# in F# Major");
        // The following E-sharp test requires PitchSymbol::Es to be correctly implemented and preferred.
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(5)), PitchSymbol::Es, "E# in F# Major"); 
    }

    #[test]
    fn test_preferred_spelling_a_natural_minor() {
        // A natural minor (0 sharps/flats, relative to C major)
        // KeySignature::new_with_mode is used to provide context, though current get_preferred_spelling might not fully use Mode yet.
        let key_sig = KeySignature::new_with_mode(Pitch::new(NoteLetter::A, 0), Some(Mode::Aeolian));
        // In A natural minor, G is natural. For G#, it's usually a chromatic alteration or from harmonic/melodic minor.
        // The current KEY_SIGNATURE_SPELLINGS for (A,0) is [A, B, Cs, D, E, Fs, Gs] which implies A Major or A Dorian etc., not A natural minor directly for spelling.
        // The fallback for tonic A (is_sharp_key = true) would also tend towards sharps.
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(7)), PitchSymbol::G, "G natural in A natural minor");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Gs, "G# as chromatic/leading tone in A minor context");
    }

     #[test]
    fn test_pitch_symbol_bs_cb_spellings() {
        // Test B# spelling (should be preferred in C# major, for example)
        let key_cs_major = KeySignature::new(Pitch::new(NoteLetter::C, 1)); // C# Major
        assert_eq!(key_cs_major.get_preferred_spelling(Pitch::from_u8(0)), PitchSymbol::Bs, "B# in C# Major");

        // Test Cb spelling (should be preferred in Gb major or Db major, for example)
        let key_gb_major = KeySignature::new(Pitch::new(NoteLetter::G, -1)); // Gb Major
        assert_eq!(key_gb_major.get_preferred_spelling(Pitch::from_u8(11)), PitchSymbol::Cb, "Cb in Gb Major");
    }

    #[test]
    fn test_preferred_spelling_d_major() {
        // D Major: D, E, F#, G, A, B, C#
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::D, 0));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(2)), PitchSymbol::D, "D in D Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(4)), PitchSymbol::E, "E in D Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Fs, "F# in D Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(7)), PitchSymbol::G, "G in D Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(9)), PitchSymbol::A, "A in D Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(11)), PitchSymbol::B, "B in D Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Cs, "C# in D Major");
    }

    #[test]
    fn test_preferred_spelling_a_major() {
        // A Major: A, B, C#, D, E, F#, G#
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::A, 0));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(9)), PitchSymbol::A, "A in A Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(11)), PitchSymbol::B, "B in A Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Cs, "C# in A Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(2)), PitchSymbol::D, "D in A Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(4)), PitchSymbol::E, "E in A Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Fs, "F# in A Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Gs, "G# in A Major");
    }

    #[test]
    fn test_preferred_spelling_e_major() {
        // E Major: E, F#, G#, A, B, C#, D#
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::E, 0));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(4)), PitchSymbol::E, "E in E Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Fs, "F# in E Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Gs, "G# in E Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(9)), PitchSymbol::A, "A in E Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(11)), PitchSymbol::B, "B in E Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Cs, "C# in E Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Ds, "D# in E Major");
    }

    #[test]
    fn test_preferred_spelling_b_major() {
        // B Major: B, C#, D#, E, F#, G#, A#
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::B, 0));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(11)), PitchSymbol::B, "B in B Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Cs, "C# in B Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Ds, "D# in B Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(4)), PitchSymbol::E, "E in B Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Fs, "F# in B Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Gs, "G# in B Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::As, "A# in B Major");
    }

    #[test]
    fn test_preferred_spelling_bb_major() {
        // Bb Major: Bb, C, D, Eb, F, G, A
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::B, -1));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::Bb, "Bb in Bb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(0)), PitchSymbol::C, "C in Bb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(2)), PitchSymbol::D, "D in Bb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Eb, "Eb in Bb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(5)), PitchSymbol::F, "F in Bb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(7)), PitchSymbol::G, "G in Bb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(9)), PitchSymbol::A, "A in Bb Major");
    }

    #[test]
    fn test_preferred_spelling_eb_major() {
        // Eb Major: Eb, F, G, Ab, Bb, C, D
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::E, -1));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Eb, "Eb in Eb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(5)), PitchSymbol::F, "F in Eb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(7)), PitchSymbol::G, "G in Eb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Ab, "Ab in Eb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::Bb, "Bb in Eb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(0)), PitchSymbol::C, "C in Eb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(2)), PitchSymbol::D, "D in Eb Major");
    }

    #[test]
    fn test_preferred_spelling_ab_major() {
        // Ab Major: Ab, Bb, C, Db, Eb, F, G
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::A, -1));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Ab, "Ab in Ab Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::Bb, "Bb in Ab Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(0)), PitchSymbol::C, "C in Ab Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Db, "Db in Ab Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Eb, "Eb in Ab Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(5)), PitchSymbol::F, "F in Ab Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(7)), PitchSymbol::G, "G in Ab Major");
    }

    #[test]
    fn test_preferred_spelling_gb_major() {
        // Gb Major: Gb, Ab, Bb, Cb, Db, Eb, F
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::G, -1));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Gb, "Gb in Gb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Ab, "Ab in Gb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::Bb, "Bb in Gb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(11)), PitchSymbol::Cb, "Cb in Gb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Db, "Db in Gb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Eb, "Eb in Gb Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(5)), PitchSymbol::F, "F in Gb Major");
    }

    #[test]
    fn test_preferred_spelling_c_sharp_major() {
        // C# Major: C#, D#, E#, F#, G#, A#, B#
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::C, 1));
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Cs, "C# in C# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Ds, "D# in C# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(5)), PitchSymbol::Es, "E# in C# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Fs, "F# in C# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Gs, "G# in C# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::As, "A# in C# Major");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(0)), PitchSymbol::Bs, "B# in C# Major");
    }

    // Add tests for chromatic notes in C major
    #[test]
    fn test_chromatic_notes_in_c_major() {
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::C, 0));
        // C Major prefers sharps for chromatic notes not in its diatonic scale based on current logic
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Cs, "C#/Db in C Major should be C#"); // Diatonic C#
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Ds, "D#/Eb in C Major should be D#"); // Diatonic D#
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Fs, "F#/Gb in C Major should be F#"); // Diatonic F#
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Gs, "G#/Ab in C Major should be G#"); // Diatonic G#
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::As, "A#/Bb in C Major should be A#"); // Diatonic A#
    }

    // Add tests for chromatic notes in G major (a sharp key)
    #[test]
    fn test_chromatic_notes_in_g_major() {
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::G, 0));
        // G Major (F#) - should prefer sharps generally
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Cs, "C#/Db in G Major should be C#");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Ds, "D#/Eb in G Major should be D#"); 
        // F# is diatonic ( PitchSymbol::Fs)
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Gs, "G#/Ab in G Major should be G#");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(10)), PitchSymbol::As, "A#/Bb in G Major should be A#");
    }

    // Add tests for chromatic notes in F major (a flat key)
    #[test]
    fn test_chromatic_notes_in_f_major() {
        let key_sig = KeySignature::new(Pitch::new(NoteLetter::F, 0));
        // F Major (Bb) - should prefer flats for notes where applicable based on current logic
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(1)), PitchSymbol::Db, "C#/Db in F Major should be Db");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(3)), PitchSymbol::Eb, "D#/Eb in F Major should be Eb"); 
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(6)), PitchSymbol::Gb, "F#/Gb in F Major should be Gb");
        assert_eq!(key_sig.get_preferred_spelling(Pitch::from_u8(8)), PitchSymbol::Ab, "G#/Ab in F Major should be Ab");
        // Bb is diatonic (PitchSymbol::Bb)
    }
} 