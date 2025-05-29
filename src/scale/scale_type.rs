use crate::scale::{Mode, Mode::*};
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
        use ScaleType::*;
        match mode {
            Ionian => Diatonic,
            Aeolian => Diatonic,
            Mode::HarmonicMinor => ScaleType::HarmonicMinor,
            Mode::MelodicMinor => ScaleType::MelodicMinor,
            Mode::PentatonicMajor => ScaleType::PentatonicMajor,
            Mode::PentatonicMinor => ScaleType::PentatonicMinor,
            Mode::Blues => ScaleType::Blues,
            Mode::Chromatic => ScaleType::Chromatic,
            Mode::WholeTone => ScaleType::WholeTone,
            _ => Diatonic,
        }
    }
}

impl From<Mode> for ScaleType {
    fn from(mode: Mode) -> Self {
        Self::from_mode(mode)
    }
}
