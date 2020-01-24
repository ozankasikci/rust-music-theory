extern crate rust_music_theory as theory;

#[cfg(test)]
mod chord_regex_tests {
    use super::*;

    #[test]
    fn test_regex() {
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
}
