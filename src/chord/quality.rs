use crate::chord::errors::ChordError;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use strum_macros::Display;

lazy_static! {
    static ref QUALITY_REGEXES: Vec<(Regex, Quality)> = {
        use Quality::*;

        vec![
            (
                Regex::new(r"^(m\s+|m$|(?i:min(?:or)?))").unwrap(),
                Minor,
            ),
            (
                Regex::new(r"^(M\s+|M$|(?i:maj(?:or)?))").unwrap(),
                Major,
            ),
            (
                Regex::new(r"(?i)^(half\s*diminished|halfdiminished|halfdim)").unwrap(),
                HalfDiminished,
            ),
            (Regex::new(r"(?i)^dim(?:inished)?").unwrap(), Diminished),
            (Regex::new(r"(?i)^aug(?:mented)?").unwrap(), Augmented),
            (Regex::new(r"(?i)^dom(?:inant)?").unwrap(), Dominant),
            (
                Regex::new(r"(?i)^(sus2\s+|sus2$|suspended2)").unwrap(),
                Suspended2,
            ),
            (
                Regex::new(r"(?i)^(sus4\s+|sus4$|suspended4)").unwrap(),
                Suspended4,
            ),
        ]
    };
}

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
    pub fn from_regex(string: &str) -> Result<(Self, Option<Match<'_>>), ChordError> {
        use Quality::*;

        for (regex, quality_enum) in &*QUALITY_REGEXES {
            let mode = regex.find(string.trim());

            if let Some(quality_match) = mode {
                return Ok((*quality_enum, Some(quality_match)));
            };
        }

        Ok((Major, None))
    }
}
