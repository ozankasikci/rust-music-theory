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

/// The mode of a scale.
#[derive(Display, Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum Mode {
    /// An Ionian/Major scale.
    Ionian,
    /// A Dorian scale.
    Dorian,
    /// A Phygian scale.
    Phrygian,
    /// A lydian scale.
    Lydian,
    /// A mixolydian scale.
    Mixolydian,
    /// An aelian/natural minor scale.
    Aeolian,
    /// A locrian scale.
    Locrian,
    /// A harmonic minor scale.
    HarmonicMinor,
    /// A melodic minor scale.
    MelodicMinor,
}

impl Mode {
    /// Parse a mode using a regex.
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

            if let Some(mode_match) = mode {
                return Ok((mode_enum, mode_match));
            };
        }

        Err(ModeFromRegex)
    }

    /// Get whether the mode is diatonic (not harmonic or melodic minor).
    pub fn is_diatonic(self) -> bool {
        !matches!(self, Self::HarmonicMinor | Self::MelodicMinor)
    }
}
