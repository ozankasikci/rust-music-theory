extern crate rust_music_theory as theory;
use theory::chord::{Chord, Number, Number::*, Quality, Quality::*};
use theory::note::{Pitch, NoteLetter::*, pitch};

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
            ("C Major", pitch(C, 0), Major, Triad),
            ("E MAJOR", pitch(E, 0), Major, Triad),
            ("C Maj", pitch(C, 0), Major, Triad),
            ("Cb MAJ", pitch(C, -1), Major, Triad),
            ("Cb MAJ Seventh", pitch(C, -1), Major, Seventh),
            ("C M", pitch(C, 0), Major, Triad),
            ("C MaJ Triad", pitch(C, 0), Major, Triad),
            ("C Major Seventh", pitch(C, 0), Major, Seventh),
            ("C M Ninth", pitch(C, 0), Major, Ninth),
            ("C M eleventh", pitch(C, 0), Major, Eleventh),
            ("C M ThirTeenth", pitch(C, 0), Major, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_minor() {
        let table = vec![
            ("C Minor", pitch(C, 0), Minor, Triad),
            ("E MINOR", pitch(E, 0), Minor, Triad),
            ("C Min", pitch(C, 0), Minor, Triad),
            ("Cb MIN", pitch(C, -1), Minor, Triad),
            ("Cb MIN Seventh", pitch(C, -1), Minor, Seventh),
            ("C m", pitch(C, 0), Minor, Triad),
            ("C MiN Triad", pitch(C, 0), Minor, Triad),
            ("C Minor Seventh", pitch(C, 0), Minor, Seventh),
            ("C m Ninth", pitch(C, 0), Minor, Ninth),
            ("C#m Eleventh", pitch(C, 1), Minor, Eleventh),
            ("Dsm Thirteenth", pitch(D, 1), Minor, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_augmented() {
        let table = vec![
            ("C augmented", pitch(C, 0), Augmented, Triad),
            ("E Augmented", pitch(E, 0), Augmented, Triad),
            ("C augmented", pitch(C, 0), Augmented, Triad),
            ("Cb augmented", pitch(C, -1), Augmented, Triad),
            ("Cb augmented seventh", pitch(C, -1), Augmented, Seventh),
            ("C augmented", pitch(C, 0), Augmented, Triad),
            ("C augmented Triad", pitch(C, 0), Augmented, Triad),
            ("C Augmented Seventh", pitch(C, 0), Augmented, Seventh),
            ("C Augmented Ninth", pitch(C, 0), Augmented, Ninth),
            ("C# augmented Eleventh", pitch(C, 1), Augmented, Eleventh),
            ("Ds augmented Thirteenth", pitch(D, 1), Augmented, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_diminished() {
        let table = vec![
            ("C Diminished", pitch(C, 0), Diminished, Triad),
            ("E Diminished", pitch(E, 0), Diminished, Triad),
            ("C Diminished", pitch(C, 0), Diminished, Triad),
            ("Cb Diminished", pitch(C, -1), Diminished, Triad),
            ("Cb Diminished seventh", pitch(C, -1), Diminished, Seventh),
            ("C Diminished", pitch(C, 0), Diminished, Triad),
            ("C Diminished Triad", pitch(C, 0), Diminished, Triad),
            ("C Diminished Seventh", pitch(C, 0), Diminished, Seventh),
            ("C Diminished Ninth", pitch(C, 0), Diminished, Ninth),
            ("C# Diminished Eleventh", pitch(C, 1), Diminished, Eleventh),
            ("Ds Diminished Thirteenth", pitch(D, 1), Diminished, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_half_diminished() {
        let table = vec![
            ("C Half Diminished", pitch(C, 0), HalfDiminished, Triad),
            ("E halfdiminished", pitch(E, 0), HalfDiminished, Triad),
            ("C half diminished", pitch(C, 0), HalfDiminished, Triad),
            ("Cb HALFDIMINISHED", pitch(C, -1), HalfDiminished, Triad),
            ("Cb HalfDiminished seventh", pitch(C, -1), HalfDiminished, Seventh),
            ("C HalfDiminished", pitch(C, 0), HalfDiminished, Triad),
            ("C HalfDiminished Triad", pitch(C, 0), HalfDiminished, Triad),
            ("C HalfDiminished Seventh", pitch(C, 0), HalfDiminished, Seventh),
            ("C HalfDiminished Ninth", pitch(C, 0), HalfDiminished, Ninth),
            ("C# HalfDiminished Eleventh", pitch(C, 1), HalfDiminished, Eleventh),
            (
                "Ds HalfDiminished Thirteenth",
                pitch(D, 1),
                HalfDiminished,
                Thirteenth,
            ),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_dominant() {
        let table = vec![
            ("C dominant", pitch(C, 0), Dominant, Triad),
            ("E DOMINANT", pitch(E, 0), Dominant, Triad),
            ("C DOmInAnT", pitch(C, 0), Dominant, Triad),
            ("Cb Dominant", pitch(C, -1), Dominant, Triad),
            ("Cb Dominant seventh", pitch(C, -1), Dominant, Seventh),
            ("C Dominant", pitch(C, 0), Dominant, Triad),
            ("C Dominant Triad", pitch(C, 0), Dominant, Triad),
            ("C Dominant Seventh", pitch(C, 0), Dominant, Seventh),
            ("C Dominant Ninth", pitch(C, 0), Dominant, Ninth),
            ("C# Dominant Eleventh", pitch(C, 1), Dominant, Eleventh),
            ("Ds Dominant Thirteenth", pitch(D, 1), Dominant, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_suspended() {
        let table = vec![
            ("C sus2", pitch(C, 0), Suspended2, Triad),
            ("E sus2 triad", pitch(E, 0), Suspended2, Triad),
            ("C sus4", pitch(C, 0), Suspended4, Triad),
            ("Cb suspended4", pitch(C, -1), Suspended4, Triad),
            ("Cb suspended2", pitch(C, -1), Suspended2, Triad),
        ];

        assert_chords(table);
    }
}
