use crate::note::Note;
use serde::{Deserialize, Serialize};

/// WASM-compatible note representation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmNote {
    pub pitch: String,
    pub octave: i16,
    pub display: String,
}

/// WASM-compatible scale representation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmScale {
    pub notes: Vec<WasmNote>,
    pub scale_type: String,
    pub tonic: String,
    pub mode: Option<String>,
    pub direction: String,
}

/// WASM-compatible chord representation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmChord {
    pub notes: Vec<WasmNote>,
    pub root: String,
    pub quality: String,
    pub number: String,
    pub canonical_symbol: String,
    pub bass: Option<String>,
    pub modifiers: Vec<String>,
    pub formula: Vec<String>,
}

impl From<Note> for WasmNote {
    fn from(note: Note) -> Self {
        WasmNote {
            pitch: format!("{}", note.pitch),
            octave: note.octave,
            display: format!("{}", note),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::{NoteLetter, Pitch};

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_wasm_note_from_note() {
        let note = Note::new(Pitch::new(NoteLetter::C, 1), 4); // C# in octave 4
        let wasm_note = WasmNote::from(note);

        assert_eq!(wasm_note.pitch, "C#");
        assert_eq!(wasm_note.octave, 4);
        assert_eq!(wasm_note.display, "C#");
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_wasm_note_with_flats() {
        let note = Note::new(Pitch::new(NoteLetter::B, -1), 3); // Bb in octave 3
        let wasm_note = WasmNote::from(note);

        assert_eq!(wasm_note.pitch, "Bb");
        assert_eq!(wasm_note.octave, 3);
        assert_eq!(wasm_note.display, "Bb");
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_wasm_note_serialization() {
        let note = Note::new(Pitch::new(NoteLetter::F, 1), 5); // F# in octave 5
        let wasm_note = WasmNote::from(note);

        // Test that the WasmNote has the expected fields for serialization
        assert_eq!(wasm_note.pitch, "F#");
        assert_eq!(wasm_note.octave, 5);
        assert_eq!(wasm_note.display, "F#");
    }
}
