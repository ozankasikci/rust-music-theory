use crate::chord::errors::ChordError;
use regex::{Match, Regex};

const REGEX_NUMBER_TRIAD: &str = "(?i)(triad)";
const REGEX_NUMBER_SEVENTH: &str = "(?i)(seventh)";
const REGEX_NUMBER_MAJOR_SEVENTH: &str = r"(?i)(major\s*seventh)";
const REGEX_NUMBER_NINTH: &str = "(?i)(ninth)";
const REGEX_NUMBER_ELEVENTH: &str = "(?i)(eleventh)";
const REGEX_NUMBER_THIRTEENTH: &str = "(?i)(thirteenth)";

#[derive(Debug, PartialEq)]
pub enum Number {
    Triad,
    Seventh,
    MajorSeventh,
    Ninth,
    Eleventh,
    Thirteenth,
}

impl Number {
    pub fn from_regex(string: &str) -> Result<(Self, Option<Match>), ChordError> {
        use Number::*;
        let regexes = vec![
            (Regex::new(REGEX_NUMBER_TRIAD), Triad),
            (Regex::new(REGEX_NUMBER_SEVENTH), Seventh),
            (Regex::new(REGEX_NUMBER_MAJOR_SEVENTH), MajorSeventh),
            (Regex::new(REGEX_NUMBER_NINTH), Ninth),
            (Regex::new(REGEX_NUMBER_ELEVENTH), Eleventh),
            (Regex::new(REGEX_NUMBER_THIRTEENTH), Thirteenth),
        ];

        for (regex, number_enum) in regexes {
            let mode = regex?.find(string);

            match mode {
                Some(number_match) => return Ok((number_enum, Some(number_match))),
                _ => {}
            };
        }

        Err(ChordError::InvalidRegex)
    }
}
