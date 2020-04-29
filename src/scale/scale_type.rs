use crate::scale::{Mode, Mode::*};
use strum_macros::{Display, EnumIter};

#[derive(Display, Debug, Clone, Copy, EnumIter, PartialEq)]
pub enum ScaleType {
    Diatonic,
    MelodicMinor,
    HarmonicMinor,
}

impl ScaleType {
    pub fn from_mode(mode: Mode) -> Self {
        use ScaleType::*;
        match mode {
            Ionian => Diatonic,
            Aeolian => Diatonic,
            Mode::HarmonicMinor => ScaleType::HarmonicMinor,
            Mode::MelodicMinor => ScaleType::MelodicMinor,
            _ => Diatonic,
        }
    }
}
