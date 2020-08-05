extern crate rust_music_theory as theory;
use theory::chord::{Chord, Number, Number::*, Quality, Quality::*};
use theory::note::{PitchClass, PitchSymbol::*, pclass};

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
            ("C Major", pclass(C,0), Major, Triad),
            ("E MAJOR", pclass(E,0), Major, Triad),
            ("C Maj", pclass(C,0), Major, Triad),
            ("Cb MAJ", pclass(C,-1), Major, Triad),
            ("Cb MAJ Seventh", pclass(C,-1), Major, Seventh),
            ("C M", pclass(C,0), Major, Triad),
            ("C MaJ Triad", pclass(C,0), Major, Triad),
            ("C Major Seventh", pclass(C,0), Major, Seventh),
            ("C M Ninth", pclass(C,0), Major, Ninth),
            ("C M eleventh", pclass(C,0), Major, Eleventh),
            ("C M ThirTeenth", pclass(C,0), Major, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_minor() {
        let table = vec![
            ("C Minor", pclass(C,0), Minor, Triad),
            ("E MINOR", pclass(E,0), Minor, Triad),
            ("C Min", pclass(C,0), Minor, Triad),
            ("Cb MIN", pclass(C,-1), Minor, Triad),
            ("Cb MIN Seventh", pclass(C,-1), Minor, Seventh),
            ("C m", pclass(C,0), Minor, Triad),
            ("C MiN Triad", pclass(C,0), Minor, Triad),
            ("C Minor Seventh", pclass(C,0), Minor, Seventh),
            ("C m Ninth", pclass(C,0), Minor, Ninth),
            ("C#m Eleventh", pclass(C,1), Minor, Eleventh),
            ("Dsm Thirteenth", pclass(D,1), Minor, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_augmented() {
        let table = vec![
            ("C augmented", pclass(C,0), Augmented, Triad),
            ("E Augmented", pclass(E,0), Augmented, Triad),
            ("C augmented", pclass(C,0), Augmented, Triad),
            ("Cb augmented", pclass(C,-1), Augmented, Triad),
            ("Cb augmented seventh", pclass(C,-1), Augmented, Seventh),
            ("C augmented", pclass(C,0), Augmented, Triad),
            ("C augmented Triad", pclass(C,0), Augmented, Triad),
            ("C Augmented Seventh", pclass(C,0), Augmented, Seventh),
            ("C Augmented Ninth", pclass(C,0), Augmented, Ninth),
            ("C# augmented Eleventh", pclass(C,1), Augmented, Eleventh),
            ("Ds augmented Thirteenth", pclass(D,1), Augmented, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_diminished() {
        let table = vec![
            ("C Diminished", pclass(C,0), Diminished, Triad),
            ("E Diminished", pclass(E,0), Diminished, Triad),
            ("C Diminished", pclass(C,0), Diminished, Triad),
            ("Cb Diminished", pclass(C,-1), Diminished, Triad),
            ("Cb Diminished seventh", pclass(C,-1), Diminished, Seventh),
            ("C Diminished", pclass(C,0), Diminished, Triad),
            ("C Diminished Triad", pclass(C,0), Diminished, Triad),
            ("C Diminished Seventh", pclass(C,0), Diminished, Seventh),
            ("C Diminished Ninth", pclass(C,0), Diminished, Ninth),
            ("C# Diminished Eleventh", pclass(C,1), Diminished, Eleventh),
            ("Ds Diminished Thirteenth", pclass(D,1), Diminished, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_half_diminished() {
        let table = vec![
            ("C Half Diminished", pclass(C,0), HalfDiminished, Triad),
            ("E halfdiminished", pclass(E,0), HalfDiminished, Triad),
            ("C half diminished", pclass(C,0), HalfDiminished, Triad),
            ("Cb HALFDIMINISHED", pclass(C,-1), HalfDiminished, Triad),
            ("Cb HalfDiminished seventh", pclass(C,-1), HalfDiminished, Seventh),
            ("C HalfDiminished", pclass(C,0), HalfDiminished, Triad),
            ("C HalfDiminished Triad", pclass(C,0), HalfDiminished, Triad),
            ("C HalfDiminished Seventh", pclass(C,0), HalfDiminished, Seventh),
            ("C HalfDiminished Ninth", pclass(C,0), HalfDiminished, Ninth),
            ("C# HalfDiminished Eleventh", pclass(C,1), HalfDiminished, Eleventh),
            (
                "Ds HalfDiminished Thirteenth",
                pclass(D,1),
                HalfDiminished,
                Thirteenth,
            ),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_dominant() {
        let table = vec![
            ("C dominant", pclass(C,0), Dominant, Triad),
            ("E DOMINANT", pclass(E,0), Dominant, Triad),
            ("C DOmInAnT", pclass(C,0), Dominant, Triad),
            ("Cb Dominant", pclass(C,-1), Dominant, Triad),
            ("Cb Dominant seventh", pclass(C,-1), Dominant, Seventh),
            ("C Dominant", pclass(C,0), Dominant, Triad),
            ("C Dominant Triad", pclass(C,0), Dominant, Triad),
            ("C Dominant Seventh", pclass(C,0), Dominant, Seventh),
            ("C Dominant Ninth", pclass(C,0), Dominant, Ninth),
            ("C# Dominant Eleventh", pclass(C,1), Dominant, Eleventh),
            ("Ds Dominant Thirteenth", pclass(D,1), Dominant, Thirteenth),
        ];

        assert_chords(table);
    }

    #[test]
    fn test_suspended() {
        let table = vec![
            ("C sus2", pclass(C,0), Suspended2, Triad),
            ("E sus2 triad", pclass(E,0), Suspended2, Triad),
            ("C sus4", pclass(C,0), Suspended4, Triad),
            ("Cb suspended4", pclass(C,-1), Suspended4, Triad),
            ("Cb suspended2", pclass(C,-1), Suspended2, Triad),
        ];

        assert_chords(table);
    }
}
