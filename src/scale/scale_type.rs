use crate::scale::Mode;
use strum_macros::{Display, EnumIter};

/// The type of a scale.
#[derive(Display, Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum ScaleType {
    Diatonic,
    MelodicMinor,
    HarmonicMinor,
    PentatonicMajor,
    PentatonicMinor,
    Blues,
    Chromatic,
    WholeTone,
}

impl ScaleType {
    /// Get the scale type from the mode.
    pub fn from_mode(mode: Mode) -> Self {
        mode.scale_type()
    }
}

impl From<Mode> for ScaleType {
    fn from(mode: Mode) -> Self {
        Self::from_mode(mode)
    }
}
