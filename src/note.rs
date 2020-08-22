//! Individual notes.

mod errors;
mod note;
mod pitch_class;
mod pitch_symbol;

pub use errors::NoteError;
pub use note::{Note, Notes};
pub use pitch_class::{Pitch, NoteLetter, pitch};
pub use pitch_symbol::PitchSymbol;
