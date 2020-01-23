use regex::{Match, Regex};
use crate::chord::errors::ChordError;

const REGEX_QUALITY_MAJOR: &str = "(M|maj|Maj|Major|major)";
const REGEX_QUALITY_MINOR: &str = "(m|min|Min|Minor|minor)";
const REGEX_QUALITY_DIMINISHED: &str = "(dim|diminished|Diminished)";


#[derive(Debug, PartialEq)]
pub enum Quality {
    Major,
    Minor,
    Diminished,
    Augmented,
    AugmentedMajor,
    HalfDiminished,
    MinorMajor,
    Dominant,
}

impl Quality {
    pub fn from_regex(string: &str) -> Result<(Self, Match), ChordError> {
        use Quality::*;
        let regexes = vec![
            (Regex::new(REGEX_QUALITY_MAJOR), Major),
            (Regex::new(REGEX_QUALITY_MINOR), Minor),
        ];

        let quality: Option<Match>;

        for (regex, quality_enum) in regexes {
            let mode = regex?.find(string);

            match mode {
                Some(quality_match) => return Ok((quality_enum, quality_match)),
                _ => {}
            };
        }

        Err(ChordError::InvalidRegex)

    }
}
