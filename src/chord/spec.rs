use crate::chord::ChordError;

/// The quality of the chord's base triad before extensions and modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriadQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Power,
}

/// The quality of the seventh used by seventh and extended chords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeventhQuality {
    Major,
    Minor,
    Diminished,
}

/// The highest structural extension of a chord.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordExtension {
    Triad,
    Sixth,
    SixNine,
    Seventh,
    Ninth,
    Eleventh,
    Thirteenth,
}

/// A suspension replacing the chord's third.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suspension {
    Second,
    Fourth,
}

/// A written chord tone represented by its compound degree and alteration.
///
/// `degree = 9, alteration = -1` represents a flat ninth. Alterations are
/// measured in semitones relative to the major/perfect form of the degree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChordTone {
    degree: u8,
    alteration: i8,
}

impl ChordTone {
    pub fn new(degree: u8, alteration: i8) -> Result<Self, ChordError> {
        if !matches!(degree, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 9 | 11 | 13) {
            return Err(ChordError::InvalidDegree(degree));
        }
        if !(-2..=2).contains(&alteration) {
            return Err(ChordError::InvalidAlteration(alteration));
        }
        Ok(Self { degree, alteration })
    }

    pub(crate) const fn new_unchecked(degree: u8, alteration: i8) -> Self {
        Self { degree, alteration }
    }

    pub fn degree(self) -> u8 {
        self.degree
    }

    pub fn alteration(self) -> i8 {
        self.alteration
    }

    pub fn semitones(self) -> i16 {
        let natural = match self.degree {
            1 => 0,
            2 => 2,
            3 => 4,
            4 => 5,
            5 => 7,
            6 => 9,
            7 => 11,
            9 => 14,
            11 => 17,
            13 => 21,
            _ => unreachable!("validated chord degree"),
        };
        natural + self.alteration as i16
    }

    pub fn letter_offset(self) -> i16 {
        (self.degree as i16 - 1) % 7
    }

    pub(crate) fn accidental_prefix(self) -> String {
        let accidental = if self.alteration < 0 { 'b' } else { '#' };
        (0..self.alteration.unsigned_abs())
            .map(|_| accidental)
            .collect()
    }
}

/// A modification applied to a base chord formula.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChordModifier {
    Add(ChordTone),
    Alter(ChordTone),
    Omit(u8),
    Altered,
}

/// A normalized, validated chord description independent of root and voicing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChordSpec {
    triad_quality: TriadQuality,
    seventh_quality: Option<SeventhQuality>,
    extension: ChordExtension,
    suspension: Option<Suspension>,
    modifiers: Vec<ChordModifier>,
}

impl ChordSpec {
    pub fn new(
        triad_quality: TriadQuality,
        seventh_quality: Option<SeventhQuality>,
        extension: ChordExtension,
    ) -> Result<Self, ChordError> {
        let spec = Self {
            triad_quality,
            seventh_quality,
            extension,
            suspension: None,
            modifiers: Vec::new(),
        };
        spec.validate()?;
        Ok(spec)
    }

    pub(crate) fn from_parts(
        triad_quality: TriadQuality,
        seventh_quality: Option<SeventhQuality>,
        extension: ChordExtension,
        suspension: Option<Suspension>,
        modifiers: Vec<ChordModifier>,
    ) -> Result<Self, ChordError> {
        let mut spec = Self {
            triad_quality,
            seventh_quality,
            extension,
            suspension,
            modifiers,
        };
        spec.normalize_equivalent_constructions();
        spec.normalize_modifiers();
        spec.validate()?;
        spec.formula()?;
        Ok(spec)
    }

    pub fn triad_quality(&self) -> TriadQuality {
        self.triad_quality
    }

    pub fn seventh_quality(&self) -> Option<SeventhQuality> {
        self.seventh_quality
    }

    pub fn extension(&self) -> ChordExtension {
        self.extension
    }

    pub fn suspension(&self) -> Option<Suspension> {
        self.suspension
    }

    pub fn modifiers(&self) -> &[ChordModifier] {
        &self.modifiers
    }

    fn normalize_modifiers(&mut self) {
        self.modifiers.sort_by_key(modifier_sort_key);
    }

    fn normalize_equivalent_constructions(&mut self) {
        if self.extension == ChordExtension::Sixth {
            if let Some(index) = self.modifiers.iter().position(|modifier| {
                matches!(modifier, ChordModifier::Add(tone) if tone.degree == 9 && tone.alteration == 0)
            }) {
                self.extension = ChordExtension::SixNine;
                self.modifiers.remove(index);
            }
        }

        if self.triad_quality == TriadQuality::Minor
            && self.extension == ChordExtension::Seventh
            && self.seventh_quality == Some(SeventhQuality::Minor)
        {
            if let Some(index) = self.modifiers.iter().position(|modifier| {
                matches!(modifier, ChordModifier::Alter(tone) if tone.degree == 5 && tone.alteration == -1)
            }) {
                self.triad_quality = TriadQuality::Diminished;
                self.modifiers.remove(index);
            }
        }
    }

    fn validate(&self) -> Result<(), ChordError> {
        use ChordExtension::*;

        let needs_seventh = matches!(self.extension, Seventh | Ninth | Eleventh | Thirteenth);
        if needs_seventh != self.seventh_quality.is_some() {
            return Err(ChordError::InvalidSpecification(
                "seventh and extended chords require exactly one seventh quality".to_string(),
            ));
        }

        if self.triad_quality == TriadQuality::Power
            && (self.extension != Triad
                || self.seventh_quality.is_some()
                || self.suspension.is_some()
                || !self.modifiers.is_empty())
        {
            return Err(ChordError::InvalidSpecification(
                "power chords cannot have extensions or modifiers".to_string(),
            ));
        }

        if matches!(self.extension, Sixth | SixNine)
            && !matches!(
                self.triad_quality,
                TriadQuality::Major | TriadQuality::Minor
            )
        {
            return Err(ChordError::InvalidSpecification(
                "sixth chords require a major or minor triad".to_string(),
            ));
        }

        if self.triad_quality == TriadQuality::Diminished
            && !matches!(self.extension, Triad | Seventh)
        {
            return Err(ChordError::InvalidSpecification(
                "diminished chords support triad and seventh forms".to_string(),
            ));
        }

        if needs_seventh {
            let seventh = self.seventh_quality.unwrap();
            let compatible = match self.triad_quality {
                TriadQuality::Diminished => {
                    matches!(seventh, SeventhQuality::Minor | SeventhQuality::Diminished)
                }
                TriadQuality::Major | TriadQuality::Minor | TriadQuality::Augmented => {
                    matches!(seventh, SeventhQuality::Major | SeventhQuality::Minor)
                }
                TriadQuality::Power => false,
            };
            if !compatible {
                return Err(ChordError::InvalidSpecification(
                    "seventh quality is incompatible with the base triad".to_string(),
                ));
            }
        }

        if self.triad_quality == TriadQuality::Augmented
            && !matches!(self.extension, Triad | Seventh)
        {
            return Err(ChordError::InvalidSpecification(
                "augmented chords support triad and seventh forms".to_string(),
            ));
        }

        if self.suspension.is_some()
            && !matches!(
                self.triad_quality,
                TriadQuality::Major | TriadQuality::Minor
            )
        {
            return Err(ChordError::InvalidSpecification(
                "suspensions require a major or minor base".to_string(),
            ));
        }

        let altered_count = self
            .modifiers
            .iter()
            .filter(|modifier| matches!(modifier, ChordModifier::Altered))
            .count();
        if altered_count > 0 {
            if altered_count != 1
                || self.modifiers.len() != 1
                || self.triad_quality != TriadQuality::Major
                || self.extension != Seventh
                || self.seventh_quality != Some(SeventhQuality::Minor)
                || self.suspension.is_some()
            {
                return Err(ChordError::ConflictingModifiers(
                    "alt is only valid by itself on a dominant seventh chord".to_string(),
                ));
            }
            return Ok(());
        }

        if self.suspension.is_some()
            && self
                .modifiers
                .iter()
                .any(|modifier| matches!(modifier, ChordModifier::Omit(3)))
        {
            return Err(ChordError::ConflictingModifiers(
                "a suspended chord already omits the third".to_string(),
            ));
        }

        for (index, modifier) in self.modifiers.iter().enumerate() {
            validate_modifier(modifier)?;
            for other in self.modifiers.iter().skip(index + 1) {
                if modifier == other {
                    return Err(ChordError::ConflictingModifiers(format!(
                        "duplicate modifier {}",
                        format_modifier(modifier)
                    )));
                }
                if modifier_degree(modifier).is_some()
                    && modifier_degree(modifier) == modifier_degree(other)
                {
                    return Err(ChordError::ConflictingModifiers(format!(
                        "multiple operations target degree {}",
                        modifier_degree(modifier).unwrap()
                    )));
                }
            }
        }

        Ok(())
    }

    pub fn formula(&self) -> Result<ChordFormula, ChordError> {
        use ChordExtension::*;

        if self.modifiers == [ChordModifier::Altered] {
            return Ok(ChordFormula::new(vec![
                tone(1, 0),
                tone(3, 0),
                tone(5, -1),
                tone(5, 1),
                tone(7, -1),
                tone(9, -1),
                tone(9, 1),
            ]));
        }

        let mut tones = match self.triad_quality {
            TriadQuality::Major => vec![tone(1, 0), tone(3, 0), tone(5, 0)],
            TriadQuality::Minor => vec![tone(1, 0), tone(3, -1), tone(5, 0)],
            TriadQuality::Diminished => vec![tone(1, 0), tone(3, -1), tone(5, -1)],
            TriadQuality::Augmented => vec![tone(1, 0), tone(3, 0), tone(5, 1)],
            TriadQuality::Power => vec![tone(1, 0), tone(5, 0)],
        };

        match self.extension {
            Triad => {}
            Sixth => tones.push(tone(6, 0)),
            SixNine => {
                tones.push(tone(6, 0));
                tones.push(tone(9, 0));
            }
            Seventh | Ninth | Eleventh | Thirteenth => {
                tones.push(match self.seventh_quality.unwrap() {
                    SeventhQuality::Major => tone(7, 0),
                    SeventhQuality::Minor => tone(7, -1),
                    SeventhQuality::Diminished => tone(7, -2),
                });
                if matches!(self.extension, Ninth | Eleventh | Thirteenth) {
                    tones.push(tone(9, 0));
                }
                if matches!(self.extension, Eleventh | Thirteenth) {
                    tones.push(tone(11, 0));
                }
                if self.extension == Thirteenth {
                    tones.push(tone(13, 0));
                }
            }
        }

        if let Some(suspension) = self.suspension {
            tones.retain(|tone| tone.degree != 3);
            tones.push(match suspension {
                Suspension::Second => tone(2, 0),
                Suspension::Fourth => tone(4, 0),
            });
        }

        for modifier in &self.modifiers {
            match modifier {
                ChordModifier::Alter(altered) => {
                    if tones.iter().any(|tone| {
                        tone.degree == altered.degree && tone.alteration == altered.alteration
                    }) {
                        return Err(ChordError::ConflictingModifiers(format!(
                            "degree {} already has the requested alteration",
                            altered.degree
                        )));
                    }
                    tones.retain(|tone| tone.degree != altered.degree);
                    tones.push(*altered);
                }
                ChordModifier::Add(added) => {
                    if tones.iter().any(|tone| tone.degree == added.degree) {
                        return Err(ChordError::ConflictingModifiers(format!(
                            "degree {} is already present",
                            added.degree
                        )));
                    }
                    tones.push(*added);
                }
                ChordModifier::Omit(degree) => {
                    let previous_len = tones.len();
                    tones.retain(|tone| tone.degree != *degree);
                    if tones.len() == previous_len {
                        return Err(ChordError::ConflictingModifiers(format!(
                            "degree {} is not present",
                            degree
                        )));
                    }
                }
                ChordModifier::Altered => unreachable!("handled above"),
            }
        }

        if tones.len() < 2 {
            return Err(ChordError::InvalidSpecification(
                "a chord must contain at least two tones".to_string(),
            ));
        }

        Ok(ChordFormula::new(tones))
    }
}

/// The resolved tones of a chord, sorted in written degree order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChordFormula {
    tones: Vec<ChordTone>,
}

impl ChordFormula {
    fn new(mut tones: Vec<ChordTone>) -> Self {
        tones.sort_by_key(|tone| (tone.degree, tone.alteration));
        Self { tones }
    }

    pub fn tones(&self) -> &[ChordTone] {
        &self.tones
    }
}

pub(crate) fn format_modifier(modifier: &ChordModifier) -> String {
    match modifier {
        ChordModifier::Add(tone) => {
            format!("add{}{}", tone.accidental_prefix(), tone.degree)
        }
        ChordModifier::Alter(tone) => {
            format!("{}{}", tone.accidental_prefix(), tone.degree)
        }
        ChordModifier::Omit(degree) => format!("no{}", degree),
        ChordModifier::Altered => "alt".to_string(),
    }
}

fn tone(degree: u8, alteration: i8) -> ChordTone {
    ChordTone::new_unchecked(degree, alteration)
}

fn validate_modifier(modifier: &ChordModifier) -> Result<(), ChordError> {
    match modifier {
        ChordModifier::Add(tone) => {
            if !matches!(tone.degree, 2 | 3 | 4 | 6 | 9 | 11 | 13) {
                return Err(ChordError::InvalidModifier(format_modifier(modifier)));
            }
        }
        ChordModifier::Alter(tone) => {
            if tone.alteration == 0 || !matches!(tone.degree, 5 | 9 | 11 | 13) {
                return Err(ChordError::InvalidModifier(format_modifier(modifier)));
            }
        }
        ChordModifier::Omit(degree) => {
            if !matches!(degree, 3 | 5 | 7 | 9 | 11 | 13) {
                return Err(ChordError::InvalidModifier(format_modifier(modifier)));
            }
        }
        ChordModifier::Altered => {}
    }
    Ok(())
}

fn modifier_degree(modifier: &ChordModifier) -> Option<u8> {
    match modifier {
        ChordModifier::Add(tone) | ChordModifier::Alter(tone) => Some(tone.degree),
        ChordModifier::Omit(degree) => Some(*degree),
        ChordModifier::Altered => None,
    }
}

fn modifier_sort_key(modifier: &ChordModifier) -> (u8, u8, i8) {
    match modifier {
        ChordModifier::Alter(tone) => (0, tone.degree, tone.alteration),
        ChordModifier::Add(tone) => (1, tone.degree, tone.alteration),
        ChordModifier::Omit(degree) => (2, *degree, 0),
        ChordModifier::Altered => (3, 0, 0),
    }
}
