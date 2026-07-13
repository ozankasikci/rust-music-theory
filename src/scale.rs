//! Scales and modes.
//!
//! The seven modes of the major, harmonic-minor, and melodic-minor families preserve generic
//! letter spelling, including theoretical double accidentals.
//!
//! ```
//! use rust_music_theory::note::{NoteLetter, Notes, Pitch};
//! use rust_music_theory::scale::{Direction, Mode, Scale, ScaleType};
//!
//! let scale = Scale::new(
//!     ScaleType::MelodicMinor,
//!     Pitch::new(NoteLetter::C, 0),
//!     4,
//!     Some(Mode::Altered),
//!     Direction::Ascending,
//! ).unwrap();
//! let pitches = scale
//!     .notes()
//!     .iter()
//!     .map(|note| note.pitch.to_string())
//!     .collect::<Vec<_>>();
//! assert_eq!(pitches, ["C", "Db", "Eb", "Fb", "Gb", "Ab", "Bb", "C"]);
//! ```

mod errors;
mod mode;
mod scale;
mod scale_type;

pub use errors::ScaleError;
pub use mode::Mode;
pub use scale::{Direction, Scale};
pub use scale_type::ScaleType;
