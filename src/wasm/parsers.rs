use crate::note::PitchSymbol;
use crate::scale::{ScaleType, Mode};
use crate::chord::{Quality as ChordQuality, Number as ChordNumber};

/// Parse a string representation of a pitch into a PitchSymbol
pub fn parse_pitch_symbol(input: &str) -> PitchSymbol {
    match input.to_uppercase().as_str() {
        "C" => PitchSymbol::C,
        "C#" | "CS" => PitchSymbol::Cs,
        "DB" => PitchSymbol::Db,
        "D" => PitchSymbol::D,
        "D#" | "DS" => PitchSymbol::Ds,
        "EB" => PitchSymbol::Eb,
        "E" => PitchSymbol::E,
        "F" => PitchSymbol::F,
        "F#" | "FS" => PitchSymbol::Fs,
        "GB" => PitchSymbol::Gb,
        "G" => PitchSymbol::G,
        "G#" | "GS" => PitchSymbol::Gs,
        "AB" => PitchSymbol::Ab,
        "A" => PitchSymbol::A,
        "A#" | "AS" => PitchSymbol::As,
        "BB" => PitchSymbol::Bb,
        "B" => PitchSymbol::B,
        _ => PitchSymbol::C, // default to C
    }
}

/// Parse a string representation of a scale type into a ScaleType
pub fn parse_scale_type(input: &str) -> ScaleType {
    match input.to_lowercase().as_str() {
        "diatonic" => ScaleType::Diatonic,
        "pentatonic_major" => ScaleType::PentatonicMajor,
        "pentatonic_minor" => ScaleType::PentatonicMinor,
        "blues" => ScaleType::Blues,
        "chromatic" => ScaleType::Chromatic,
        "whole_tone" => ScaleType::WholeTone,
        "harmonic_minor" => ScaleType::HarmonicMinor,
        "melodic_minor" => ScaleType::MelodicMinor,
        _ => ScaleType::Diatonic, // default
    }
}

/// Parse a string representation of a mode into a Mode
pub fn parse_mode(input: &str) -> Option<Mode> {
    match input.to_lowercase().as_str() {
        "ionian" => Some(Mode::Ionian),
        "dorian" => Some(Mode::Dorian),
        "phrygian" => Some(Mode::Phrygian),
        "lydian" => Some(Mode::Lydian),
        "mixolydian" => Some(Mode::Mixolydian),
        "aeolian" => Some(Mode::Aeolian),
        "locrian" => Some(Mode::Locrian),
        _ => None,
    }
}

/// Parse a string representation of a chord quality into a ChordQuality
pub fn parse_chord_quality(input: &str) -> ChordQuality {
    match input.to_lowercase().as_str() {
        "major" => ChordQuality::Major,
        "minor" => ChordQuality::Minor,
        "diminished" => ChordQuality::Diminished,
        "augmented" => ChordQuality::Augmented,
        "dominant" => ChordQuality::Dominant,
        "half_diminished" => ChordQuality::HalfDiminished,
        "sus2" | "suspended2" => ChordQuality::Suspended2,
        "sus4" | "suspended4" => ChordQuality::Suspended4,
        _ => ChordQuality::Major,
    }
}

/// Parse a string representation of a chord number into a ChordNumber
pub fn parse_chord_number(input: &str) -> ChordNumber {
    match input.to_lowercase().as_str() {
        "triad" => ChordNumber::Triad,
        "seventh" => ChordNumber::Seventh,
        "ninth" => ChordNumber::Ninth,
        "eleventh" => ChordNumber::Eleventh,
        "thirteenth" => ChordNumber::Thirteenth,
        _ => ChordNumber::Triad,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pitch_symbol() {
        assert_eq!(parse_pitch_symbol("C"), PitchSymbol::C);
        assert_eq!(parse_pitch_symbol("c"), PitchSymbol::C);
        assert_eq!(parse_pitch_symbol("C#"), PitchSymbol::Cs);
        assert_eq!(parse_pitch_symbol("CS"), PitchSymbol::Cs);
        assert_eq!(parse_pitch_symbol("cs"), PitchSymbol::Cs);
        assert_eq!(parse_pitch_symbol("DB"), PitchSymbol::Db);
        assert_eq!(parse_pitch_symbol("db"), PitchSymbol::Db);
        assert_eq!(parse_pitch_symbol("F#"), PitchSymbol::Fs);
        assert_eq!(parse_pitch_symbol("FS"), PitchSymbol::Fs);
        assert_eq!(parse_pitch_symbol("BB"), PitchSymbol::Bb);
        assert_eq!(parse_pitch_symbol("invalid"), PitchSymbol::C); // default
    }

    #[test]
    fn test_parse_scale_type() {
        assert_eq!(parse_scale_type("diatonic"), ScaleType::Diatonic);
        assert_eq!(parse_scale_type("DIATONIC"), ScaleType::Diatonic);
        assert_eq!(parse_scale_type("pentatonic_major"), ScaleType::PentatonicMajor);
        assert_eq!(parse_scale_type("blues"), ScaleType::Blues);
        assert_eq!(parse_scale_type("chromatic"), ScaleType::Chromatic);
        assert_eq!(parse_scale_type("harmonic_minor"), ScaleType::HarmonicMinor);
        assert_eq!(parse_scale_type("invalid"), ScaleType::Diatonic); // default
    }

    #[test]
    fn test_parse_mode() {
        assert_eq!(parse_mode("ionian"), Some(Mode::Ionian));
        assert_eq!(parse_mode("IONIAN"), Some(Mode::Ionian));
        assert_eq!(parse_mode("dorian"), Some(Mode::Dorian));
        assert_eq!(parse_mode("phrygian"), Some(Mode::Phrygian));
        assert_eq!(parse_mode("lydian"), Some(Mode::Lydian));
        assert_eq!(parse_mode("mixolydian"), Some(Mode::Mixolydian));
        assert_eq!(parse_mode("aeolian"), Some(Mode::Aeolian));
        assert_eq!(parse_mode("locrian"), Some(Mode::Locrian));
        assert_eq!(parse_mode("invalid"), None);
    }

    #[test]
    fn test_parse_chord_quality() {
        assert_eq!(parse_chord_quality("major"), ChordQuality::Major);
        assert_eq!(parse_chord_quality("MAJOR"), ChordQuality::Major);
        assert_eq!(parse_chord_quality("minor"), ChordQuality::Minor);
        assert_eq!(parse_chord_quality("diminished"), ChordQuality::Diminished);
        assert_eq!(parse_chord_quality("augmented"), ChordQuality::Augmented);
        assert_eq!(parse_chord_quality("dominant"), ChordQuality::Dominant);
        assert_eq!(parse_chord_quality("sus2"), ChordQuality::Suspended2);
        assert_eq!(parse_chord_quality("suspended2"), ChordQuality::Suspended2);
        assert_eq!(parse_chord_quality("sus4"), ChordQuality::Suspended4);
        assert_eq!(parse_chord_quality("invalid"), ChordQuality::Major); // default
    }

    #[test]
    fn test_parse_chord_number() {
        assert_eq!(parse_chord_number("triad"), ChordNumber::Triad);
        assert_eq!(parse_chord_number("TRIAD"), ChordNumber::Triad);
        assert_eq!(parse_chord_number("seventh"), ChordNumber::Seventh);
        assert_eq!(parse_chord_number("ninth"), ChordNumber::Ninth);
        assert_eq!(parse_chord_number("eleventh"), ChordNumber::Eleventh);
        assert_eq!(parse_chord_number("thirteenth"), ChordNumber::Thirteenth);
        assert_eq!(parse_chord_number("invalid"), ChordNumber::Triad); // default
    }
}