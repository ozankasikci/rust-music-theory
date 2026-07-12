use super::parsers::{parse_chord_number, parse_chord_quality, parse_pitch_symbol};
use super::types::{WasmChord, WasmNote};
use crate::chord::{Chord, ChordModifier, Suspension};
use crate::note::{Notes, Pitch};
use wasm_bindgen::prelude::*;

/// Parse a complete lead-sheet chord symbol.
///
/// JavaScript receives an exception containing the structured Rust parse
/// error instead of an ambiguous `null` value.
#[wasm_bindgen]
pub fn parse_chord_symbol(symbol: &str) -> Result<JsValue, JsValue> {
    let chord = Chord::parse(symbol).map_err(|error| JsValue::from_str(&error.to_string()))?;
    serde_wasm_bindgen::to_value(&to_wasm_chord(&chord))
        .map_err(|error| JsValue::from_str(&error.to_string()))
}

/// Generate a chord from the legacy root/quality/number parameters.
#[wasm_bindgen]
pub fn generate_chord(root: &str, quality: &str, number: &str) -> JsValue {
    let pitch_symbol = parse_pitch_symbol(root);
    let chord_quality = parse_chord_quality(quality);
    let chord_number = parse_chord_number(number);

    let chord = match Chord::try_new(Pitch::from(pitch_symbol), chord_quality, chord_number) {
        Ok(chord) => chord,
        Err(_) => return JsValue::NULL,
    };
    serde_wasm_bindgen::to_value(&to_wasm_chord(&chord)).unwrap_or(JsValue::NULL)
}

fn to_wasm_chord(chord: &Chord) -> WasmChord {
    let notes = chord.notes().into_iter().map(WasmNote::from).collect();
    let mut modifiers = Vec::new();
    if let Some(suspension) = chord.spec().suspension() {
        modifiers.push(match suspension {
            Suspension::Second => "sus2".to_string(),
            Suspension::Fourth => "sus4".to_string(),
        });
    }
    modifiers.extend(chord.spec().modifiers().iter().map(format_modifier));
    let formula = chord
        .formula()
        .tones()
        .iter()
        .map(|tone| format_tone(tone.degree(), tone.alteration()))
        .collect();
    WasmChord {
        notes,
        root: chord.root().to_string(),
        quality: chord.quality().to_string(),
        number: chord.number().to_string(),
        canonical_symbol: chord.canonical_symbol(),
        bass: chord.bass().map(|bass| bass.to_string()),
        modifiers,
        formula,
    }
}

fn format_tone(degree: u8, alteration: i8) -> String {
    let accidental = if alteration < 0 { 'b' } else { '#' };
    let prefix: String = (0..alteration.unsigned_abs()).map(|_| accidental).collect();
    format!("{}{}", prefix, degree)
}

fn format_modifier(modifier: &ChordModifier) -> String {
    match modifier {
        ChordModifier::Add(tone) => {
            format!("add{}", format_tone(tone.degree(), tone.alteration()))
        }
        ChordModifier::Alter(tone) => format_tone(tone.degree(), tone.alteration()),
        ChordModifier::Omit(degree) => format!("no{}", degree),
        ChordModifier::Altered => "alt".to_string(),
    }
}

/// Get the legacy quality names accepted by [`generate_chord`].
#[wasm_bindgen]
pub fn get_available_chord_qualities() -> JsValue {
    let qualities = vec![
        "major",
        "minor",
        "diminished",
        "augmented",
        "dominant",
        "half_diminished",
        "sus2",
        "sus4",
        "power",
    ];
    serde_wasm_bindgen::to_value(&qualities).unwrap_or(JsValue::NULL)
}

/// Get the legacy number names accepted by [`generate_chord`].
#[wasm_bindgen]
pub fn get_available_chord_numbers() -> JsValue {
    let numbers = vec![
        "triad",
        "fifth",
        "sixth",
        "six_nine",
        "seventh",
        "major_seventh",
        "ninth",
        "eleventh",
        "thirteenth",
    ];
    serde_wasm_bindgen::to_value(&numbers).unwrap_or(JsValue::NULL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::{Number, Quality};

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_chord_generation_logic() {
        let chord = Chord::new(
            Pitch::from(parse_pitch_symbol("C")),
            parse_chord_quality("major"),
            parse_chord_number("triad"),
        );
        assert_eq!(chord.notes().len(), 3);
        assert_eq!(chord.quality(), Quality::Major);
        assert_eq!(chord.number(), Number::Triad);
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_complete_symbol_contains_normalized_metadata() {
        let chord: WasmChord =
            serde_wasm_bindgen::from_value(parse_chord_symbol("C7(b9,#11)/G").unwrap()).unwrap();
        assert_eq!(chord.canonical_symbol, "C7b9#11/G");
        assert_eq!(chord.bass.as_deref(), Some("G"));
        assert_eq!(chord.modifiers, vec!["b9", "#11"]);
        assert_eq!(chord.formula, vec!["1", "3", "5", "b7", "b9", "#11"]);
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_complete_symbol_returns_an_error() {
        assert!(parse_chord_symbol("C7altb9").is_err());
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_suspension_is_exposed_as_a_modifier() {
        let chord: WasmChord =
            serde_wasm_bindgen::from_value(parse_chord_symbol("C7sus4").unwrap()).unwrap();
        assert_eq!(chord.modifiers, vec!["sus4"]);
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_aliases_and_compound_degrees_match_the_rust_api() {
        let alias: WasmChord =
            serde_wasm_bindgen::from_value(parse_chord_symbol("C+M7").unwrap()).unwrap();
        assert_eq!(alias.canonical_symbol, "CaugMaj7");
        assert_eq!(alias.formula, vec!["1", "3", "#5", "7"]);

        let added: WasmChord =
            serde_wasm_bindgen::from_value(parse_chord_symbol("Cadd13").unwrap()).unwrap();
        assert_eq!(added.notes.last().unwrap().pitch, "A");
        assert_eq!(added.notes.last().unwrap().octave, 5);
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_malformed_modifier_separators_return_an_error() {
        assert!(parse_chord_symbol("C7(b9,,#11)").is_err());
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_second_audit_unicode_and_power_chord_regressions() {
        let chord: WasmChord =
            serde_wasm_bindgen::from_value(parse_chord_symbol("C𝄫maj7").unwrap()).unwrap();
        assert_eq!(chord.canonical_symbol, "Cbbmaj7");
        assert_eq!(
            chord
                .notes
                .iter()
                .map(|note| (note.pitch.as_str(), note.octave))
                .collect::<Vec<_>>(),
            vec![("Cbb", 4), ("Ebb", 4), ("Gbb", 4), ("Bbb", 4)]
        );
        assert!(parse_chord_symbol("C5add9").is_err());
        assert!(parse_chord_symbol("CmMaj6").is_err());
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_legacy_generation_and_lists() {
        let chord: WasmChord =
            serde_wasm_bindgen::from_value(generate_chord("C", "augmented", "seventh")).unwrap();
        assert_eq!(
            chord
                .notes
                .iter()
                .map(|note| note.pitch.as_str())
                .collect::<Vec<_>>(),
            vec!["C", "E", "G#", "Bb"]
        );
        assert_eq!(chord.canonical_symbol, "Caug7");

        let qualities: Vec<String> =
            serde_wasm_bindgen::from_value(get_available_chord_qualities()).unwrap();
        let numbers: Vec<String> =
            serde_wasm_bindgen::from_value(get_available_chord_numbers()).unwrap();
        assert!(qualities.contains(&"power".to_string()));
        assert!(numbers.contains(&"six_nine".to_string()));
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_legacy_generation_rejects_unsupported_combinations() {
        assert!(generate_chord("C", "diminished", "ninth").is_null());
    }
}
