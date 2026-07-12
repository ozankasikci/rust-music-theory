use crate::chord::errors::ChordError;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use strum_macros::Display;

lazy_static! {
    static ref NUMBER_REGEXES: Vec<(Regex, Number)> = {
        use Number::*;
        vec![
            (
                Regex::new(r"(?i)^(major\s*seventh|maj7|major7)$").unwrap(),
                MajorSeventh,
            ),
            (Regex::new("(?i)^(triad)$").unwrap(), Triad),
            (Regex::new("(?i)^(seventh|7)$").unwrap(), Seventh),
            (Regex::new("(?i)^(ninth|9)$").unwrap(), Ninth),
            (Regex::new("(?i)^(eleventh|11)$").unwrap(), Eleventh),
            (Regex::new("(?i)^(thirteenth|13)$").unwrap(), Thirteenth),
        ]
    };
}

/// The superscript number after a chord.
#[derive(Display, Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Triad,
    Seventh,
    MajorSeventh,
    Ninth,
    Eleventh,
    Thirteenth,
}

impl Number {
    /// Parse the number using a regex.
    pub fn from_regex(string: &str) -> Result<(Self, Option<Match<'_>>), ChordError> {
        for (regex, number_enum) in &*NUMBER_REGEXES {
            let mode = regex.find(string);

            if let Some(number_match) = mode {
                return Ok((*number_enum, Some(number_match)));
            };
        }

        Err(ChordError::InvalidRegex)
    }
}
