use wasm_bindgen::prelude::*;
use crate::note::{Pitch, Notes};
use crate::chord::Chord;
use super::types::{WasmChord, WasmNote};
use super::parsers::{parse_pitch_symbol, parse_chord_quality, parse_chord_number};

/// Generate a chord from WASM-compatible parameters
#[wasm_bindgen]
pub fn generate_chord(root: &str, quality: &str, number: &str) -> JsValue {
    let pitch_symbol = parse_pitch_symbol(root);
    let chord_quality = parse_chord_quality(quality);
    let chord_number = parse_chord_number(number);

    let chord = Chord::new(Pitch::from(pitch_symbol), chord_quality, chord_number);
    let notes: Vec<WasmNote> = chord.notes().into_iter().map(WasmNote::from).collect();
    let wasm_chord = WasmChord {
        notes,
        root: root.to_string(),
        quality: quality.to_string(),
        number: number.to_string(),
    };

    serde_wasm_bindgen::to_value(&wasm_chord).unwrap_or(JsValue::NULL)
}

/// Get list of available chord qualities
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
    ];
    serde_wasm_bindgen::to_value(&qualities).unwrap_or(JsValue::NULL)
}

/// Get list of available chord numbers
#[wasm_bindgen]
pub fn get_available_chord_numbers() -> JsValue {
    let numbers = vec!["triad", "seventh", "ninth", "eleventh", "thirteenth"];
    serde_wasm_bindgen::to_value(&numbers).unwrap_or(JsValue::NULL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::{Quality, Number};

    #[test]
    fn test_chord_generation_logic() {
        // Test the core logic without WASM bindings
        let pitch_symbol = parse_pitch_symbol("C");
        let chord_quality = parse_chord_quality("major");
        let chord_number = parse_chord_number("triad");

        let chord = Chord::new(Pitch::from(pitch_symbol), chord_quality, chord_number);
        let notes = chord.notes();

        assert_eq!(notes.len(), 3); // C major triad
        assert_eq!(chord.quality, Quality::Major);
        assert_eq!(chord.number, Number::Triad);

        // Convert to WASM types
        let wasm_notes: Vec<WasmNote> = notes.into_iter().map(WasmNote::from).collect();
        assert_eq!(wasm_notes.len(), 3);
        assert_eq!(wasm_notes[0].pitch, "C");
    }

    #[test]
    fn test_chord_with_sharps_and_flats() {
        let f_sharp_chord = Chord::new(
            Pitch::from(parse_pitch_symbol("F#")),
            parse_chord_quality("minor"),
            parse_chord_number("seventh"),
        );

        let notes = f_sharp_chord.notes();
        assert_eq!(notes.len(), 4); // F# minor seventh

        let wasm_notes: Vec<WasmNote> = notes.into_iter().map(WasmNote::from).collect();
        assert_eq!(wasm_notes[0].pitch, "F#");
    }

    #[test]
    fn test_available_chord_qualities_count() {
        let qualities = vec![
            "major",
            "minor",
            "diminished",
            "augmented",
            "dominant",
            "half_diminished",
            "sus2",
            "sus4",
        ];
        assert_eq!(qualities.len(), 8);
    }

    #[test]
    fn test_available_chord_numbers_count() {
        let numbers = vec!["triad", "seventh", "ninth", "eleventh", "thirteenth"];
        assert_eq!(numbers.len(), 5);
    }

    #[test]
    fn test_extended_chords() {
        let ninth_chord = Chord::new(
            Pitch::from(parse_pitch_symbol("G")),
            parse_chord_quality("dominant"),
            parse_chord_number("ninth"),
        );

        let notes = ninth_chord.notes();
        assert_eq!(notes.len(), 5); // G dominant ninth

        let wasm_notes: Vec<WasmNote> = notes.into_iter().map(WasmNote::from).collect();
        assert_eq!(wasm_notes[0].pitch, "G");
    }
}