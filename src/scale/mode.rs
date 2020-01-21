use strum_macros::{EnumIter, Display};
use regex::{Regex, Match};
use std::error;

const REGEX_MINOR: &str = "(minor|min|m)";
const REGEX_MAJOR: &str = "(M|maj|major)";

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

    pub fn from_str(str: &str) -> Self {
        use Mode::*;
        match str {
            "m" | "min" | "Minor" |"minor" => Aeolian,
            "M" | "maj" | "Major" | "major" => Ionian,
            _ => {
                Ionian
            },
        }
    }

    pub fn from_regex(string: &str) -> Result<(Self, Match), Box<dyn error::Error>> {
        let r_major = Regex::new(REGEX_MAJOR)?;
        let r_minor = Regex::new(REGEX_MINOR)?;

        let pitch = r_major.find(&string)
            .or_else(|| r_minor.find(&string))
            .ok_or("no item")?;

        Ok((Self::from_str(&string[pitch.start()..pitch.end()]), pitch))

    }
}

