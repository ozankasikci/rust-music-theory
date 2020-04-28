use crate::scale::errors::ScaleError;
use crate::scale::errors::ScaleError::ModeFromRegex;
use crate::scale::mode::Mode::*;
use regex::{Match, Regex};
use strum_macros::{Display, EnumIter};

const REGEX_MAJOR: &str = r"^(M\s+|M$|(?i)maj|major|ionian)";
const REGEX_MINOR: &str = r"^(m\s+|m$|(?i)min|minor|aeolian)";
const REGEX_DORIAN: &str = r"(?i)^(dorian)";
const REGEX_PHRYGIAN: &str = r"(?i)^(phrygian)";
const REGEX_LYDIAN: &str = r"(?i)^(lydian)";
const REGEX_MIXOLYDIAN: &str = r"(?i)^(mixolydian)";
const REGEX_LOCRIAN: &str = r"(?i)^(locrian)";
const REGEX_MELODIC_MINOR: &str = r"(?i)(mel minor|melodicminor|melodic\s+minor)";
const REGEX_HARMONIC_MINOR: &str = r"(?i)(har minor|harmonicminor|harmonic\s+minor)";

#[derive(Display, Debug, Clone, Copy, EnumIter, PartialEq)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    HarmonicMinor,
    MelodicMinor,
}

impl Mode {
    pub fn from_regex(string: &str) -> Result<(Self, Match), ScaleError> {
        let regexes = vec![
            (Regex::new(REGEX_MAJOR), Ionian),
            (Regex::new(REGEX_HARMONIC_MINOR), Mode::HarmonicMinor),
            (Regex::new(REGEX_MELODIC_MINOR), Mode::MelodicMinor),
            (Regex::new(REGEX_MINOR), Aeolian),
            (Regex::new(REGEX_DORIAN), Dorian),
            (Regex::new(REGEX_LOCRIAN), Locrian),
            (Regex::new(REGEX_MIXOLYDIAN), Mixolydian),
            (Regex::new(REGEX_PHRYGIAN), Phrygian),
            (Regex::new(REGEX_LYDIAN), Lydian),
        ];

        for (regex, mode_enum) in regexes {
            let mode = regex?.find(string.trim());

            match mode {
                Some(mode_match) => return Ok((mode_enum, mode_match)),
                _ => {}
            };
        }

        Err(ModeFromRegex)
    }
}
