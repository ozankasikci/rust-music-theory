extern crate rust_music_theory as theory;
use theory::note::{PitchClass, PitchSymbol::*, pclass};

#[cfg(test)]
mod test_note {
    use super::*;

    #[test]
    fn test_pitch_class_from_str() {
        let table = vec![
            ("Cb", pclass(C,-1)),
            ("C#", pclass(C,1)),
            ("C♯", pclass(C,1)),
            ("D",  pclass(D,0)),
            ("Db", pclass(D,-1)),
            ("Ds", pclass(D,1)),
            ("E",  pclass(E,0)),
            ("Es", pclass(E,1)),
            ("Eb", pclass(E,-1)),
            ("F",  pclass(F,0)),
            ("f",  pclass(F,0)),
            ("Fb", pclass(F,-1)),
            ("G",  pclass(G,0)),
            ("Gb", pclass(G,-1)),
            ("Gs", pclass(G,1)),
            ("A",  pclass(A,0)),
            ("As", pclass(A,1)),
            ("Ab", pclass(A,-1)),
            ("B",  pclass(B,0)),
            ("B♯", pclass(B,1)),
            ("Bb", pclass(B,-1)),
        ];

        for (string, pitch_class) in table {
            let p = PitchClass::from_str(string).unwrap();
            assert_eq!(p, pitch_class);
            assert_eq!(string.parse::<PitchClass>().unwrap(), pitch_class);
        }
    }

    #[test]
    fn test_pitch_class_into_u8() {
        let table = vec![
            (pclass(C,0), 0),
            (pclass(C,1), 1),
            (pclass(D,0), 2),
            (pclass(D,1), 3),
            (pclass(E,0), 4),
            (pclass(F,0), 5),
            (pclass(F,1), 6),
            (pclass(G,0), 7),
            (pclass(G,1), 8),
            (pclass(A,0), 9),
            (pclass(A,1), 10),
            (pclass(B,0), 11),
        ];

        for (pitch_class, number) in table {
            let n = pitch_class.into_u8();
            assert_eq!(n, number);
        }
    }
}
