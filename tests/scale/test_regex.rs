extern crate rust_music_theory as theory;
use theory::note::{PitchSymbol::*, pclass};
use theory::scale::{Mode, Scale, ScaleType};

#[cfg(test)]
mod chord_regex_tests {
    use super::*;

    #[test]
    fn test_all_scales() {
        let table = vec![
            ("C Major", pclass(C,0), ScaleType::Diatonic, Mode::Ionian),
            ("CM", pclass(C,0), ScaleType::Diatonic, Mode::Ionian),
            ("C Maj", pclass(C,0), ScaleType::Diatonic, Mode::Ionian),
            ("C MAJOR", pclass(C,0), ScaleType::Diatonic, Mode::Ionian),
            ("As locrian", pclass(A,1), ScaleType::Diatonic, Mode::Locrian),
            ("Bs phrygian", pclass(B,1), ScaleType::Diatonic, Mode::Phrygian),
            ("E lydian", pclass(E,0), ScaleType::Diatonic, Mode::Lydian),
            ("F dorian", pclass(F,0), ScaleType::Diatonic, Mode::Dorian),
            ("Gb mixolydian", pclass(G,-1), ScaleType::Diatonic, Mode::Mixolydian),
            ("B MAJOR", pclass(B,0), ScaleType::Diatonic, Mode::Ionian),
            ("Bb MAJOR", pclass(B,-1), ScaleType::Diatonic, Mode::Ionian),
            (
                "Bb Harmonic Minor",
                pclass(B,-1),
                ScaleType::HarmonicMinor,
                Mode::HarmonicMinor,
            ),
            (
                "Ds Melodic Minor",
                pclass(D,1),
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
