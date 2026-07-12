use crate::chord::{
    ChordError, ChordExtension, ChordModifier, ChordSpec, ChordTone, SeventhQuality, Suspension,
    TriadQuality,
};
use crate::note::Pitch;

pub(crate) enum ParsedSlash {
    Bass(Pitch),
    Inversion { value: u8, position: usize },
}

pub(crate) struct ParsedChord {
    pub root: Pitch,
    pub spec: ChordSpec,
    pub slash: Option<ParsedSlash>,
}

#[derive(Clone)]
struct MappedText {
    text: String,
    positions: Vec<usize>,
    end: usize,
}

impl MappedText {
    fn from_source(input: &str, offset: usize) -> Self {
        let leading = input.len() - input.trim_start().len();
        let trimmed = input.trim();
        let mut positions = Vec::with_capacity(trimmed.len());
        for (position, character) in trimmed.char_indices() {
            for _ in 0..character.len_utf8() {
                positions.push(offset + leading + position);
            }
        }
        Self {
            text: trimmed.to_string(),
            positions,
            end: offset + leading + trimmed.len(),
        }
    }

    fn position(&self, normalized_offset: usize) -> usize {
        self.positions
            .get(normalized_offset)
            .copied()
            .unwrap_or(self.end)
    }

    fn push_replacement(
        output: &mut String,
        positions: &mut Vec<usize>,
        replacement: &str,
        source_position: usize,
    ) {
        output.push_str(replacement);
        for _ in 0..replacement.len() {
            positions.push(source_position);
        }
    }

    fn map_characters<F>(&self, mut replacement: F) -> Self
    where
        F: FnMut(usize, char, &str) -> String,
    {
        let mut text = String::new();
        let mut positions = Vec::new();
        for (position, character) in self.text.char_indices() {
            let mapped = replacement(position, character, &self.text[position..]);
            Self::push_replacement(&mut text, &mut positions, &mapped, self.position(position));
        }
        Self {
            text,
            positions,
            end: self.end,
        }
    }

    fn replace_all(&self, needle: &str, replacement: &str) -> Self {
        let mut text = String::new();
        let mut positions = Vec::new();
        let mut cursor = 0;
        while cursor < self.text.len() {
            if self.text[cursor..].starts_with(needle) {
                Self::push_replacement(
                    &mut text,
                    &mut positions,
                    replacement,
                    self.position(cursor),
                );
                cursor += needle.len();
            } else {
                let character = self.text[cursor..].chars().next().unwrap();
                let end = cursor + character.len_utf8();
                text.push(character);
                positions.extend_from_slice(&self.positions[cursor..end]);
                cursor = end;
            }
        }
        Self {
            text,
            positions,
            end: self.end,
        }
    }

    fn filter_characters<F>(&self, mut keep: F) -> Self
    where
        F: FnMut(char) -> bool,
    {
        let mut text = String::new();
        let mut positions = Vec::new();
        for (position, character) in self.text.char_indices() {
            if keep(character) {
                let end = position + character.len_utf8();
                text.push(character);
                positions.extend_from_slice(&self.positions[position..end]);
            }
        }
        Self {
            text,
            positions,
            end: self.end,
        }
    }

    fn replace_prefix(&self, prefix_len: usize, replacement: &str) -> Self {
        let mut text = String::new();
        let mut positions = Vec::new();
        Self::push_replacement(&mut text, &mut positions, replacement, self.position(0));
        text.push_str(&self.text[prefix_len..]);
        positions.extend_from_slice(&self.positions[prefix_len..]);
        Self {
            text,
            positions,
            end: self.end,
        }
    }

    fn slice(&self, start: usize, end: usize) -> Self {
        Self {
            text: self.text[start..end].to_string(),
            positions: self.positions[start..end].to_vec(),
            end: self.position(end),
        }
    }

    fn append_literal(&mut self, literal: &str, source_position: usize) {
        Self::push_replacement(
            &mut self.text,
            &mut self.positions,
            literal,
            source_position,
        );
    }

    fn append(&mut self, other: &Self) {
        self.text.push_str(&other.text);
        self.positions.extend_from_slice(&other.positions);
        self.end = other.end;
    }
}

pub(crate) fn parse_chord(input: &str) -> Result<ParsedChord, ChordError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(ChordError::EmptySymbol);
    }

    let lowercase = input.to_lowercase();
    if lowercase.contains("half") && lowercase.contains("diminished") && lowercase.contains("triad")
    {
        return Err(ChordError::UnsupportedConstruction {
            position: lowercase.find("half").unwrap_or(0),
            message: "half-diminished describes a seventh chord, not a triad".to_string(),
        });
    }

    let (root, root_end) = parse_root(input)?;
    let source_descriptor = &input[root_end..];
    validate_parentheses(source_descriptor, root_end)?;

    let normalized = normalize_descriptor_mapped(source_descriptor, root_end);
    let (descriptor, slash_text) = split_slash(&normalized)?;
    let spec = parse_descriptor(&descriptor, root_end)?;
    let slash = match slash_text {
        None => None,
        Some(text) => {
            let position = text.position(0);
            if text.text.is_empty() {
                return Err(ChordError::InvalidSlashBass { position });
            }
            if text
                .text
                .chars()
                .all(|character| character.is_ascii_digit())
            {
                let inversion = text
                    .text
                    .parse::<u8>()
                    .map_err(|_| ChordError::InvalidSlashBass { position })?;
                Some(ParsedSlash::Inversion {
                    value: inversion,
                    position,
                })
            } else {
                let bass =
                    Pitch::from_str(&text.text).ok_or(ChordError::InvalidSlashBass { position })?;
                Some(ParsedSlash::Bass(bass))
            }
        }
    };

    Ok(ParsedChord { root, spec, slash })
}

fn validate_parentheses(input: &str, offset: usize) -> Result<(), ChordError> {
    let mut opening = None;
    let characters: Vec<(usize, char)> = input.char_indices().collect();
    for (index, (position, character)) in characters.iter().enumerate() {
        if *character != ',' {
            continue;
        }

        let previous = characters[..index]
            .iter()
            .rev()
            .find(|(_, character)| !character.is_whitespace())
            .map(|(_, character)| *character);
        let next = characters[index + 1..]
            .iter()
            .find(|(_, character)| !character.is_whitespace())
            .map(|(_, character)| *character);
        if matches!(previous, None | Some('(' | ','))
            || matches!(next, None | Some(')' | ',' | '/'))
        {
            return Err(ChordError::UnexpectedToken {
                position: offset + position,
                token: ",".to_string(),
            });
        }
    }

    for (position, character) in input.char_indices() {
        match character {
            '(' if opening.is_none() => opening = Some(position),
            '(' => {
                return Err(ChordError::UnexpectedToken {
                    position: offset + position,
                    token: "(".to_string(),
                })
            }
            ')' => match opening.take() {
                None => {
                    return Err(ChordError::UnexpectedToken {
                        position: offset + position,
                        token: ")".to_string(),
                    })
                }
                Some(start)
                    if input[start + 1..position]
                        .chars()
                        .all(|character| character.is_whitespace() || character == ',') =>
                {
                    return Err(ChordError::UnexpectedToken {
                        position: offset + start,
                        token: "()".to_string(),
                    })
                }
                Some(_) => {}
            },
            _ => {}
        }
    }
    if let Some(position) = opening {
        return Err(ChordError::UnexpectedToken {
            position: offset + position,
            token: "(".to_string(),
        });
    }
    Ok(())
}

fn parse_root(input: &str) -> Result<(Pitch, usize), ChordError> {
    let mut characters = input.char_indices();
    let (_, first) = characters
        .next()
        .ok_or(ChordError::InvalidRoot { position: 0 })?;
    if !matches!(first, 'A'..='G' | 'a'..='g') {
        return Err(ChordError::InvalidRoot { position: 0 });
    }

    let mut end = first.len_utf8();
    for (index, character) in characters {
        let is_sus_start = matches!(character, 's' | 'S')
            && input[index + character.len_utf8()..]
                .to_ascii_lowercase()
                .starts_with("us");
        if is_sus_start {
            break;
        }
        if matches!(
            character,
            'b' | '♭' | '𝄫' | '#' | '♯' | 's' | 'S' | '𝄪' | 'x'
        ) {
            end = index + character.len_utf8();
        } else {
            break;
        }
    }

    Pitch::from_str(&input[..end])
        .map(|pitch| (pitch, end))
        .ok_or(ChordError::InvalidRoot { position: 0 })
}

fn normalize_descriptor_mapped(input: &str, offset: usize) -> MappedText {
    let mut text = MappedText::from_source(input, offset);
    text = text.map_characters(|_, character, tail| {
        let lowercase_tail = tail.to_ascii_lowercase();
        let starts_long_quality = lowercase_tail.starts_with("maj")
            || lowercase_tail.starts_with("major")
            || lowercase_tail.starts_with("min")
            || lowercase_tail.starts_with("minor");
        if character == 'M' && !starts_long_quality {
            "maj".to_string()
        } else {
            character.to_string()
        }
    });
    text = expand_delta_aliases_mapped(&text);
    text = text.map_characters(|_, character, _| match character {
        'Δ' | '∆' | '^' => "maj".to_string(),
        'ø' | 'Ø' => "halfdim".to_string(),
        '°' | 'º' => "dim".to_string(),
        '♭' => "b".to_string(),
        '♯' => "#".to_string(),
        _ => character.to_lowercase().collect(),
    });
    for (long, short) in [
        ("half diminished", "halfdim"),
        ("half-diminished", "halfdim"),
        ("halfdiminished", "halfdim"),
        ("major", "maj"),
        ("minor", "m"),
        ("diminished", "dim"),
        ("augmented", "aug"),
        ("dominant", "dom"),
        ("suspended", "sus"),
        ("thirteenth", "13"),
        ("eleventh", "11"),
        ("ninth", "9"),
        ("seventh", "7"),
        ("sixth", "6"),
        ("fifth", "5"),
        ("triad", ""),
    ] {
        text = text.replace_all(long, short);
    }
    text = text.filter_characters(|character| {
        !character.is_whitespace() && !matches!(character, '(' | ')' | ',')
    });
    text = text
        .replace_all("m/maj", "mmaj")
        .replace_all("m/ma", "mmaj")
        .replace_all("6/9", "6_9");

    let slash = text.text.find('/');
    let (core, bass, slash_position) = match slash {
        Some(position) => (
            text.slice(0, position),
            Some(text.slice(position + 1, text.text.len())),
            Some(text.position(position)),
        ),
        None => (text.clone(), None, None),
    };
    let mut core = normalize_core_alias_mapped(core).replace_all("6_9", "6/9");
    if let (Some(bass), Some(slash_position)) = (bass, slash_position) {
        core.append_literal("/", slash_position);
        core.append(&bass);
    }
    core
}

fn expand_delta_aliases_mapped(input: &MappedText) -> MappedText {
    input.map_characters(|_, character, tail| {
        if !matches!(character, 'Δ' | '∆') {
            return character.to_string();
        }
        let tail = &tail[character.len_utf8()..];
        let trimmed = tail.trim_start();
        let has_explicit_extension = ["6", "7", "9", "11", "13"]
            .iter()
            .any(|extension| trimmed.starts_with(extension));
        if has_explicit_extension {
            "maj".to_string()
        } else {
            "maj7".to_string()
        }
    })
}

fn normalize_core_alias_mapped(mut core: MappedText) -> MappedText {
    if core.text.starts_with('-') {
        core = core.replace_prefix(1, "m");
    } else if core.text.starts_with('+') {
        core = core.replace_prefix(1, "aug");
    } else if core.text == "o" || core.text.starts_with("o7") {
        core = core.replace_prefix(1, "dim");
    }

    if core.text == "0" || core.text == "07" {
        return core.replace_prefix(1, "dim");
    }
    if core.text == "h" || core.text == "h7" {
        let len = core.text.len();
        return core.replace_prefix(len, "halfdim7");
    }
    if core.text.starts_with("m7b5") {
        return core.replace_prefix(4, "halfdim7");
    }
    if core.text.starts_with("mi") && !core.text.starts_with("min") {
        core = core.replace_prefix(2, "m");
    } else if core
        .text
        .strip_prefix("ma")
        .map(|rest| matches!(rest, "7" | "9" | "11" | "13"))
        .unwrap_or(false)
    {
        core = core.replace_prefix(2, "maj");
    }

    for (alias, canonical) in [
        ("maj6add9", "6/9"),
        ("m6add9", "m6/9"),
        ("6add9", "6/9"),
        ("maj69", "6/9"),
        ("m69", "m6/9"),
        ("69", "6/9"),
    ] {
        if core.text.starts_with(alias) {
            return core.replace_prefix(alias.len(), canonical);
        }
    }

    match core.text.as_str() {
        "2" => core.replace_prefix(1, "add2"),
        "4" => core.replace_prefix(1, "add4"),
        "alt7" => core.replace_prefix(4, "7alt"),
        "7+" | "7aug" => {
            let len = core.text.len();
            core.replace_prefix(len, "aug7")
        }
        _ => core,
    }
}

fn split_slash(descriptor: &MappedText) -> Result<(MappedText, Option<MappedText>), ChordError> {
    let protected = descriptor.replace_all("6/9", "6_9");
    let mut slashes = protected
        .text
        .match_indices('/')
        .map(|(position, _)| position);
    let first = slashes.next();
    if slashes.next().is_some() {
        return Err(ChordError::InvalidSlashBass {
            position: protected.position(first.unwrap() + 1),
        });
    }
    match first {
        Some(position) => Ok((
            protected.slice(0, position).replace_all("6_9", "6/9"),
            Some(protected.slice(position + 1, protected.text.len())),
        )),
        None => Ok((protected.replace_all("6_9", "6/9"), None)),
    }
}

fn parse_descriptor(descriptor: &MappedText, root_offset: usize) -> Result<ChordSpec, ChordError> {
    let mut rest = descriptor.text.as_str();
    let mut suspension = None;

    if let Some((parsed, remaining)) = take_suspension(rest) {
        suspension = Some(parsed);
        rest = remaining;
    }

    let mut triad_quality = TriadQuality::Major;
    let mut explicit_major = false;
    let mut explicit_dominant = false;
    let mut half_diminished = false;
    let mut major_seventh_marker = false;

    if let Some(remaining) = rest.strip_prefix("halfdim") {
        triad_quality = TriadQuality::Diminished;
        half_diminished = true;
        rest = remaining;
    } else if let Some(remaining) = rest.strip_prefix("mmaj") {
        triad_quality = TriadQuality::Minor;
        major_seventh_marker = true;
        rest = remaining;
    } else if let Some(remaining) = rest.strip_prefix("maj") {
        triad_quality = TriadQuality::Major;
        explicit_major = true;
        major_seventh_marker = true;
        rest = remaining;
    } else if let Some(remaining) = rest.strip_prefix("min") {
        triad_quality = TriadQuality::Minor;
        rest = remaining;
    } else if let Some(remaining) = rest.strip_prefix('m') {
        triad_quality = TriadQuality::Minor;
        rest = remaining;
    } else if let Some(remaining) = rest.strip_prefix("dim") {
        triad_quality = TriadQuality::Diminished;
        rest = remaining;
    } else if let Some(remaining) = rest.strip_prefix("aug") {
        triad_quality = TriadQuality::Augmented;
        rest = remaining;
        if let Some(remaining) = rest.strip_prefix("maj") {
            major_seventh_marker = true;
            rest = remaining;
        }
    } else if let Some(remaining) = rest.strip_prefix("dom") {
        explicit_dominant = true;
        rest = remaining;
    }

    if suspension.is_none() {
        if let Some((parsed, remaining)) = take_suspension(rest) {
            suspension = Some(parsed);
            rest = remaining;
        }
    }

    let explicit_fifth = rest.starts_with('5');
    let (mut extension, remaining) = take_extension(rest);
    rest = remaining;

    if suspension.is_none() {
        if let Some((parsed, remaining)) = take_suspension(rest) {
            suspension = Some(parsed);
            rest = remaining;
        }
    }

    if extension.is_none() && (half_diminished || explicit_dominant || rest.starts_with("alt")) {
        extension = Some(ChordExtension::Seventh);
    }

    if explicit_fifth {
        if explicit_major
            || explicit_dominant
            || half_diminished
            || major_seventh_marker
            || triad_quality != TriadQuality::Major
        {
            return Err(ChordError::UnsupportedConstruction {
                position: root_offset,
                message: "the fifth marker cannot be combined with another chord quality"
                    .to_string(),
            });
        }
        triad_quality = TriadQuality::Power;
    }

    let extension = extension.unwrap_or(ChordExtension::Triad);
    let uses_seventh = matches!(
        extension,
        ChordExtension::Seventh
            | ChordExtension::Ninth
            | ChordExtension::Eleventh
            | ChordExtension::Thirteenth
    );
    if major_seventh_marker && triad_quality != TriadQuality::Major && !uses_seventh {
        return Err(ChordError::UnsupportedConstruction {
            position: root_offset,
            message: "minor-major and augmented-major qualities require a seventh or extension"
                .to_string(),
        });
    }
    if explicit_dominant && !uses_seventh {
        return Err(ChordError::UnsupportedConstruction {
            position: root_offset,
            message: "dominant quality requires a seventh or extension".to_string(),
        });
    }
    let seventh_quality = if matches!(
        extension,
        ChordExtension::Seventh
            | ChordExtension::Ninth
            | ChordExtension::Eleventh
            | ChordExtension::Thirteenth
    ) {
        Some(if half_diminished {
            SeventhQuality::Minor
        } else if major_seventh_marker || explicit_major {
            SeventhQuality::Major
        } else {
            match triad_quality {
                TriadQuality::Diminished => SeventhQuality::Diminished,
                _ => SeventhQuality::Minor,
            }
        })
    } else {
        None
    };

    let mut modifiers = Vec::new();
    let mut modifier_offsets = Vec::new();
    let mut consumed = descriptor.text.len() - rest.len();
    while !rest.is_empty() {
        if let Some((parsed, remaining)) = take_suspension(rest) {
            if suspension.replace(parsed).is_some() {
                return Err(ChordError::ConflictingModifiersAt {
                    position: descriptor.position(consumed),
                    message: "a chord cannot contain two suspensions".to_string(),
                });
            }
            consumed += rest.len() - remaining.len();
            rest = remaining;
            continue;
        }
        if let Some(remaining) = rest.strip_prefix("alt") {
            push_modifier(
                &mut modifiers,
                &mut modifier_offsets,
                ChordModifier::Altered,
                consumed,
                descriptor,
            )?;
            consumed += 3;
            rest = remaining;
            continue;
        }
        if let Some(remaining) = rest.strip_prefix("add") {
            let (tone, remaining) =
                take_tone(remaining, true).ok_or_else(|| ChordError::UnexpectedToken {
                    position: descriptor.position(consumed),
                    token: rest.to_string(),
                })?;
            push_modifier(
                &mut modifiers,
                &mut modifier_offsets,
                ChordModifier::Add(tone),
                consumed,
                descriptor,
            )?;
            consumed += rest.len() - remaining.len();
            rest = remaining;
            continue;
        }
        if let Some(remaining) = rest
            .strip_prefix("omit")
            .or_else(|| rest.strip_prefix("no"))
        {
            let (degree, remaining) =
                take_degree(remaining).ok_or_else(|| ChordError::UnexpectedToken {
                    position: descriptor.position(consumed),
                    token: rest.to_string(),
                })?;
            push_modifier(
                &mut modifiers,
                &mut modifier_offsets,
                ChordModifier::Omit(degree),
                consumed,
                descriptor,
            )?;
            consumed += rest.len() - remaining.len();
            rest = remaining;
            continue;
        }
        if rest.starts_with('b') || rest.starts_with('#') {
            let (tone, remaining) =
                take_tone(rest, false).ok_or_else(|| ChordError::UnexpectedToken {
                    position: descriptor.position(consumed),
                    token: rest.to_string(),
                })?;
            push_modifier(
                &mut modifiers,
                &mut modifier_offsets,
                ChordModifier::Alter(tone),
                consumed,
                descriptor,
            )?;
            consumed += rest.len() - remaining.len();
            rest = remaining;
            continue;
        }

        return Err(ChordError::UnexpectedToken {
            position: descriptor.position(consumed),
            token: rest.to_string(),
        });
    }

    let modifier_error_position = modifier_offsets
        .last()
        .map(|position| descriptor.position(*position))
        .unwrap_or(root_offset);
    ChordSpec::from_parts(
        triad_quality,
        seventh_quality,
        extension,
        suspension,
        modifiers,
    )
    .map_err(|error| match error {
        ChordError::InvalidModifier(modifier) => ChordError::InvalidModifierAt {
            position: modifier_error_position,
            modifier,
        },
        ChordError::ConflictingModifiers(message) => ChordError::ConflictingModifiersAt {
            position: modifier_error_position,
            message,
        },
        ChordError::InvalidSpecification(message) => ChordError::UnsupportedConstruction {
            position: root_offset,
            message,
        },
        error => error,
    })
}

fn push_modifier(
    modifiers: &mut Vec<ChordModifier>,
    offsets: &mut Vec<usize>,
    modifier: ChordModifier,
    normalized_offset: usize,
    descriptor: &MappedText,
) -> Result<(), ChordError> {
    let position = descriptor.position(normalized_offset);
    if modifiers.iter().any(|existing| existing == &modifier) {
        return Err(ChordError::ConflictingModifiersAt {
            position,
            message: "duplicate modifier".to_string(),
        });
    }
    if modifiers.iter().any(|existing| {
        parser_modifier_degree(existing).is_some()
            && parser_modifier_degree(existing) == parser_modifier_degree(&modifier)
    }) {
        return Err(ChordError::ConflictingModifiersAt {
            position,
            message: "multiple operations target the same degree".to_string(),
        });
    }
    if matches!(modifier, ChordModifier::Altered) && !modifiers.is_empty()
        || modifiers
            .iter()
            .any(|existing| matches!(existing, ChordModifier::Altered))
    {
        return Err(ChordError::ConflictingModifiersAt {
            position,
            message: "alt cannot be combined with another modifier".to_string(),
        });
    }
    modifiers.push(modifier);
    offsets.push(normalized_offset);
    Ok(())
}

fn parser_modifier_degree(modifier: &ChordModifier) -> Option<u8> {
    match modifier {
        ChordModifier::Add(tone) | ChordModifier::Alter(tone) => Some(tone.degree()),
        ChordModifier::Omit(degree) => Some(*degree),
        ChordModifier::Altered => None,
    }
}

fn take_extension(input: &str) -> (Option<ChordExtension>, &str) {
    for (token, extension) in [
        ("6/9", ChordExtension::SixNine),
        ("13", ChordExtension::Thirteenth),
        ("11", ChordExtension::Eleventh),
        ("9", ChordExtension::Ninth),
        ("7", ChordExtension::Seventh),
        ("6", ChordExtension::Sixth),
        ("5", ChordExtension::Triad),
    ] {
        if let Some(remaining) = input.strip_prefix(token) {
            return (Some(extension), remaining);
        }
    }
    (None, input)
}

fn take_suspension(input: &str) -> Option<(Suspension, &str)> {
    if let Some(remaining) = input.strip_prefix("sus2") {
        Some((Suspension::Second, remaining))
    } else if let Some(remaining) = input.strip_prefix("sus4") {
        Some((Suspension::Fourth, remaining))
    } else {
        input
            .strip_prefix("sus")
            .map(|remaining| (Suspension::Fourth, remaining))
    }
}

fn take_tone(input: &str, alteration_optional: bool) -> Option<(ChordTone, &str)> {
    let mut alteration: i8 = 0;
    let mut rest = input;
    while let Some(remaining) = rest.strip_prefix('b') {
        alteration = alteration.checked_sub(1)?;
        rest = remaining;
    }
    while let Some(remaining) = rest.strip_prefix('#') {
        alteration = alteration.checked_add(1)?;
        rest = remaining;
    }
    if alteration == 0 && !alteration_optional {
        return None;
    }
    let (degree, remaining) = take_degree(rest)?;
    let tone = ChordTone::new(degree, alteration).ok()?;
    Some((tone, remaining))
}

fn take_degree(input: &str) -> Option<(u8, &str)> {
    for degree in [13u8, 11, 9, 7, 6, 5, 4, 3, 2, 1] {
        let token = degree.to_string();
        if let Some(remaining) = input.strip_prefix(&token) {
            return Some((degree, remaining));
        }
    }
    None
}
