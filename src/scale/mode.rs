use strum_macros::{EnumIter, Display};
use regex::{Regex, Match};
use std::error;
use crate::scale::errors::ScaleError::ModeFromRegexError;
use crate::scale::errors::ScaleError;

const REGEX_MAJOR: &str = "(M|maj|Maj|Major|major)";
const REGEX_MINOR: &str = "(m|min|Min|Minor|minor)";
const REGEX_DORIAN: &str = "(dor|dorian)";
const REGEX_PHRYGIAN: &str = "(phy|phr|phrygian)";
const REGEX_LYDIAN: &str = "(lyd|lydian)";
const REGEX_MIXOLYDIAN: &str = "(mix|mixolydian)";
const REGEX_LOCRIAN: &str = "(loc|locrian)";

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
            "M" | "maj" | "Major" | "major" | "ion" | "Ionian" | "ionian" => Ionian,
            "m" | "min" | "Minor" |"minor" | "aeo" | "Aeolian" | "aeolian" => Aeolian,
            "Dorian" | "dor" | "dorian" => Dorian,
            "Phrygian" | "phr" | "phy" | "phrygian" => Phrygian,
            "Lydian" | "lyd" | "lydian" => Lydian,
            "Mixolydian" | "mix" | "mixolydian" => Mixolydian,
            "Locrian" | "loc" | "locrian" => Locrian,
            _ => {
                Ionian
            },
        }
    }

    pub fn from_regex(string: &str) -> Result<(Self, Match), ScaleError> {
        let regexes = vec![
            Regex::new(REGEX_MAJOR),
            Regex::new(REGEX_MINOR),
            Regex::new(REGEX_DORIAN),
            Regex::new(REGEX_LOCRIAN),
            Regex::new(REGEX_MIXOLYDIAN),
            Regex::new(REGEX_PHRYGIAN),
            Regex::new(REGEX_LYDIAN),
        ];

        let mode: Option<Match>;

        for regex in regexes {
           let mode = regex?.find(string);

            match mode {
                Some(mode) => {
                    return Ok((Self::from_str(&string[mode.start()..mode.end()]), mode));
                },
                _ => {}
            }
        }

        Err(ModeFromRegexError)
    }
}

