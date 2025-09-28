extern crate rust_music_theory as theory;
use theory::note::{PitchSymbol, Pitch, NoteLetter};

#[cfg(test)]
mod test_pitch_symbol {
    use super::*;

    #[test]
    fn test_pitch_symbol_display() {
        assert_eq!(format!("{}", PitchSymbol::Bs), "B♯");
        assert_eq!(format!("{}", PitchSymbol::C), "C");
        assert_eq!(format!("{}", PitchSymbol::Cs), "C♯");
        assert_eq!(format!("{}", PitchSymbol::Db), "D♭");
        assert_eq!(format!("{}", PitchSymbol::D), "D");
        assert_eq!(format!("{}", PitchSymbol::Ds), "D♯");
        assert_eq!(format!("{}", PitchSymbol::Eb), "E♭");
        assert_eq!(format!("{}", PitchSymbol::E), "E");
        assert_eq!(format!("{}", PitchSymbol::Es), "E♯");
        assert_eq!(format!("{}", PitchSymbol::F), "F");
        assert_eq!(format!("{}", PitchSymbol::Fs), "F♯");
        assert_eq!(format!("{}", PitchSymbol::Gb), "G♭");
        assert_eq!(format!("{}", PitchSymbol::G), "G");
        assert_eq!(format!("{}", PitchSymbol::Gs), "G♯");
        assert_eq!(format!("{}", PitchSymbol::Ab), "A♭");
        assert_eq!(format!("{}", PitchSymbol::A), "A");
        assert_eq!(format!("{}", PitchSymbol::As), "A♯");
        assert_eq!(format!("{}", PitchSymbol::Bb), "B♭");
        assert_eq!(format!("{}", PitchSymbol::B), "B");
        assert_eq!(format!("{}", PitchSymbol::Cb), "C♭");
    }

    #[test]
    fn test_pitch_symbol_to_pitch_conversion() {
        // Test natural notes
        assert_eq!(Pitch::from(PitchSymbol::C), Pitch::new(NoteLetter::C, 0));
        assert_eq!(Pitch::from(PitchSymbol::D), Pitch::new(NoteLetter::D, 0));
        assert_eq!(Pitch::from(PitchSymbol::E), Pitch::new(NoteLetter::E, 0));
        assert_eq!(Pitch::from(PitchSymbol::F), Pitch::new(NoteLetter::F, 0));
        assert_eq!(Pitch::from(PitchSymbol::G), Pitch::new(NoteLetter::G, 0));
        assert_eq!(Pitch::from(PitchSymbol::A), Pitch::new(NoteLetter::A, 0));
        assert_eq!(Pitch::from(PitchSymbol::B), Pitch::new(NoteLetter::B, 0));

        // Test sharp notes
        assert_eq!(Pitch::from(PitchSymbol::Cs), Pitch::new(NoteLetter::C, 1));
        assert_eq!(Pitch::from(PitchSymbol::Ds), Pitch::new(NoteLetter::D, 1));
        assert_eq!(Pitch::from(PitchSymbol::Es), Pitch::new(NoteLetter::E, 1));
        assert_eq!(Pitch::from(PitchSymbol::Fs), Pitch::new(NoteLetter::F, 1));
        assert_eq!(Pitch::from(PitchSymbol::Gs), Pitch::new(NoteLetter::G, 1));
        assert_eq!(Pitch::from(PitchSymbol::As), Pitch::new(NoteLetter::A, 1));
        assert_eq!(Pitch::from(PitchSymbol::Bs), Pitch::new(NoteLetter::B, 1));

        // Test flat notes
        assert_eq!(Pitch::from(PitchSymbol::Cb), Pitch::new(NoteLetter::C, -1));
        assert_eq!(Pitch::from(PitchSymbol::Db), Pitch::new(NoteLetter::D, -1));
        assert_eq!(Pitch::from(PitchSymbol::Eb), Pitch::new(NoteLetter::E, -1));
        assert_eq!(Pitch::from(PitchSymbol::Gb), Pitch::new(NoteLetter::G, -1));
        assert_eq!(Pitch::from(PitchSymbol::Ab), Pitch::new(NoteLetter::A, -1));
        assert_eq!(Pitch::from(PitchSymbol::Bb), Pitch::new(NoteLetter::B, -1));
    }

    #[test]
    fn test_pitch_symbol_enharmonic_equivalents() {
        // B# and C are enharmonically equivalent
        let bs_pitch = Pitch::from(PitchSymbol::Bs);
        let c_pitch = Pitch::from(PitchSymbol::C);
        assert_eq!(bs_pitch.into_u8() % 12, c_pitch.into_u8());

        // C# and Db are enharmonically equivalent
        let cs_pitch = Pitch::from(PitchSymbol::Cs);
        let db_pitch = Pitch::from(PitchSymbol::Db);
        assert_eq!(cs_pitch.into_u8(), db_pitch.into_u8());

        // E# and F are enharmonically equivalent
        let es_pitch = Pitch::from(PitchSymbol::Es);
        let f_pitch = Pitch::from(PitchSymbol::F);
        assert_eq!(es_pitch.into_u8(), f_pitch.into_u8());

        // Cb and B are enharmonically equivalent
        let cb_pitch = Pitch::from(PitchSymbol::Cb);
        let b_pitch = Pitch::from(PitchSymbol::B);
        assert_eq!(cb_pitch.into_u8() % 12, b_pitch.into_u8());
    }

    #[test]
    fn test_pitch_symbol_equality() {
        assert_eq!(PitchSymbol::C, PitchSymbol::C);
        assert_ne!(PitchSymbol::C, PitchSymbol::Cs);
        assert_ne!(PitchSymbol::Cs, PitchSymbol::Db);
        assert_eq!(PitchSymbol::Fs, PitchSymbol::Fs);
        assert_ne!(PitchSymbol::Fs, PitchSymbol::Gb);
    }
}