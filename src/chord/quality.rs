use crate::chord::errors::ChordError;
use regex::{Match, Regex};
use strum_macros::Display;

const REGEX_QUALITY_MAJOR: &str = r"^(M\s+|M$|(?i)maj|Maj|Major|major)";
const REGEX_QUALITY_MINOR: &str = r"^(m\s+|m$|(?i)min|Min|Minor|minor)";
const REGEX_QUALITY_DIMINISHED: &str = r"(?i)^(diminished)";
const REGEX_QUALITY_AUGMENTED: &str = r"(?i)^(augmented)";
const REGEX_QUALITY_HALF_DIMINISHED: &str = r"(?i)^(half\s*diminished|halfdiminished)";
const REGEX_QUALITY_DOMINANT: &str = r"(?i)^(dominant)";
const REGEX_QUALITY_SUSPENDED_4: &str = r"(?i)^(sus4\s+|sus4$|suspended4)";
const REGEX_QUALITY_SUSPENDED_2: &str = r"(?i)^(sus2\s+|sus2$|suspended2)";

/// The quality of a chord.
#[derive(Display, Debug, Clone, Copy, PartialEq)]
pub enum Quality {
    Major,
    Minor,
    Diminished,
    Augmented,
    HalfDiminished,
    Dominant,
    Suspended2,
    Suspended4,
}

impl Quality {
    /// Parse a quality from a regex.
    pub fn from_regex(string: &str) -> Result<(Self, Option<Match>), ChordError> {
        use Quality::*;
        let regexes = vec![
            (Regex::new(REGEX_QUALITY_MAJOR), Major),
            (Regex::new(REGEX_QUALITY_MINOR), Minor),
            (Regex::new(REGEX_QUALITY_DIMINISHED), Diminished),
            (Regex::new(REGEX_QUALITY_AUGMENTED), Augmented),
            (Regex::new(REGEX_QUALITY_HALF_DIMINISHED), HalfDiminished),
            (Regex::new(REGEX_QUALITY_DOMINANT), Dominant),
            (Regex::new(REGEX_QUALITY_SUSPENDED_2), Suspended2),
            (Regex::new(REGEX_QUALITY_SUSPENDED_4), Suspended4),
        ];

        for (regex, quality_enum) in regexes {
            let mode = regex?.find(string.trim());

            if let Some(quality_match) = mode {
                return Ok((quality_enum, Some(quality_match)));
            };
        }

        Ok((Major, None))
    }
}
