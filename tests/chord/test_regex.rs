extern crate rust_music_theory as theory;
use theory::chord::{Chord, Number, Number::*, Quality, Quality::*};
use theory::note::{PitchClass, PitchClass::*};

fn assert_chords(table: Vec<(&str, PitchClass, Quality, Number)>) {
    for (string, pitch, quality, number) in table {
        let chord = Chord::from_regex(string).unwrap();
        assert_eq!(chord.quality, quality);
        assert_eq!(chord.root, pitch);
        assert_eq!(chord.number, number);
    }
}

#[cfg(test)]
mod chord_regex_tests {
    use super::*;

    #[test]
    fn test_major() {
        let table = vec![
            ("C Major", C, Major, Triad),
            ("E MAJOR", E, Major, Triad),
            ("C Maj", C, Major, Triad),
            ("Cb MAJ", B, Major, Triad),
            ("Cb MAJ Seventh", B, Major, Seventh),
            ("C M", C, Major, Triad),
            ("C MaJ Triad", C, Major, Triad),
            ("C Major Seventh", C, Major, Seventh),
            ("C M Ninth", C, Major, Ninth),
            ("C M eleventh", C, Major, Eleventh),
            ("C M ThirTeenth", C, Major, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_minor() {
        let table = vec![
            ("C Minor", C, Minor, Triad),
            ("E MINOR", E, Minor, Triad),
            ("C Min", C, Minor, Triad),
            ("Cb MIN", B, Minor, Triad),
            ("Cb MIN Seventh", B, Minor, Seventh),
            ("C m", C, Minor, Triad),
            ("C MiN Triad", C, Minor, Triad),
            ("C Minor Seventh", C, Minor, Seventh),
            ("C m Ninth", C, Minor, Ninth),
            ("C#m Eleventh", Cs, Minor, Eleventh),
            ("Dsm Thirteenth", Ds, Minor, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_augmented() {
        let table = vec![
            ("C augmented", C, Augmented, Triad),
            ("E Augmented", E, Augmented, Triad),
            ("C augmented", C, Augmented, Triad),
            ("Cb augmented", B, Augmented, Triad),
            ("Cb augmented seventh", B, Augmented, Seventh),
            ("C augmented", C, Augmented, Triad),
            ("C augmented Triad", C, Augmented, Triad),
            ("C Augmented Seventh", C, Augmented, Seventh),
            ("C Augmented Ninth", C, Augmented, Ninth),
            ("C# augmented Eleventh", Cs, Augmented, Eleventh),
            ("Ds augmented Thirteenth", Ds, Augmented, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_diminished() {
        let table = vec![
            ("C Diminished", C, Diminished, Triad),
            ("E Diminished", E, Diminished, Triad),
            ("C Diminished", C, Diminished, Triad),
            ("Cb Diminished", B, Diminished, Triad),
            ("Cb Diminished seventh", B, Diminished, Seventh),
            ("C Diminished", C, Diminished, Triad),
            ("C Diminished Triad", C, Diminished, Triad),
            ("C Diminished Seventh", C, Diminished, Seventh),
            ("C Diminished Ninth", C, Diminished, Ninth),
            ("C# Diminished Eleventh", Cs, Diminished, Eleventh),
            ("Ds Diminished Thirteenth", Ds, Diminished, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_half_diminished() {
        let table = vec![
            ("C Half Diminished", C, HalfDiminished, Triad),
            ("E halfdiminished", E, HalfDiminished, Triad),
            ("C half diminished", C, HalfDiminished, Triad),
            ("Cb HALFDIMINISHED", B, HalfDiminished, Triad),
            ("Cb HalfDiminished seventh", B, HalfDiminished, Seventh),
            ("C HalfDiminished", C, HalfDiminished, Triad),
            ("C HalfDiminished Triad", C, HalfDiminished, Triad),
            ("C HalfDiminished Seventh", C, HalfDiminished, Seventh),
            ("C HalfDiminished Ninth", C, HalfDiminished, Ninth),
            ("C# HalfDiminished Eleventh", Cs, HalfDiminished, Eleventh),
            (
                "Ds HalfDiminished Thirteenth",
                Ds,
                HalfDiminished,
                Thirteenth,
            ),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_dominant() {
        let table = vec![
            ("C dominant", C, Dominant, Triad),
            ("E DOMINANT", E, Dominant, Triad),
            ("C DOmInAnT", C, Dominant, Triad),
            ("Cb Dominant", B, Dominant, Triad),
            ("Cb Dominant seventh", B, Dominant, Seventh),
            ("C Dominant", C, Dominant, Triad),
            ("C Dominant Triad", C, Dominant, Triad),
            ("C Dominant Seventh", C, Dominant, Seventh),
            ("C Dominant Ninth", C, Dominant, Ninth),
            ("C# Dominant Eleventh", Cs, Dominant, Eleventh),
            ("Ds Dominant Thirteenth", Ds, Dominant, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_suspended() {
        let table = vec![
            ("C sus2", C, Suspended2, Triad),
            ("E sus2 triad", E, Suspended2, Triad),
            ("C sus4", C, Suspended4, Triad),
            ("Cb suspended4", B, Suspended4, Triad),
            ("Cb suspended2", B, Suspended2, Triad),
        ];

        assert_chords(table);
    }
}
