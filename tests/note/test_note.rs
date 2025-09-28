extern crate rust_music_theory as theory;
use theory::note::{Note, Pitch, NoteLetter};

#[cfg(test)]
mod test_note {
    use super::*;

    #[test]
    fn test_note_new() {
        let note = Note::new(Pitch::new(NoteLetter::C, 0), 4);
        assert_eq!(note.pitch, Pitch::new(NoteLetter::C, 0));
        assert_eq!(note.octave, 4);

        let note_sharp = Note::new(Pitch::new(NoteLetter::F, 1), 5);
        assert_eq!(note_sharp.pitch, Pitch::new(NoteLetter::F, 1));
        assert_eq!(note_sharp.octave, 5);

        let note_flat = Note::new(Pitch::new(NoteLetter::B, -1), 3);
        assert_eq!(note_flat.pitch, Pitch::new(NoteLetter::B, -1));
        assert_eq!(note_flat.octave, 3);
    }

    #[test]
    fn test_note_display() {
        let note_c = Note::new(Pitch::new(NoteLetter::C, 0), 4);
        assert_eq!(format!("{}", note_c), "C");

        let note_fs = Note::new(Pitch::new(NoteLetter::F, 1), 5);
        assert_eq!(format!("{}", note_fs), "F#");

        let note_bb = Note::new(Pitch::new(NoteLetter::B, -1), 3);
        assert_eq!(format!("{}", note_bb), "Bb");

        let note_css = Note::new(Pitch::new(NoteLetter::C, 2), 6);
        assert_eq!(format!("{}", note_css), "C##");

        let note_ebb = Note::new(Pitch::new(NoteLetter::E, -2), 2);
        assert_eq!(format!("{}", note_ebb), "Ebb");
    }

    #[test]
    fn test_note_different_octaves() {
        let note_c0 = Note::new(Pitch::new(NoteLetter::C, 0), 0);
        let note_c4 = Note::new(Pitch::new(NoteLetter::C, 0), 4);
        let note_c8 = Note::new(Pitch::new(NoteLetter::C, 0), 8);

        assert_eq!(note_c0.pitch, note_c4.pitch);
        assert_eq!(note_c4.pitch, note_c8.pitch);
        assert_ne!(note_c0.octave, note_c4.octave);
        assert_ne!(note_c4.octave, note_c8.octave);
    }

    #[test]
    fn test_note_enharmonic_equivalents() {
        let note_cs = Note::new(Pitch::new(NoteLetter::C, 1), 4);
        let note_db = Note::new(Pitch::new(NoteLetter::D, -1), 4);

        // Both notes should have the same pitch class (same semitone value)
        assert_eq!(note_cs.pitch.into_u8(), note_db.pitch.into_u8());
        assert_eq!(note_cs.octave, note_db.octave);

        // But different pitch representations
        assert_ne!(format!("{}", note_cs), format!("{}", note_db));
        assert_eq!(format!("{}", note_cs), "C#");
        assert_eq!(format!("{}", note_db), "Db");
    }

    #[test]
    fn test_note_extreme_accidentals() {
        let note_c_triple_sharp = Note::new(Pitch::new(NoteLetter::C, 3), 4);
        assert_eq!(format!("{}", note_c_triple_sharp), "C###");

        let note_f_triple_flat = Note::new(Pitch::new(NoteLetter::F, -3), 5);
        assert_eq!(format!("{}", note_f_triple_flat), "Fbbb");
    }
}