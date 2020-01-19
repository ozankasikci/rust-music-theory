use strum_macros::{EnumIter, Display};

#[derive(Debug, EnumIter, Display)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    pub fn from_u8(val: u8) -> Self {
        use Mode::*;
        match val {
            1 => Ionian,
            2 => Dorian,
            3 => Phrygian,
            4 => Lydian,
            5 => Mixolydian,
            6 => Aeolian,
            7 => Locrian,
            _ => Ionian
        }
    }
}

