extern crate rust_music_theory as theory;
use theory::scale::{Scale, Mode, ScaleType};
use theory::note::PitchClass::*;

#[cfg(test)]
mod chord_regex_tests {
    use super::*;

    #[test]
    fn test_all_scales() {
        let table = vec![
            ("C Major", C, ScaleType::Diatonic, Mode::Ionian),
            ("CM", C, ScaleType::Diatonic, Mode::Ionian),
            ("C Maj", C, ScaleType::Diatonic, Mode::Ionian),
            ("C MAJOR", C, ScaleType::Diatonic, Mode::Ionian),
            ("As locrian", As, ScaleType::Diatonic, Mode::Locrian),
            ("Bs phrygian", C, ScaleType::Diatonic, Mode::Phrygian),
            ("E lydian", E, ScaleType::Diatonic, Mode::Lydian),
            ("F dorian", F, ScaleType::Diatonic, Mode::Dorian),
            ("Gb mixolydian", Fs, ScaleType::Diatonic, Mode::Mixolydian),
            ("B MAJOR", B, ScaleType::Diatonic, Mode::Ionian),
            ("Bb MAJOR", As, ScaleType::Diatonic, Mode::Ionian),
            ("Bb Harmonic Minor", As, ScaleType::HarmonicMinor, Mode::HarmonicMinor),
            ("Ds Melodic Minor", Ds, ScaleType::MelodicMinor, Mode::MelodicMinor),
        ];

        for (string, pitch, scale_type, mode) in table {
            let scale = Scale::from_regex(string).unwrap();
            assert_eq!(scale.mode.unwrap(), mode);
            assert_eq!(scale.tonic, pitch);
            assert_eq!(scale.scale_type, scale_type);
        }
    }
}
