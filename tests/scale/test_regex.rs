extern crate rust_music_theory as theory;
use theory::note::{NoteLetter::*, pitch};
use theory::scale::{Mode, Scale, ScaleType};

#[cfg(test)]
mod chord_regex_tests {
    use super::*;

    #[test]
    fn test_all_scales() {
        let table = vec![
            ("C Major", pitch(C, 0), ScaleType::Diatonic, Mode::Ionian),
            ("CM", pitch(C, 0), ScaleType::Diatonic, Mode::Ionian),
            ("C Maj", pitch(C, 0), ScaleType::Diatonic, Mode::Ionian),
            ("C MAJOR", pitch(C, 0), ScaleType::Diatonic, Mode::Ionian),
            ("As locrian", pitch(A, 1), ScaleType::Diatonic, Mode::Locrian),
            ("Bs phrygian", pitch(B, 1), ScaleType::Diatonic, Mode::Phrygian),
            ("E lydian", pitch(E, 0), ScaleType::Diatonic, Mode::Lydian),
            ("F dorian", pitch(F, 0), ScaleType::Diatonic, Mode::Dorian),
            ("Gb mixolydian", pitch(G, -1), ScaleType::Diatonic, Mode::Mixolydian),
            ("B MAJOR", pitch(B, 0), ScaleType::Diatonic, Mode::Ionian),
            ("Bb MAJOR", pitch(B, -1), ScaleType::Diatonic, Mode::Ionian),
            (
                "Bb Harmonic Minor",
                pitch(B, -1),
                ScaleType::HarmonicMinor,
                Mode::HarmonicMinor,
            ),
            (
                "Ds Melodic Minor",
                pitch(D, 1),
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
