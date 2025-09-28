//! WASM module for exposing rust-music-theory functionality to JavaScript/WebAssembly

pub mod types;
pub mod parsers;
pub mod scales;
pub mod chords;
pub mod utils;

// Re-export the main WASM functions for easier access
pub use scales::{generate_scale, get_available_scales, get_available_modes};
pub use chords::{generate_chord, get_available_chord_qualities, get_available_chord_numbers};
pub use utils::{init, get_chromatic_pitches};

// Re-export types for external use
pub use types::{WasmNote, WasmScale, WasmChord};