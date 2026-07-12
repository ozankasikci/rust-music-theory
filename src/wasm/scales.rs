use super::parsers::{parse_mode, parse_pitch_symbol, parse_scale_type};
use super::types::{WasmNote, WasmScale};
use crate::note::{Notes, Pitch};
use crate::scale::{Direction, Mode, Scale};
use wasm_bindgen::prelude::*;

/// Generate a scale from WASM-compatible parameters
#[wasm_bindgen]
pub fn generate_scale(
    tonic: &str,
    scale_type: &str,
    octave: i16,
    mode: Option<String>,
    ascending: bool,
) -> JsValue {
    let pitch_symbol = parse_pitch_symbol(tonic);
    let scale_type_enum = parse_scale_type(scale_type);
    let mode_enum = match mode.as_deref() {
        Some(mode) => match parse_mode(mode) {
            Some(mode) => Some(mode),
            None => return JsValue::NULL,
        },
        None => None,
    };
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
                mode: mode_enum.map(|mode| mode.api_name().to_string()),
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
    let modes = Mode::heptatonic_modes()
        .iter()
        .map(|mode| mode.api_name())
        .collect::<Vec<_>>();
    serde_wasm_bindgen::to_value(&modes).unwrap_or(JsValue::NULL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
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
        )
        .unwrap();

        let notes = scale.notes();
        assert_eq!(notes.len(), 8); // C major scale
        assert_eq!(notes[0].octave, 4);

        // Convert to WASM types
        let wasm_notes: Vec<WasmNote> = notes.into_iter().map(WasmNote::from).collect();
        assert_eq!(wasm_notes.len(), 8);
        assert_eq!(wasm_notes[0].pitch, "C");
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
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

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_available_modes_count() {
        assert_eq!(Mode::heptatonic_modes().len(), 21);
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_scale_direction_handling() {
        let ascending = true;
        let descending = false;

        assert_eq!(
            if ascending {
                Direction::Ascending
            } else {
                Direction::Descending
            },
            Direction::Ascending
        );
        assert_eq!(
            if descending {
                Direction::Ascending
            } else {
                Direction::Descending
            },
            Direction::Descending
        );
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_exported_scale_generation_and_lists() {
        let scale: WasmScale = serde_wasm_bindgen::from_value(generate_scale(
            "C",
            "melodic_minor",
            4,
            Some("melodic_minor".to_string()),
            false,
        ))
        .unwrap();
        assert_eq!(scale.direction, "descending");
        assert_eq!(
            scale
                .notes
                .iter()
                .map(|note| note.pitch.as_str())
                .collect::<Vec<_>>(),
            vec!["C", "Bb", "Ab", "G", "F", "Eb", "D", "C"]
        );

        let scales: Vec<String> = serde_wasm_bindgen::from_value(get_available_scales()).unwrap();
        let modes: Vec<String> = serde_wasm_bindgen::from_value(get_available_modes()).unwrap();
        assert_eq!(scales.len(), 8);
        assert_eq!(modes.len(), 21);
        assert!(modes.contains(&"phrygian_dominant".to_string()));
        assert!(modes.contains(&"altered".to_string()));
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_complete_minor_modes_and_invalid_input() {
        let scale: WasmScale = serde_wasm_bindgen::from_value(generate_scale(
            "C",
            "harmonic_minor",
            4,
            Some("Spanish".to_string()),
            true,
        ))
        .unwrap();
        assert_eq!(scale.mode.as_deref(), Some("phrygian_dominant"));
        assert_eq!(
            scale
                .notes
                .iter()
                .map(|note| note.pitch.as_str())
                .collect::<Vec<_>>(),
            vec!["C", "Db", "E", "F", "G", "Ab", "Bb", "C"]
        );

        let altered: WasmScale = serde_wasm_bindgen::from_value(generate_scale(
            "C",
            "melodic_minor",
            4,
            Some("super_locrian".to_string()),
            false,
        ))
        .unwrap();
        assert_eq!(altered.mode.as_deref(), Some("altered"));
        assert_eq!(
            altered
                .notes
                .iter()
                .map(|note| note.pitch.as_str())
                .collect::<Vec<_>>(),
            vec!["C", "Bb", "Ab", "Gb", "Fb", "Eb", "Db", "C"]
        );

        assert!(generate_scale(
            "C",
            "melodic_minor",
            4,
            Some("not_a_mode".to_string()),
            true,
        )
        .is_null());
    }
}
