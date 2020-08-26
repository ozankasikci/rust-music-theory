use crate::scale::errors::ScaleError;
use crate::scale::errors::ScaleError::ModeFromRegex;
use crate::scale::mode::Mode::*;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use strum_macros::{Display, EnumIter};

lazy_static! {
    static ref MODE_REGEXES: Vec<(Regex, Mode)> = vec![
        (
            Regex::new(r"^(M\s+|M$|(?i)maj|major|ionian)").unwrap(),
            Ionian
        ),
        (
            Regex::new(r"(?i)(har minor|harmonicminor|harmonic\s+minor)").unwrap(),
            HarmonicMinor
        ),
        (
            Regex::new(r"(?i)(mel minor|melodicminor|melodic\s+minor)").unwrap(),
            MelodicMinor
        ),
        (
            Regex::new(r"^(m\s+|m$|(?i)min|minor|aeolian)").unwrap(),
            Aeolian
        ),
        (Regex::new(r"(?i)^(dorian)").unwrap(), Dorian),
        (Regex::new(r"(?i)^(locrian)").unwrap(), Locrian),
        (Regex::new(r"(?i)^(mixolydian)").unwrap(), Mixolydian),
        (Regex::new(r"(?i)^(phrygian)").unwrap(), Phrygian),
        (Regex::new(r"(?i)^(lydian)").unwrap(), Lydian),
    ];
}

/// The mode of a scale.
#[derive(Display, Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum Mode {
    /// Also known as a major scale.
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    /// Also known as a natural minor scale.
    Aeolian,
    Locrian,
    HarmonicMinor,
    MelodicMinor,
}

impl Mode {
    /// Parse a mode using a regex.
    pub fn from_regex(string: &str) -> Result<(Self, Match), ScaleError> {
        for (regex, mode_enum) in &*MODE_REGEXES {
            let mode = regex.find(string.trim());

            if let Some(mode_match) = mode {
                return Ok((*mode_enum, mode_match));
            };
        }

        Err(ModeFromRegex)
    }

    /// Get whether the mode is diatonic (not harmonic or melodic minor).
    pub fn is_diatonic(self) -> bool {
        !matches!(self, Self::HarmonicMinor | Self::MelodicMinor)
    }
}
