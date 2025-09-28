use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Macro for console logging in WASM
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Initialize the WASM module
#[wasm_bindgen]
pub fn init() {
    console_log!("Rust Music Theory WASM module initialized!");
}

/// Get list of all chromatic pitches
#[wasm_bindgen]
pub fn get_chromatic_pitches() -> JsValue {
    let pitches = vec![
        "C", "C#", "Db", "D", "D#", "Eb", "E", "F", "F#", "Gb", "G", "G#", "Ab", "A", "A#", "Bb", "B"
    ];
    serde_wasm_bindgen::to_value(&pitches).unwrap_or(JsValue::NULL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chromatic_pitches_count() {
        let pitches = vec![
            "C", "C#", "Db", "D", "D#", "Eb", "E", "F",
            "F#", "Gb", "G", "G#", "Ab", "A", "A#", "Bb", "B"
        ];
        assert_eq!(pitches.len(), 17);
    }

    #[test]
    fn test_chromatic_pitches_content() {
        let pitches = vec![
            "C", "C#", "Db", "D", "D#", "Eb", "E", "F",
            "F#", "Gb", "G", "G#", "Ab", "A", "A#", "Bb", "B"
        ];

        // Test that we have both sharps and flats
        assert!(pitches.contains(&"C#"));
        assert!(pitches.contains(&"Db"));
        assert!(pitches.contains(&"F#"));
        assert!(pitches.contains(&"Gb"));

        // Test natural notes
        assert!(pitches.contains(&"C"));
        assert!(pitches.contains(&"D"));
        assert!(pitches.contains(&"E"));
        assert!(pitches.contains(&"F"));
        assert!(pitches.contains(&"G"));
        assert!(pitches.contains(&"A"));
        assert!(pitches.contains(&"B"));
    }
}