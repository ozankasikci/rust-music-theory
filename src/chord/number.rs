use crate::chord::errors::ChordError;
use regex::{Match, Regex};

const REGEX_NUMBER_TRIAD: &str = "(triad|Triad)";
const REGEX_NUMBER_SEVENTH: &str = "(seventh|Seventh)";
const REGEX_NUMBER_NINTH: &str = "(ninth|Ninth)";
const REGEX_NUMBER_ELEVENTH: &str = "(eleventh|Eleventh)";
const REGEX_NUMBER_THIRTEENTH: &str = "(thirteenth|Thirteenth)";

#[derive(Debug, PartialEq)]
pub enum Number {
    Triad,
    Seventh,
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
            (Regex::new(REGEX_NUMBER_NINTH), Ninth),
            (Regex::new(REGEX_NUMBER_ELEVENTH), Eleventh),
            (Regex::new(REGEX_NUMBER_THIRTEENTH), Thirteenth),
        ];

        let number: Option<Match>;

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
