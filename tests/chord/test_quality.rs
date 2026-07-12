extern crate rust_music_theory as theory;
use theory::chord::Quality;

#[cfg(test)]
mod chord_quality_tests {
    use super::*;

    #[test]
    fn test_explicit_qualities() {
        let cases = [
            ("major", Quality::Major),
            ("minor", Quality::Minor),
            ("dim", Quality::Diminished),
            ("augmented", Quality::Augmented),
            ("half diminished", Quality::HalfDiminished),
            ("dominant", Quality::Dominant),
            ("sus2", Quality::Suspended2),
            ("sus4", Quality::Suspended4),
        ];
        for (input, expected) in cases {
            assert_eq!(Quality::from_regex(input).unwrap().0, expected);
        }
    }

    #[test]
    fn test_omitted_quality_defaults_to_major() {
        let (quality, matched) = Quality::from_regex("").unwrap();
        assert_eq!(quality, Quality::Major);
        assert!(matched.is_none());
    }
}
