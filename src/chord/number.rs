use regex::{Match, Regex};
use crate::chord::errors::ChordError;

const REGEX_NUMBER_SEVENTH: &str = "(seventh|Seventh)";
const REGEX_NUMBER_TRIAD: &str = "(triad|Triad)";

#[derive(Debug, PartialEq)]
pub enum Number {
    Triad,
    Seventh,
}

impl Number {
    pub fn from_regex(string: &str) -> Result<(Self, Match), ChordError> {
        use Number::*;
        let regexes = vec![
            (Regex::new(REGEX_NUMBER_TRIAD), Triad),
            (Regex::new(REGEX_NUMBER_SEVENTH), Seventh),
        ];

        let number: Option<Match>;

        for (regex, number_enum) in regexes {
            let mode = regex?.find(string);

            match mode {
                Some(number_match) => return Ok((number_enum, number_match)),
                _ => {}
            };
        }

        Err(ChordError::InvalidRegex)
    }
}