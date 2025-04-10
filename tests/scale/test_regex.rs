extern crate rust_music_theory as theory;
use theory::note::{NoteLetter::*, Pitch};
use theory::scale::{Mode, Scale, ScaleType};

#[cfg(test)]
mod chord_regex_tests {
    use super::*;

    #[test]
    fn test_all_scales() {
        let table = vec![
            (
                "C Major",
                Pitch::new(C, 0),
                ScaleType::Diatonic,
                Mode::Ionian,
            ),
            ("CM", Pitch::new(C, 0), ScaleType::Diatonic, Mode::Ionian),
            ("C Maj", Pitch::new(C, 0), ScaleType::Diatonic, Mode::Ionian),
            (
                "C MAJOR",
                Pitch::new(C, 0),
                ScaleType::Diatonic,
                Mode::Ionian,
            ),
            (
                "As locrian",
                Pitch::new(A, 1),
                ScaleType::Diatonic,
                Mode::Locrian,
            ),
            (
                "Bs phrygian",
                Pitch::new(B, 1),
                ScaleType::Diatonic,
                Mode::Phrygian,
            ),
            (
                "E lydian",
                Pitch::new(E, 0),
                ScaleType::Diatonic,
                Mode::Lydian,
            ),
            (
                "F dorian",
                Pitch::new(F, 0),
                ScaleType::Diatonic,
                Mode::Dorian,
            ),
            (
                "Gb mixolydian",
                Pitch::new(G, -1),
                ScaleType::Diatonic,
                Mode::Mixolydian,
            ),
            (
                "B MAJOR",
                Pitch::new(B, 0),
                ScaleType::Diatonic,
                Mode::Ionian,
            ),
            (
                "Bb MAJOR",
                Pitch::new(B, -1),
                ScaleType::Diatonic,
                Mode::Ionian,
            ),
            (
                "Bb Harmonic Minor",
                Pitch::new(B, -1),
                ScaleType::HarmonicMinor,
                Mode::HarmonicMinor,
            ),
            (
                "Ds Melodic Minor",
                Pitch::new(D, 1),
                ScaleType::MelodicMinor,
                Mode::MelodicMinor,
            ),
        ];

        for (string, pitch, scale_type, mode) in table {
            let scale = Scale::from_regex(string).unwrap();
            assert_eq!(scale.mode.unwrap(), mode);
            assert_eq!(scale.tonic, pitch);
            assert_eq!(scale.scale_type, scale_type);
        }
    }
}
