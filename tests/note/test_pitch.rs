extern crate rust_music_theory as theory;
use theory::note::{Pitch, NoteLetter::*, pitch};

#[cfg(test)]
mod test_note {
    use super::*;

    #[test]
    fn test_pitch_from_str() {
        let table = vec![
            ("Cb", pitch(C, -1)),
            ("C#", pitch(C, 1)),
            ("C♯", pitch(C, 1)),
            ("D", pitch(D, 0)),
            ("Db", pitch(D, -1)),
            ("Ds", pitch(D, 1)),
            ("E", pitch(E, 0)),
            ("Es", pitch(E, 1)),
            ("Eb", pitch(E, -1)),
            ("F", pitch(F, 0)),
            ("f", pitch(F, 0)),
            ("Fb", pitch(F, -1)),
            ("G", pitch(G, 0)),
            ("Gb", pitch(G, -1)),
            ("Gs", pitch(G, 1)),
            ("A", pitch(A, 0)),
            ("As", pitch(A, 1)),
            ("Ab", pitch(A, -1)),
            ("B", pitch(B, 0)),
            ("B♯", pitch(B, 1)),
            ("Bb", pitch(B, -1)),
        ];

        for (string, pitch) in table {
            let p = Pitch::from_str(string).unwrap();
            assert_eq!(p, pitch);
            assert_eq!(string.parse::<Pitch>().unwrap(), pitch);
        }
    }

    #[test]
    fn test_pitch_into_u8() {
        let table = vec![
            (pitch(C, 0), 0),
            (pitch(C, 1), 1),
            (pitch(D, 0), 2),
            (pitch(D, 1), 3),
            (pitch(E, 0), 4),
            (pitch(F, 0), 5),
            (pitch(F, 1), 6),
            (pitch(G, 0), 7),
            (pitch(G, 1), 8),
            (pitch(A, 0), 9),
            (pitch(A, 1), 10),
            (pitch(B, 0), 11),
        ];

        for (pitch, number) in table {
            let n = pitch.into_u8();
            assert_eq!(n, number);
        }
    }

    #[test]
    fn test_pitch_format() {
        assert_eq!(format!("{}", pitch(C,2)), "C##");
        assert_eq!(format!("{}", pitch(C,-2)), "Cbb");
        assert_eq!(format!("{}", pitch(C,0)), "C");
    }
}
