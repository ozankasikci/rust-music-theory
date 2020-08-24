extern crate rust_music_theory as theory;
use theory::note::{Pitch, NoteLetter::*};

#[cfg(test)]
mod test_note {
    use super::*;

    #[test]
    fn test_pitch_from_str() {
        let table = vec![
            ("Cb", Pitch::new(C, -1)),
            ("C#", Pitch::new(C, 1)),
            ("C♯#", Pitch::new(C, 2)),
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
            ("G♭b♭", Pitch::new(G, -3)),
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
}
