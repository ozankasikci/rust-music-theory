extern crate rust_music_theory as theory;
use theory::chord::{Chord, Quality::*};
use theory::note::PitchClass::*;


#[cfg(test)]
mod chord_regex_tests {
    use super::*;

    #[test]
    fn test_major() {
        let table = vec![
            ("C Major", C),
            ("E MAJOR", E),
            ("C Maj", C),
            ("Cb MAJ", B),
            ("Cb MAJ Seventh", B),
            ("C M", C),
            ("C MaJ Triad", C),
            ("C Major Seventh", C),
            ("C M Ninth", C),
        ];

        for (string, pitch) in table {
            let chord = Chord::from_regex(string).unwrap();
            assert_eq!(chord.quality, Major);
            assert_eq!(chord.root, pitch);
        }
    }

    #[test]
    fn test_minor() {
        let table = vec![
            ("C Minor", C),
            ("E MINOR", E),
            ("C Min", C),
            ("Cb MIN", B),
            ("Cb MIN Seventh", B),
            ("C m", C),
            ("C MiN Triad", C),
            ("C Minor Seventh", C),
            ("C m Ninth", C),
        ];

        for (string, pitch) in table {
            let chord = Chord::from_regex(string).unwrap();
            assert_eq!(chord.quality, Minor);
            assert_eq!(chord.root, pitch);
        }
    }
}