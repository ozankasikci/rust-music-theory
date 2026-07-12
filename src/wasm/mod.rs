//! WASM module for exposing rust-music-theory functionality to JavaScript/WebAssembly

pub mod chords;
pub mod parsers;
pub mod scales;
pub mod types;
pub mod utils;

// Re-export the main WASM functions for easier access
pub use chords::{
    generate_chord, get_available_chord_numbers, get_available_chord_qualities, parse_chord_symbol,
};
pub use scales::{generate_scale, get_available_modes, get_available_scales};
pub use utils::{get_chromatic_pitches, init};

// Re-export types for external use
pub use types::{WasmChord, WasmNote, WasmScale};
