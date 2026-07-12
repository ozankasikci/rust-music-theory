use crate::chord::errors::ChordError;
use crate::chord::parser::{parse_chord, ParsedSlash};
use crate::chord::spec::format_modifier;
use crate::chord::{
    ChordExtension, ChordFormula, ChordModifier, ChordSpec, ChordTone, Number, Quality,
    SeventhQuality, Suspension, TriadQuality,
};
use crate::interval::Interval;
use crate::note::{Note, NoteLetter, Notes, Pitch};
use std::fmt;
use std::str::FromStr;

/// A validated lead-sheet chord, including its voicing and optional slash bass.
#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    root: Pitch,
    octave: i16,
    spec: ChordSpec,
    formula: ChordFormula,
    intervals: Vec<Interval>,
    inversion: u8,
    slash_bass: Option<Pitch>,
    legacy_quality: Quality,
    legacy_number: Number,
}

impl Chord {
    /// Create a chord through the legacy quality/number API.
    pub fn new(root: Pitch, quality: Quality, number: Number) -> Self {
        Self::try_new(root, quality, number).expect("unsupported chord quality/number combination")
    }

    /// Try to create a chord through the legacy quality/number API.
    pub fn try_new(root: Pitch, quality: Quality, number: Number) -> Result<Self, ChordError> {
        Self::try_with_inversion(root, quality, number, 0)
    }

    /// Create a chord through the legacy API with a numeric inversion.
    pub fn with_inversion(root: Pitch, quality: Quality, number: Number, inversion: u8) -> Self {
        Self::try_with_inversion(root, quality, number, inversion)
            .expect("unsupported chord or invalid inversion")
    }

    /// Try to create a chord through the legacy API with a numeric inversion.
    pub fn try_with_inversion(
        root: Pitch,
        quality: Quality,
        number: Number,
        inversion: u8,
    ) -> Result<Self, ChordError> {
        let spec = legacy_spec(quality, number)?;
        let mut chord = Self::from_spec(root, spec)?;
        chord.legacy_quality = quality;
        chord.legacy_number = number;
        chord.set_inversion(inversion)?;
        Ok(chord)
    }

    /// Create a chord from a normalized specification.
    pub fn from_spec(root: Pitch, spec: ChordSpec) -> Result<Self, ChordError> {
        let formula = spec.formula()?;
        let intervals = formula_intervals(&formula)?;
        let legacy_quality = legacy_quality(&spec);
        let legacy_number = legacy_number(&spec);
        Ok(Self {
            root,
            octave: 4,
            spec,
            formula,
            intervals,
            inversion: 0,
            slash_bass: None,
            legacy_quality,
            legacy_number,
        })
    }

    /// Parse a compact or long-form lead-sheet chord symbol.
    pub fn parse(symbol: &str) -> Result<Self, ChordError> {
        let parsed = parse_chord(symbol)?;
        let mut chord = Self::from_spec(parsed.root, parsed.spec)?;
        match parsed.slash {
            Some(ParsedSlash::Inversion { value, position }) => chord
                .set_inversion(value)
                .map_err(|_| ChordError::InvalidSlashBass { position })?,
            Some(ParsedSlash::Bass(bass)) => chord.set_bass(bass)?,
            None => {}
        }
        Ok(chord)
    }

    /// Start a programmatic chord specification.
    pub fn builder(root: Pitch) -> ChordBuilder {
        ChordBuilder::new(root)
    }

    /// The written chord root.
    pub fn root(&self) -> Pitch {
        self.root
    }

    /// The octave used for the first voiced note.
    pub fn octave(&self) -> i16 {
        self.octave
    }

    /// The normalized theory specification.
    pub fn spec(&self) -> &ChordSpec {
        &self.spec
    }

    /// The fully resolved and sorted chord tones.
    pub fn formula(&self) -> &ChordFormula {
        &self.formula
    }

    /// Adjacent semitone intervals retained for legacy consumers.
    pub fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    /// Numeric inversion, where zero is root position.
    pub fn inversion(&self) -> u8 {
        self.inversion
    }

    /// The actual slash bass, whether supplied as a note or numeric inversion.
    pub fn bass(&self) -> Option<Pitch> {
        if let Some(bass) = self.slash_bass {
            Some(bass)
        } else if self.inversion > 0 {
            self.root_position_notes()
                .get(self.inversion as usize)
                .map(|note| note.pitch)
        } else {
            None
        }
    }

    /// Legacy quality adapter for this chord's normalized specification.
    pub fn quality(&self) -> Quality {
        self.legacy_quality
    }

    /// Legacy number adapter for this chord's normalized specification.
    pub fn number(&self) -> Number {
        self.legacy_number
    }

    /// Return the canonical, portable ASCII chord symbol.
    pub fn canonical_symbol(&self) -> String {
        self.to_string()
    }

    /// Return a copy voiced in a different octave.
    pub fn with_octave(mut self, octave: i16) -> Self {
        self.octave = octave;
        self
    }

    /// Parse a whitespace-separated set of chord notes and recognize a legacy formula.
    pub fn from_string(string: &str) -> Result<Self, ChordError> {
        let normalized = string.replace(',', "");
        let notes: Vec<Pitch> = normalized
            .split_whitespace()
            .map(|pitch| Pitch::from_str(pitch).ok_or(ChordError::InvalidRegex))
            .collect::<Result<_, _>>()?;

        if notes.is_empty() {
            return Err(ChordError::UnknownIntervalPattern(vec![]));
        }

        let intervals: Vec<u8> = notes
            .windows(2)
            .map(|window| {
                let first = window[0].into_u8();
                let second = window[1].into_u8();
                if first < second {
                    second - first
                } else {
                    second + 12 - first
                }
            })
            .collect();

        Self::from_interval(notes[0], &intervals)
    }

    /// Recognize an adjacent-semitone legacy formula.
    pub fn from_interval(root: Pitch, interval: &[u8]) -> Result<Self, ChordError> {
        use Number::*;
        use Quality::*;
        let (quality, number) = match *interval {
            [7] => (Power, Fifth),
            [4, 3] => (Major, Triad),
            [3, 4] => (Minor, Triad),
            [2, 5] => (Suspended2, Triad),
            [5, 2] => (Suspended4, Triad),
            [4, 4] => (Augmented, Triad),
            [3, 3] => (Diminished, Triad),
            [4, 3, 2] => (Major, Sixth),
            [3, 4, 2] => (Minor, Sixth),
            [4, 3, 2, 5] => (Major, SixNine),
            [3, 4, 2, 5] => (Minor, SixNine),
            [4, 3, 4] => (Major, Seventh),
            [3, 4, 3] => (Minor, Seventh),
            [4, 4, 2] => (Augmented, Seventh),
            [4, 4, 3] => (Augmented, MajorSeventh),
            [3, 3, 3] => (Diminished, Seventh),
            [3, 3, 4] => (HalfDiminished, Seventh),
            [3, 4, 4] => (Minor, MajorSeventh),
            [4, 3, 3] => (Dominant, Seventh),
            [4, 3, 3, 4] => (Dominant, Ninth),
            [4, 3, 4, 3] => (Major, Ninth),
            [3, 4, 3, 4] => (Minor, Ninth),
            [4, 3, 3, 4, 3] => (Dominant, Eleventh),
            [4, 3, 4, 3, 3] => (Major, Eleventh),
            [3, 4, 3, 4, 3] => (Minor, Eleventh),
            [4, 3, 3, 4, 3, 4] => (Dominant, Thirteenth),
            [4, 3, 4, 3, 3, 4] => (Major, Thirteenth),
            [3, 4, 3, 4, 3, 4] => (Minor, Thirteenth),
            _ => return Err(ChordError::UnknownIntervalPattern(interval.to_vec())),
        };
        Self::try_new(root, quality, number)
    }

    pub fn chord_intervals(quality: Quality, number: Number) -> Vec<Interval> {
        Self::try_chord_intervals(quality, number)
            .expect("unsupported chord quality/number combination")
    }

    /// Return adjacent intervals for a chord accepted by the legacy adapters.
    pub fn try_chord_intervals(
        quality: Quality,
        number: Number,
    ) -> Result<Vec<Interval>, ChordError> {
        let spec = legacy_spec(quality, number)?;
        formula_intervals(&spec.formula()?)
    }

    /// Deprecated compatibility name for [`Chord::parse`].
    #[deprecated(since = "0.5.0", note = "use Chord::parse or str::parse")]
    pub fn from_regex(string: &str) -> Result<Self, ChordError> {
        Self::parse(string)
    }

    fn set_inversion(&mut self, inversion: u8) -> Result<(), ChordError> {
        if inversion as usize >= self.formula.tones().len() {
            return Err(ChordError::InvalidInversion(inversion));
        }
        self.inversion = inversion;
        self.slash_bass = None;
        Ok(())
    }

    fn set_bass(&mut self, bass: Pitch) -> Result<(), ChordError> {
        if let Some(index) = self
            .root_position_notes()
            .iter()
            .position(|note| note.pitch.into_u8() == bass.into_u8())
        {
            self.set_inversion(index as u8)?;
        } else {
            self.inversion = 0;
            self.slash_bass = Some(bass);
        }
        Ok(())
    }

    fn root_position_notes(&self) -> Vec<Note> {
        let root_pitch = self.root.into_u8() as i16;
        self.formula
            .tones()
            .iter()
            .map(|tone| {
                let absolute = root_pitch + tone.semitones();
                let letter_steps = tone.degree() as i16 - 1;
                let letter = self.root.letter.offset(letter_steps);
                let pitch = Pitch::from_u8_with_letter(absolute.rem_euclid(12) as u8, letter);
                let octave_delta = (self.root.letter.index() + letter_steps).div_euclid(7);
                Note::new(pitch, self.octave.saturating_add(octave_delta))
            })
            .collect()
    }
}

impl FromStr for Chord {
    type Err = ChordError;

    fn from_str(symbol: &str) -> Result<Self, Self::Err> {
        Self::parse(symbol)
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.root, canonical_descriptor(&self.spec))?;
        if let Some(bass) = self.bass() {
            write!(f, "/{}", bass)?;
        }
        Ok(())
    }
}

impl Notes for Chord {
    fn notes(&self) -> Vec<Note> {
        let mut notes = self.root_position_notes();
        if self.inversion == 0 && self.slash_bass.is_none() {
            return notes;
        }
        notes.rotate_left(self.inversion as usize);

        if self.inversion > 0 {
            notes[0].octave = self.octave;
            let mut previous_absolute = note_absolute(&notes[0]);
            for note in notes.iter_mut().skip(1) {
                note.octave = self.octave;
                while note_absolute(note) <= previous_absolute && note.octave < i16::MAX {
                    note.octave += 1;
                }
                previous_absolute = note_absolute(note);
            }
        }

        if let Some(bass) = self.slash_bass {
            let first = &notes[0];
            let first_absolute = note_absolute(first);
            let mut bass_note = Note::new(bass, first.octave);
            while note_absolute(&bass_note) >= first_absolute && bass_note.octave > i16::MIN {
                bass_note.octave -= 1;
            }
            notes.insert(0, bass_note);
        }
        notes
    }
}

fn note_absolute(note: &Note) -> i32 {
    let natural = match note.pitch.letter {
        NoteLetter::C => 0,
        NoteLetter::D => 2,
        NoteLetter::E => 4,
        NoteLetter::F => 5,
        NoteLetter::G => 7,
        NoteLetter::A => 9,
        NoteLetter::B => 11,
    };
    note.octave as i32 * 12 + natural + note.pitch.accidental as i32
}

impl Default for Chord {
    fn default() -> Self {
        Self::new(Pitch::new(NoteLetter::C, 0), Quality::Major, Number::Triad)
    }
}

/// Builder for normalized chord specifications, modifiers and slash basses.
#[derive(Debug, Clone)]
pub struct ChordBuilder {
    root: Pitch,
    octave: i16,
    triad_quality: TriadQuality,
    seventh_quality: Option<SeventhQuality>,
    extension: ChordExtension,
    suspension: Option<Suspension>,
    modifiers: Vec<ChordModifier>,
    inversion: u8,
    bass: Option<Pitch>,
}

impl ChordBuilder {
    fn new(root: Pitch) -> Self {
        Self {
            root,
            octave: 4,
            triad_quality: TriadQuality::Major,
            seventh_quality: None,
            extension: ChordExtension::Triad,
            suspension: None,
            modifiers: Vec::new(),
            inversion: 0,
            bass: None,
        }
    }

    pub fn octave(mut self, octave: i16) -> Self {
        self.octave = octave;
        self
    }

    pub fn triad_quality(mut self, quality: TriadQuality) -> Self {
        self.triad_quality = quality;
        self
    }

    pub fn seventh_quality(mut self, quality: SeventhQuality) -> Self {
        self.seventh_quality = Some(quality);
        self
    }

    pub fn extension(mut self, extension: ChordExtension) -> Self {
        self.extension = extension;
        self
    }

    pub fn suspension(mut self, suspension: Suspension) -> Self {
        self.suspension = Some(suspension);
        self
    }

    pub fn add(mut self, degree: u8, alteration: i8) -> Result<Self, ChordError> {
        self.modifiers
            .push(ChordModifier::Add(ChordTone::new(degree, alteration)?));
        Ok(self)
    }

    pub fn alter(mut self, degree: u8, alteration: i8) -> Result<Self, ChordError> {
        self.modifiers
            .push(ChordModifier::Alter(ChordTone::new(degree, alteration)?));
        Ok(self)
    }

    pub fn omit(mut self, degree: u8) -> Self {
        self.modifiers.push(ChordModifier::Omit(degree));
        self
    }

    pub fn altered(mut self) -> Self {
        self.modifiers.push(ChordModifier::Altered);
        self
    }

    pub fn inversion(mut self, inversion: u8) -> Self {
        self.inversion = inversion;
        self.bass = None;
        self
    }

    pub fn bass(mut self, bass: Pitch) -> Self {
        self.bass = Some(bass);
        self
    }

    pub fn build(self) -> Result<Chord, ChordError> {
        let spec = ChordSpec::from_parts(
            self.triad_quality,
            self.seventh_quality,
            self.extension,
            self.suspension,
            self.modifiers,
        )?;
        let mut chord = Chord::from_spec(self.root, spec)?.with_octave(self.octave);
        if let Some(bass) = self.bass {
            chord.set_bass(bass)?;
        } else {
            chord.set_inversion(self.inversion)?;
        }
        Ok(chord)
    }
}

fn formula_intervals(formula: &ChordFormula) -> Result<Vec<Interval>, ChordError> {
    let semitones: Vec<u8> = formula
        .tones()
        .windows(2)
        .map(|window| {
            let difference = window[1].semitones() - window[0].semitones();
            let simple = difference.rem_euclid(12);
            if simple == 0 {
                12
            } else {
                simple as u8
            }
        })
        .collect();
    Interval::from_semitones(&semitones).map_err(|_| {
        ChordError::InvalidSpecification("formula cannot be represented as intervals".to_string())
    })
}

fn legacy_spec(quality: Quality, number: Number) -> Result<ChordSpec, ChordError> {
    use Number::*;
    use Quality::*;

    if quality == Power {
        if matches!(number, Triad | Fifth) {
            return ChordSpec::new(TriadQuality::Power, None, ChordExtension::Triad);
        }
        return unsupported(quality, number);
    }
    if number == Fifth {
        return unsupported(quality, number);
    }
    if quality == HalfDiminished && number != Seventh {
        return unsupported(quality, number);
    }

    let supported = match quality {
        Major | Minor => matches!(
            number,
            Triad | Sixth | SixNine | Seventh | MajorSeventh | Ninth | Eleventh | Thirteenth
        ),
        Dominant => matches!(number, Seventh | Ninth | Eleventh | Thirteenth),
        Diminished => matches!(number, Triad | Seventh),
        HalfDiminished => number == Seventh,
        Augmented => matches!(number, Triad | Seventh | MajorSeventh),
        Suspended2 | Suspended4 => number == Triad,
        Power => unreachable!(),
    };
    if !supported {
        return unsupported(quality, number);
    }

    let extension = match number {
        Triad => ChordExtension::Triad,
        Sixth => ChordExtension::Sixth,
        SixNine => ChordExtension::SixNine,
        Seventh | MajorSeventh => ChordExtension::Seventh,
        Ninth => ChordExtension::Ninth,
        Eleventh => ChordExtension::Eleventh,
        Thirteenth => ChordExtension::Thirteenth,
        Fifth => unreachable!(),
    };

    let (triad, suspension) = match quality {
        Major | Dominant => (TriadQuality::Major, None),
        Minor => (TriadQuality::Minor, None),
        Diminished | HalfDiminished => (TriadQuality::Diminished, None),
        Augmented => (TriadQuality::Augmented, None),
        Suspended2 => (TriadQuality::Major, Some(Suspension::Second)),
        Suspended4 => (TriadQuality::Major, Some(Suspension::Fourth)),
        Power => unreachable!(),
    };

    if matches!(triad, TriadQuality::Diminished | TriadQuality::Augmented)
        && !matches!(extension, ChordExtension::Triad | ChordExtension::Seventh)
    {
        return unsupported(quality, number);
    }
    if matches!(extension, ChordExtension::Sixth | ChordExtension::SixNine)
        && !matches!(triad, TriadQuality::Major | TriadQuality::Minor)
    {
        return unsupported(quality, number);
    }

    let seventh = if matches!(
        extension,
        ChordExtension::Seventh
            | ChordExtension::Ninth
            | ChordExtension::Eleventh
            | ChordExtension::Thirteenth
    ) {
        Some(if number == MajorSeventh {
            SeventhQuality::Major
        } else {
            match quality {
                Major => SeventhQuality::Major,
                Diminished => SeventhQuality::Diminished,
                HalfDiminished | Dominant | Minor | Suspended2 | Suspended4 | Augmented => {
                    SeventhQuality::Minor
                }
                Power => unreachable!(),
            }
        })
    } else {
        None
    };

    ChordSpec::from_parts(triad, seventh, extension, suspension, Vec::new())
}

fn unsupported<T>(quality: Quality, number: Number) -> Result<T, ChordError> {
    Err(ChordError::UnsupportedChord(format!(
        "{} {}",
        quality, number
    )))
}

fn legacy_quality(spec: &ChordSpec) -> Quality {
    if let Some(suspension) = spec.suspension() {
        return match suspension {
            Suspension::Second => Quality::Suspended2,
            Suspension::Fourth => Quality::Suspended4,
        };
    }
    match spec.triad_quality() {
        TriadQuality::Power => Quality::Power,
        TriadQuality::Minor => Quality::Minor,
        TriadQuality::Diminished => {
            if spec.seventh_quality() == Some(SeventhQuality::Minor) {
                Quality::HalfDiminished
            } else {
                Quality::Diminished
            }
        }
        TriadQuality::Augmented => Quality::Augmented,
        TriadQuality::Major => {
            if spec.seventh_quality() == Some(SeventhQuality::Minor) {
                Quality::Dominant
            } else {
                Quality::Major
            }
        }
    }
}

fn legacy_number(spec: &ChordSpec) -> Number {
    match spec.extension() {
        ChordExtension::Triad => {
            if spec.triad_quality() == TriadQuality::Power {
                Number::Fifth
            } else {
                Number::Triad
            }
        }
        ChordExtension::Sixth => Number::Sixth,
        ChordExtension::SixNine => Number::SixNine,
        ChordExtension::Seventh => {
            if spec.seventh_quality() == Some(SeventhQuality::Major) {
                Number::MajorSeventh
            } else {
                Number::Seventh
            }
        }
        ChordExtension::Ninth => Number::Ninth,
        ChordExtension::Eleventh => Number::Eleventh,
        ChordExtension::Thirteenth => Number::Thirteenth,
    }
}

fn canonical_descriptor(spec: &ChordSpec) -> String {
    if spec.modifiers() == [ChordModifier::Altered] {
        return "7alt".to_string();
    }

    let extension = spec.extension();
    let seventh = spec.seventh_quality();
    let mut descriptor = match (spec.triad_quality(), extension, seventh) {
        (TriadQuality::Power, ChordExtension::Triad, _) => "5".to_string(),
        (TriadQuality::Major, ChordExtension::Triad, _) => String::new(),
        (TriadQuality::Major, ChordExtension::Sixth, _) => "6".to_string(),
        (TriadQuality::Major, ChordExtension::SixNine, _) => "6/9".to_string(),
        (TriadQuality::Major, ChordExtension::Seventh, Some(SeventhQuality::Minor)) => {
            "7".to_string()
        }
        (TriadQuality::Major, ChordExtension::Seventh, Some(SeventhQuality::Major)) => {
            "maj7".to_string()
        }
        (TriadQuality::Major, ChordExtension::Ninth, Some(SeventhQuality::Minor)) => {
            "9".to_string()
        }
        (TriadQuality::Major, ChordExtension::Ninth, Some(SeventhQuality::Major)) => {
            "maj9".to_string()
        }
        (TriadQuality::Major, ChordExtension::Eleventh, Some(SeventhQuality::Minor)) => {
            "11".to_string()
        }
        (TriadQuality::Major, ChordExtension::Eleventh, Some(SeventhQuality::Major)) => {
            "maj11".to_string()
        }
        (TriadQuality::Major, ChordExtension::Thirteenth, Some(SeventhQuality::Minor)) => {
            "13".to_string()
        }
        (TriadQuality::Major, ChordExtension::Thirteenth, Some(SeventhQuality::Major)) => {
            "maj13".to_string()
        }
        (TriadQuality::Minor, ChordExtension::Triad, _) => "m".to_string(),
        (TriadQuality::Minor, ChordExtension::Sixth, _) => "m6".to_string(),
        (TriadQuality::Minor, ChordExtension::SixNine, _) => "m6/9".to_string(),
        (TriadQuality::Minor, ChordExtension::Seventh, Some(SeventhQuality::Minor)) => {
            "m7".to_string()
        }
        (TriadQuality::Minor, ChordExtension::Seventh, Some(SeventhQuality::Major)) => {
            "mMaj7".to_string()
        }
        (TriadQuality::Minor, ChordExtension::Ninth, Some(SeventhQuality::Minor)) => {
            "m9".to_string()
        }
        (TriadQuality::Minor, ChordExtension::Ninth, Some(SeventhQuality::Major)) => {
            "mMaj9".to_string()
        }
        (TriadQuality::Minor, ChordExtension::Eleventh, Some(SeventhQuality::Minor)) => {
            "m11".to_string()
        }
        (TriadQuality::Minor, ChordExtension::Eleventh, Some(SeventhQuality::Major)) => {
            "mMaj11".to_string()
        }
        (TriadQuality::Minor, ChordExtension::Thirteenth, Some(SeventhQuality::Minor)) => {
            "m13".to_string()
        }
        (TriadQuality::Minor, ChordExtension::Thirteenth, Some(SeventhQuality::Major)) => {
            "mMaj13".to_string()
        }
        (TriadQuality::Diminished, ChordExtension::Triad, _) => "dim".to_string(),
        (TriadQuality::Diminished, ChordExtension::Seventh, Some(SeventhQuality::Diminished)) => {
            "dim7".to_string()
        }
        (TriadQuality::Diminished, ChordExtension::Seventh, Some(SeventhQuality::Minor)) => {
            "m7b5".to_string()
        }
        (TriadQuality::Augmented, ChordExtension::Triad, _) => "aug".to_string(),
        (TriadQuality::Augmented, ChordExtension::Seventh, Some(SeventhQuality::Minor)) => {
            "aug7".to_string()
        }
        (TriadQuality::Augmented, ChordExtension::Seventh, Some(SeventhQuality::Major)) => {
            "augMaj7".to_string()
        }
        _ => unreachable!("validated chord specification"),
    };

    // Without an explicit major marker, a root-position alteration is
    // indistinguishable from a root accidental (`C(b5)` would become `Cb5`).
    if descriptor.is_empty()
        && spec.suspension().is_none()
        && matches!(spec.modifiers().first(), Some(ChordModifier::Alter(_)))
    {
        descriptor.push_str("maj");
    }

    if let Some(suspension) = spec.suspension() {
        descriptor.push_str(match suspension {
            Suspension::Second => "sus2",
            Suspension::Fourth => "sus4",
        });
    }
    for modifier in spec.modifiers() {
        descriptor.push_str(&format_modifier(modifier));
    }
    descriptor
}
