extern crate rust_music_theory as theory;
use theory::chord::{Chord, Number, Number::*, Quality, Quality::*};
use theory::note::{NoteLetter::*, Pitch};

fn assert_chords(table: Vec<(&str, Pitch, Quality, Number)>) {
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
            ("C Major", Pitch::new(C, 0), Major, Triad),
            ("E MAJOR", Pitch::new(E, 0), Major, Triad),
            ("C Maj", Pitch::new(C, 0), Major, Triad),
            ("Cb MAJ", Pitch::new(C, -1), Major, Triad),
            ("Cb MAJ Seventh", Pitch::new(C, -1), Major, Seventh),
            ("C M", Pitch::new(C, 0), Major, Triad),
            ("C MaJ Triad", Pitch::new(C, 0), Major, Triad),
            ("C Major Seventh", Pitch::new(C, 0), Major, Seventh),
            ("C M Ninth", Pitch::new(C, 0), Major, Ninth),
            ("C M eleventh", Pitch::new(C, 0), Major, Eleventh),
            ("C M ThirTeenth", Pitch::new(C, 0), Major, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_minor() {
        let table = vec![
            ("C Minor", Pitch::new(C, 0), Minor, Triad),
            ("E MINOR", Pitch::new(E, 0), Minor, Triad),
            ("C Min", Pitch::new(C, 0), Minor, Triad),
            ("Cb MIN", Pitch::new(C, -1), Minor, Triad),
            ("Cb MIN Seventh", Pitch::new(C, -1), Minor, Seventh),
            ("C m", Pitch::new(C, 0), Minor, Triad),
            ("C MiN Triad", Pitch::new(C, 0), Minor, Triad),
            ("C Minor Seventh", Pitch::new(C, 0), Minor, Seventh),
            ("C m Ninth", Pitch::new(C, 0), Minor, Ninth),
            ("C#m Eleventh", Pitch::new(C, 1), Minor, Eleventh),
            ("Dsm Thirteenth", Pitch::new(D, 1), Minor, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_augmented() {
        let table = vec![
            ("C augmented", Pitch::new(C, 0), Augmented, Triad),
            ("E Augmented", Pitch::new(E, 0), Augmented, Triad),
            ("C augmented", Pitch::new(C, 0), Augmented, Triad),
            ("Cb augmented", Pitch::new(C, -1), Augmented, Triad),
            (
                "Cb augmented seventh",
                Pitch::new(C, -1),
                Augmented,
                Seventh,
            ),
            ("C augmented", Pitch::new(C, 0), Augmented, Triad),
            ("C augmented Triad", Pitch::new(C, 0), Augmented, Triad),
            ("C Augmented Seventh", Pitch::new(C, 0), Augmented, Seventh),
            ("C Augmented Ninth", Pitch::new(C, 0), Augmented, Ninth),
            (
                "C# augmented Eleventh",
                Pitch::new(C, 1),
                Augmented,
                Eleventh,
            ),
            (
                "Ds augmented Thirteenth",
                Pitch::new(D, 1),
                Augmented,
                Thirteenth,
            ),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_diminished() {
        let table = vec![
            ("C Diminished", Pitch::new(C, 0), Diminished, Triad),
            ("E Diminished", Pitch::new(E, 0), Diminished, Triad),
            ("C Diminished", Pitch::new(C, 0), Diminished, Triad),
            ("Cb Diminished", Pitch::new(C, -1), Diminished, Triad),
            (
                "Cb Diminished seventh",
                Pitch::new(C, -1),
                Diminished,
                Seventh,
            ),
            ("C Diminished", Pitch::new(C, 0), Diminished, Triad),
            ("C Diminished Triad", Pitch::new(C, 0), Diminished, Triad),
            (
                "C Diminished Seventh",
                Pitch::new(C, 0),
                Diminished,
                Seventh,
            ),
            ("C Diminished Ninth", Pitch::new(C, 0), Diminished, Ninth),
            (
                "C# Diminished Eleventh",
                Pitch::new(C, 1),
                Diminished,
                Eleventh,
            ),
            (
                "Ds Diminished Thirteenth",
                Pitch::new(D, 1),
                Diminished,
                Thirteenth,
            ),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_half_diminished() {
        let table = vec![
            ("C Half Diminished", Pitch::new(C, 0), HalfDiminished, Triad),
            ("E halfdiminished", Pitch::new(E, 0), HalfDiminished, Triad),
            ("C half diminished", Pitch::new(C, 0), HalfDiminished, Triad),
            (
                "Cb HALFDIMINISHED",
                Pitch::new(C, -1),
                HalfDiminished,
                Triad,
            ),
            (
                "Cb HalfDiminished seventh",
                Pitch::new(C, -1),
                HalfDiminished,
                Seventh,
            ),
            ("C HalfDiminished", Pitch::new(C, 0), HalfDiminished, Triad),
            (
                "C HalfDiminished Triad",
                Pitch::new(C, 0),
                HalfDiminished,
                Triad,
            ),
            (
                "C HalfDiminished Seventh",
                Pitch::new(C, 0),
                HalfDiminished,
                Seventh,
            ),
            (
                "C HalfDiminished Ninth",
                Pitch::new(C, 0),
                HalfDiminished,
                Ninth,
            ),
            (
                "C# HalfDiminished Eleventh",
                Pitch::new(C, 1),
                HalfDiminished,
                Eleventh,
            ),
            (
                "Ds HalfDiminished Thirteenth",
                Pitch::new(D, 1),
                HalfDiminished,
                Thirteenth,
            ),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_dominant() {
        let table = vec![
            ("C dominant", Pitch::new(C, 0), Dominant, Triad),
            ("E DOMINANT", Pitch::new(E, 0), Dominant, Triad),
            ("C DOmInAnT", Pitch::new(C, 0), Dominant, Triad),
            ("Cb Dominant", Pitch::new(C, -1), Dominant, Triad),
            ("Cb Dominant seventh", Pitch::new(C, -1), Dominant, Seventh),
            ("C Dominant", Pitch::new(C, 0), Dominant, Triad),
            ("C Dominant Triad", Pitch::new(C, 0), Dominant, Triad),
            ("C Dominant Seventh", Pitch::new(C, 0), Dominant, Seventh),
            ("C Dominant Ninth", Pitch::new(C, 0), Dominant, Ninth),
            ("C# Dominant Eleventh", Pitch::new(C, 1), Dominant, Eleventh),
            (
                "Ds Dominant Thirteenth",
                Pitch::new(D, 1),
                Dominant,
                Thirteenth,
            ),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_suspended() {
        let table = vec![
            ("C sus2", Pitch::new(C, 0), Suspended2, Triad),
            ("E sus2 triad", Pitch::new(E, 0), Suspended2, Triad),
            ("C sus4", Pitch::new(C, 0), Suspended4, Triad),
            ("Cb suspended4", Pitch::new(C, -1), Suspended4, Triad),
            ("Cb suspended2", Pitch::new(C, -1), Suspended2, Triad),
        ];

        assert_chords(table);
    }
}
