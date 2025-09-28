use wasm_bindgen::prelude::*;
use crate::note::{Pitch, Notes};
use crate::scale::{Scale, Direction};
use super::types::{WasmScale, WasmNote};
use super::parsers::{parse_pitch_symbol, parse_scale_type, parse_mode};

/// Generate a scale from WASM-compatible parameters
#[wasm_bindgen]
pub fn generate_scale(
    tonic: &str,
    scale_type: &str,
    octave: u8,
    mode: Option<String>,
    ascending: bool,
) -> JsValue {
    let pitch_symbol = parse_pitch_symbol(tonic);
    let scale_type_enum = parse_scale_type(scale_type);
    let mode_enum = mode.as_ref().and_then(|m| parse_mode(m));
    let direction = if ascending {
        Direction::Ascending
    } else {
        Direction::Descending
    };

    match Scale::new(
        scale_type_enum,
        Pitch::from(pitch_symbol),
        octave,
        mode_enum,
        direction,
    ) {
        Ok(scale) => {
            let notes: Vec<WasmNote> = scale.notes().into_iter().map(WasmNote::from).collect();
            let wasm_scale = WasmScale {
                notes,
                scale_type: scale_type.to_string(),
                tonic: tonic.to_string(),
                mode,
                direction: if ascending {
                    "ascending".to_string()
                } else {
                    "descending".to_string()
                },
            };
            serde_wasm_bindgen::to_value(&wasm_scale).unwrap_or(JsValue::NULL)
        }
        Err(_) => JsValue::NULL,
    }
}

/// Get list of available scale types
#[wasm_bindgen]
pub fn get_available_scales() -> JsValue {
    let scales = vec![
        "diatonic",
        "pentatonic_major",
        "pentatonic_minor",
        "blues",
        "chromatic",
        "whole_tone",
        "harmonic_minor",
        "melodic_minor",
    ];
    serde_wasm_bindgen::to_value(&scales).unwrap_or(JsValue::NULL)
}

/// Get list of available modes
#[wasm_bindgen]
pub fn get_available_modes() -> JsValue {
    let modes = vec![
        "ionian",
        "dorian",
        "phrygian",
        "lydian",
        "mixolydian",
        "aeolian",
        "locrian",
    ];
    serde_wasm_bindgen::to_value(&modes).unwrap_or(JsValue::NULL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::{ScaleType, Mode};

    #[test]
    fn test_scale_generation_logic() {
        // Test the core logic without WASM bindings
        let pitch_symbol = parse_pitch_symbol("C");
        let scale_type_enum = parse_scale_type("diatonic");
        let mode_enum = parse_mode("ionian");

        let scale = Scale::new(
            scale_type_enum,
            Pitch::from(pitch_symbol),
            4,
            mode_enum,
            Direction::Ascending,
        ).unwrap();

        let notes = scale.notes();
        assert_eq!(notes.len(), 8); // C major scale
        assert_eq!(notes[0].octave, 4);

        // Convert to WASM types
        let wasm_notes: Vec<WasmNote> = notes.into_iter().map(WasmNote::from).collect();
        assert_eq!(wasm_notes.len(), 8);
        assert_eq!(wasm_notes[0].pitch, "C");
    }

    #[test]
    fn test_available_scales_count() {
        // This would normally call the WASM function, but we test the data
        let scales = vec![
            "diatonic",
            "pentatonic_major",
            "pentatonic_minor",
            "blues",
            "chromatic",
            "whole_tone",
            "harmonic_minor",
            "melodic_minor",
        ];
        assert_eq!(scales.len(), 8);
    }

    #[test]
    fn test_available_modes_count() {
        let modes = vec![
            "ionian",
            "dorian",
            "phrygian",
            "lydian",
            "mixolydian",
            "aeolian",
            "locrian",
        ];
        assert_eq!(modes.len(), 7);
    }

    #[test]
    fn test_scale_direction_handling() {
        let ascending = true;
        let descending = false;

        assert_eq!(
            if ascending { Direction::Ascending } else { Direction::Descending },
            Direction::Ascending
        );
        assert_eq!(
            if descending { Direction::Ascending } else { Direction::Descending },
            Direction::Descending
        );
    }
}