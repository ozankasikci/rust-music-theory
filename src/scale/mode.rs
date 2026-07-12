use crate::scale::errors::ScaleError;
use crate::scale::errors::ScaleError::ModeFromRegex;
use crate::scale::ScaleType;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::str::FromStr;
use strum_macros::{Display, EnumIter};

lazy_static! {
    static ref MODE_TEXT_REGEX: Regex = Regex::new(r"(?s)\S(?:.*\S)?").unwrap();
}

/// The mode of a scale.
#[derive(Display, Debug, Clone, Copy, EnumIter, PartialEq, Eq, Hash)]
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
    LocrianNatural6,
    IonianSharp5,
    DorianSharp4,
    PhrygianDominant,
    LydianSharp2,
    UltraLocrian,
    MelodicMinor,
    DorianFlat2,
    LydianAugmented,
    LydianDominant,
    MixolydianFlat6,
    LocrianSharp2,
    Altered,
    PentatonicMajor,
    PentatonicMinor,
    Blues,
    Chromatic,
    WholeTone,
}

const ALL_MODES: &[Mode] = &[
    Mode::Ionian,
    Mode::Dorian,
    Mode::Phrygian,
    Mode::Lydian,
    Mode::Mixolydian,
    Mode::Aeolian,
    Mode::Locrian,
    Mode::HarmonicMinor,
    Mode::LocrianNatural6,
    Mode::IonianSharp5,
    Mode::DorianSharp4,
    Mode::PhrygianDominant,
    Mode::LydianSharp2,
    Mode::UltraLocrian,
    Mode::MelodicMinor,
    Mode::DorianFlat2,
    Mode::LydianAugmented,
    Mode::LydianDominant,
    Mode::MixolydianFlat6,
    Mode::LocrianSharp2,
    Mode::Altered,
    Mode::PentatonicMajor,
    Mode::PentatonicMinor,
    Mode::Blues,
    Mode::Chromatic,
    Mode::WholeTone,
];

const HEPTATONIC_MODES: &[Mode] = &[
    Mode::Ionian,
    Mode::Dorian,
    Mode::Phrygian,
    Mode::Lydian,
    Mode::Mixolydian,
    Mode::Aeolian,
    Mode::Locrian,
    Mode::HarmonicMinor,
    Mode::LocrianNatural6,
    Mode::IonianSharp5,
    Mode::DorianSharp4,
    Mode::PhrygianDominant,
    Mode::LydianSharp2,
    Mode::UltraLocrian,
    Mode::MelodicMinor,
    Mode::DorianFlat2,
    Mode::LydianAugmented,
    Mode::LydianDominant,
    Mode::MixolydianFlat6,
    Mode::LocrianSharp2,
    Mode::Altered,
];

fn normalize_mode_name(input: &str) -> String {
    let mut text = String::new();
    for character in input.trim().chars() {
        match character {
            '♭' => text.push('b'),
            '♯' => text.push('#'),
            '♮' => text.push_str(" natural "),
            '-' | '_' => text.push(' '),
            character => text.extend(character.to_lowercase()),
        }
    }

    let raw_tokens = text.split_whitespace().collect::<Vec<_>>();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < raw_tokens.len() {
        let token = match raw_tokens[index] {
            "flat" => "b".to_string(),
            "sharp" => "#".to_string(),
            "nat" | "natural" => "natural".to_string(),
            token
                if token.starts_with("flat")
                    && token[4..].chars().all(|ch| ch.is_ascii_digit()) =>
            {
                format!("b{}", &token[4..])
            }
            token
                if token.starts_with("sharp")
                    && token[5..].chars().all(|ch| ch.is_ascii_digit()) =>
            {
                format!("#{}", &token[5..])
            }
            token
                if token.starts_with("nat") && token[3..].chars().all(|ch| ch.is_ascii_digit()) =>
            {
                format!("natural{}", &token[3..])
            }
            token => token.to_string(),
        };

        if matches!(token.as_str(), "b" | "#" | "natural")
            && raw_tokens
                .get(index + 1)
                .map(|next| next.chars().all(|ch| ch.is_ascii_digit()))
                .unwrap_or(false)
        {
            tokens.push(format!("{}{}", token, raw_tokens[index + 1]));
            index += 2;
        } else {
            tokens.push(token);
            index += 1;
        }
    }

    tokens.join(" ")
}

impl Mode {
    /// Every supported seven-note mode, grouped by parent scale family.
    pub fn heptatonic_modes() -> &'static [Self] {
        HEPTATONIC_MODES
    }

    /// The scale family whose interval pattern produces this mode.
    pub fn scale_type(self) -> ScaleType {
        match self {
            Self::Ionian
            | Self::Dorian
            | Self::Phrygian
            | Self::Lydian
            | Self::Mixolydian
            | Self::Aeolian
            | Self::Locrian => ScaleType::Diatonic,
            Self::HarmonicMinor
            | Self::LocrianNatural6
            | Self::IonianSharp5
            | Self::DorianSharp4
            | Self::PhrygianDominant
            | Self::LydianSharp2
            | Self::UltraLocrian => ScaleType::HarmonicMinor,
            Self::MelodicMinor
            | Self::DorianFlat2
            | Self::LydianAugmented
            | Self::LydianDominant
            | Self::MixolydianFlat6
            | Self::LocrianSharp2
            | Self::Altered => ScaleType::MelodicMinor,
            Self::PentatonicMajor => ScaleType::PentatonicMajor,
            Self::PentatonicMinor => ScaleType::PentatonicMinor,
            Self::Blues => ScaleType::Blues,
            Self::Chromatic => ScaleType::Chromatic,
            Self::WholeTone => ScaleType::WholeTone,
        }
    }

    /// The zero-based rotation of this mode within its scale family.
    pub fn rotation(self) -> usize {
        match self {
            Self::Ionian | Self::HarmonicMinor | Self::MelodicMinor => 0,
            Self::Dorian | Self::LocrianNatural6 | Self::DorianFlat2 => 1,
            Self::Phrygian | Self::IonianSharp5 | Self::LydianAugmented => 2,
            Self::Lydian | Self::DorianSharp4 | Self::LydianDominant => 3,
            Self::Mixolydian | Self::PhrygianDominant | Self::MixolydianFlat6 => 4,
            Self::Aeolian | Self::LydianSharp2 | Self::LocrianSharp2 => 5,
            Self::Locrian | Self::UltraLocrian | Self::Altered => 6,
            Self::PentatonicMajor
            | Self::PentatonicMinor
            | Self::Blues
            | Self::Chromatic
            | Self::WholeTone => 0,
        }
    }

    /// The canonical human-readable ASCII name of this mode.
    pub fn canonical_name(self) -> &'static str {
        match self {
            Self::Ionian => "Ionian",
            Self::Dorian => "Dorian",
            Self::Phrygian => "Phrygian",
            Self::Lydian => "Lydian",
            Self::Mixolydian => "Mixolydian",
            Self::Aeolian => "Aeolian",
            Self::Locrian => "Locrian",
            Self::HarmonicMinor => "Harmonic Minor",
            Self::LocrianNatural6 => "Locrian natural 6",
            Self::IonianSharp5 => "Ionian #5",
            Self::DorianSharp4 => "Dorian #4",
            Self::PhrygianDominant => "Phrygian Dominant",
            Self::LydianSharp2 => "Lydian #2",
            Self::UltraLocrian => "Ultralocrian",
            Self::MelodicMinor => "Melodic Minor",
            Self::DorianFlat2 => "Dorian b2",
            Self::LydianAugmented => "Lydian Augmented",
            Self::LydianDominant => "Lydian Dominant",
            Self::MixolydianFlat6 => "Mixolydian b6",
            Self::LocrianSharp2 => "Locrian #2",
            Self::Altered => "Altered",
            Self::PentatonicMajor => "Pentatonic Major",
            Self::PentatonicMinor => "Pentatonic Minor",
            Self::Blues => "Blues",
            Self::Chromatic => "Chromatic",
            Self::WholeTone => "Whole Tone",
        }
    }

    /// The stable snake-case identifier used by WASM and other serialized APIs.
    pub fn api_name(self) -> &'static str {
        match self {
            Self::Ionian => "ionian",
            Self::Dorian => "dorian",
            Self::Phrygian => "phrygian",
            Self::Lydian => "lydian",
            Self::Mixolydian => "mixolydian",
            Self::Aeolian => "aeolian",
            Self::Locrian => "locrian",
            Self::HarmonicMinor => "harmonic_minor",
            Self::LocrianNatural6 => "locrian_natural_6",
            Self::IonianSharp5 => "ionian_sharp_5",
            Self::DorianSharp4 => "dorian_sharp_4",
            Self::PhrygianDominant => "phrygian_dominant",
            Self::LydianSharp2 => "lydian_sharp_2",
            Self::UltraLocrian => "ultralocrian",
            Self::MelodicMinor => "melodic_minor",
            Self::DorianFlat2 => "dorian_flat_2",
            Self::LydianAugmented => "lydian_augmented",
            Self::LydianDominant => "lydian_dominant",
            Self::MixolydianFlat6 => "mixolydian_flat_6",
            Self::LocrianSharp2 => "locrian_sharp_2",
            Self::Altered => "altered",
            Self::PentatonicMajor => "pentatonic_major",
            Self::PentatonicMinor => "pentatonic_minor",
            Self::Blues => "blues",
            Self::Chromatic => "chromatic",
            Self::WholeTone => "whole_tone",
        }
    }

    fn aliases(self) -> &'static [&'static str] {
        match self {
            Self::Ionian => &["ionian", "major", "maj"],
            Self::Dorian => &["dorian"],
            Self::Phrygian => &["phrygian"],
            Self::Lydian => &["lydian"],
            Self::Mixolydian => &["mixolydian", "dominant"],
            Self::Aeolian => &["aeolian", "minor", "min"],
            Self::Locrian => &["locrian"],
            Self::HarmonicMinor => &["harmonic minor", "harmonicminor", "har minor"],
            Self::LocrianNatural6 => &[
                "locrian natural 6",
                "locrian nat 6",
                "locrian 6",
                "locrian natural sixth",
            ],
            Self::IonianSharp5 => &[
                "ionian #5",
                "ionian augmented",
                "major augmented",
                "major #5",
            ],
            Self::DorianSharp4 => &[
                "dorian #4",
                "ukrainian dorian",
                "romanian minor",
                "altered dorian",
            ],
            Self::PhrygianDominant => &["phrygian dominant", "spanish", "phrygian major"],
            Self::LydianSharp2 => &["lydian #2", "lydian #9"],
            Self::UltraLocrian => &[
                "ultralocrian",
                "ultra locrian",
                "super locrian bb7",
                "superlocrian bb7",
                "super locrian diminished",
                "superlocrian diminished",
            ],
            Self::MelodicMinor => &["melodic minor", "melodicminor", "mel minor", "jazz minor"],
            Self::DorianFlat2 => &["dorian b2", "phrygian #6", "melodic minor second mode"],
            Self::LydianAugmented => &["lydian augmented", "lydian #5"],
            Self::LydianDominant => &["lydian dominant", "lydian b7", "overtone"],
            Self::MixolydianFlat6 => &["mixolydian b6", "melodic minor fifth mode", "hindu"],
            Self::LocrianSharp2 => &["locrian #2", "half diminished", "aeolian b5"],
            Self::Altered => &[
                "altered",
                "super locrian",
                "superlocrian",
                "diminished whole tone",
            ],
            Self::PentatonicMajor => &[
                "pentatonic major",
                "pentatonic maj",
                "pent major",
                "pent maj",
            ],
            Self::PentatonicMinor => &[
                "pentatonic minor",
                "pentatonic min",
                "pent minor",
                "pent min",
            ],
            Self::Blues => &["blues"],
            Self::Chromatic => &["chromatic"],
            Self::WholeTone => &["whole tone", "wholetone"],
        }
    }

    /// Parse a mode while retaining the matched source span for compatibility.
    pub fn from_regex(string: &str) -> Result<(Self, Match<'_>), ScaleError> {
        let mode_match = MODE_TEXT_REGEX.find(string).ok_or(ModeFromRegex)?;
        let mode = Self::from_str(mode_match.as_str())?;
        Ok((mode, mode_match))
    }

    /// Get whether this belongs to the seven modes of the major scale.
    pub fn is_diatonic(self) -> bool {
        self.scale_type() == ScaleType::Diatonic
    }
}

impl FromStr for Mode {
    type Err = ScaleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();
        if trimmed == "M" {
            return Ok(Self::Ionian);
        }
        if trimmed == "m" {
            return Ok(Self::Aeolian);
        }

        let normalized = normalize_mode_name(trimmed);
        let compact = normalized.replace(' ', "");
        ALL_MODES
            .iter()
            .copied()
            .find(|mode| {
                normalize_mode_name(mode.api_name()) == normalized
                    || mode.aliases().iter().any(|alias| {
                        let alias = normalize_mode_name(alias);
                        alias == normalized || alias.replace(' ', "") == compact
                    })
            })
            .ok_or(ModeFromRegex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn every_registered_alias_is_unique_and_round_trips() {
        let mut owners = HashMap::new();
        for mode in ALL_MODES.iter().copied() {
            for name in std::iter::once(mode.canonical_name())
                .chain(std::iter::once(mode.api_name()))
                .chain(mode.aliases().iter().copied())
            {
                assert_eq!(name.parse::<Mode>().unwrap(), mode, "alias {name}");
                let normalized = normalize_mode_name(name);
                if let Some(previous) = owners.insert(normalized.clone(), mode) {
                    assert_eq!(
                        previous, mode,
                        "normalized alias {normalized} belongs to two modes"
                    );
                }
            }
        }
    }

    #[test]
    fn heptatonic_registry_contains_twenty_one_unique_modes() {
        assert_eq!(HEPTATONIC_MODES.len(), 21);
        assert_eq!(
            HEPTATONIC_MODES
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                .len(),
            21
        );
        assert!(HEPTATONIC_MODES.iter().all(|mode| matches!(
            mode.scale_type(),
            ScaleType::Diatonic | ScaleType::HarmonicMinor | ScaleType::MelodicMinor
        )));
    }
}
