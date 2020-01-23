use crate::scale::{Mode, Mode::*};
use strum_macros::{Display, EnumIter};

#[derive(Display, Debug, EnumIter)]
pub enum ScaleType {
    Diatonic,
    MelodicMinor,
    HarmonicMinor,
}

impl ScaleType {
    pub fn from_u8(val: u8) -> Self {
        use ScaleType::*;
        match val {
            1 => Diatonic,
            2 => MelodicMinor,
            3 => HarmonicMinor,
            _ => Diatonic,
        }
    }

    pub fn from_mode(mode: &Mode) -> Self {
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
