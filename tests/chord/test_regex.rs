extern crate rust_music_theory as theory;
use theory::chord::{Chord, Quality::*};
use theory::note::PitchClass::*;

#[cfg(test)]
mod chord_regex_tests {
    use super::*;

    #[test]
    fn test_major() {
        let table = vec![
            ("C Major", C, Major),
            ("E MAJOR", E, Major),
            ("C Maj", C, Major),
            ("Cb MAJ", B, Major),
            ("C M", C, Major),
            ("C# Minor", Cs, Minor),
            ("Ds MINOR", Ds, Minor),
            ("D MIN", D, Minor),
            ("C min", C, Minor),
            ("C m", C, Minor),
            ("Cm", C, Minor),
            ("C Augmented", C, Augmented),
            ("C Augmented", C, Augmented),
        ];

        for (string, pitch, quality) in table {
            let chord = Chord::from_regex(string).unwrap();
            assert_eq!(chord.quality, quality);
            assert_eq!(chord.root, pitch);
        }
    }
}