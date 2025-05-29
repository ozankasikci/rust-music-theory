use crate::note::{NoteLetter, Pitch, PitchSymbol};
use crate::scale::Mode;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEY_SIGNATURE_SPELLINGS: HashMap<(NoteLetter, i8), Vec<PitchSymbol>> = {
        let mut m = HashMap::new();
        m.insert((NoteLetter::C, 0), vec![PitchSymbol::C, PitchSymbol::D, PitchSymbol::E, PitchSymbol::F, PitchSymbol::G, PitchSymbol::A, PitchSymbol::B]);
        m.insert((NoteLetter::G, 0), vec![PitchSymbol::G, PitchSymbol::A, PitchSymbol::B, PitchSymbol::C, PitchSymbol::D, PitchSymbol::E, PitchSymbol::Fs]);
        m.insert((NoteLetter::D, 0), vec![PitchSymbol::D, PitchSymbol::E, PitchSymbol::Fs, PitchSymbol::G, PitchSymbol::A, PitchSymbol::B, PitchSymbol::Cs]);
        m.insert((NoteLetter::A, 0), vec![PitchSymbol::A, PitchSymbol::B, PitchSymbol::Cs, PitchSymbol::D, PitchSymbol::E, PitchSymbol::Fs, PitchSymbol::Gs]);
        m.insert((NoteLetter::E, 0), vec![PitchSymbol::E, PitchSymbol::Fs, PitchSymbol::Gs, PitchSymbol::A, PitchSymbol::B, PitchSymbol::Cs, PitchSymbol::Ds]);
        m.insert((NoteLetter::B, 0), vec![PitchSymbol::B, PitchSymbol::Cs, PitchSymbol::Ds, PitchSymbol::E, PitchSymbol::Fs, PitchSymbol::Gs, PitchSymbol::As]);
        m.insert((NoteLetter::F, 0), vec![PitchSymbol::F, PitchSymbol::G, PitchSymbol::A, PitchSymbol::Bb, PitchSymbol::C, PitchSymbol::D, PitchSymbol::E]);
        m.insert((NoteLetter::B, -1), vec![PitchSymbol::Bb, PitchSymbol::C, PitchSymbol::D, PitchSymbol::Eb, PitchSymbol::F, PitchSymbol::G, PitchSymbol::A]);
        m.insert((NoteLetter::E, -1), vec![PitchSymbol::Eb, PitchSymbol::F, PitchSymbol::G, PitchSymbol::Ab, PitchSymbol::Bb, PitchSymbol::C, PitchSymbol::D]);
        m.insert((NoteLetter::A, -1), vec![PitchSymbol::Ab, PitchSymbol::Bb, PitchSymbol::C, PitchSymbol::Db, PitchSymbol::Eb, PitchSymbol::F, PitchSymbol::G]);
        m.insert((NoteLetter::D, -1), vec![PitchSymbol::Db, PitchSymbol::Eb, PitchSymbol::F, PitchSymbol::Gb, PitchSymbol::Ab, PitchSymbol::Bb, PitchSymbol::C]);
        m.insert((NoteLetter::G, -1), vec![PitchSymbol::Gb, PitchSymbol::Ab, PitchSymbol::Bb, PitchSymbol::Cb, PitchSymbol::Db, PitchSymbol::Eb, PitchSymbol::F]);
        m.insert((NoteLetter::F, 1), vec![PitchSymbol::Fs, PitchSymbol::Gs, PitchSymbol::As, PitchSymbol::B, PitchSymbol::Cs, PitchSymbol::Ds, PitchSymbol::Es]);
        m.insert((NoteLetter::C, 1), vec![PitchSymbol::Cs, PitchSymbol::Ds, PitchSymbol::Es, PitchSymbol::Fs, PitchSymbol::Gs, PitchSymbol::As, PitchSymbol::Bs]);
        m
    };
}

/// A key signature.
#[derive(Debug, Clone)]
pub struct KeySignature {
    /// The tonic of the key signature.
    pub tonic: Pitch,
    /// The mode of the key signature.
    pub mode: Option<Mode>,
}

impl KeySignature {
    /// Create a new key signature.
    pub fn new(tonic: Pitch) -> Self {
        KeySignature {
            tonic,
            mode: None,
        }
    }

    /// Create a new key signature with a mode.
    pub fn new_with_mode(tonic: Pitch, mode: Option<Mode>) -> Self {
        KeySignature { tonic, mode }
    }

    /// Get the relative major key for a given semitone value
    fn get_relative_major_key(&self, semitones: u8) -> (NoteLetter, i8) {
        use NoteLetter::*;
        match semitones {
            0 => (C, 0),   // C major
            1 => (D, -1),  // Db major
            2 => (D, 0),   // D major
            3 => (E, -1),  // Eb major
            4 => (E, 0),   // E major
            5 => (F, 0),   // F major
            6 => (F, 1),   // F# major
            7 => (G, 0),   // G major
            8 => (A, -1),  // Ab major
            9 => (A, 0),   // A major
            10 => (B, -1), // Bb major
            11 => (B, 0),  // B major
            _ => unreachable!(),
        }
    }

    pub fn get_preferred_spelling(&self, pitch: Pitch) -> PitchSymbol {
        use PitchSymbol::*;
        
        // Determine the key to use for spelling based on mode
        let (key_tonic_letter, key_accidental) = match self.mode {
            Some(Mode::Aeolian) => {
                // Aeolian (minor) is the 6th degree, 9 semitones above its relative major
                let relative_major_semitones = (self.tonic.into_u8() + 12 - 9) % 12;
                self.get_relative_major_key(relative_major_semitones)
            },
            Some(Mode::Dorian) => {
                // Dorian is the 2nd degree, 2 semitones above its relative major  
                let relative_major_semitones = (self.tonic.into_u8() + 12 - 2) % 12;
                self.get_relative_major_key(relative_major_semitones)
            },
            Some(Mode::Phrygian) => {
                // Phrygian is the 3rd degree, 4 semitones above its relative major
                let relative_major_semitones = (self.tonic.into_u8() + 12 - 4) % 12;
                self.get_relative_major_key(relative_major_semitones)
            },
            Some(Mode::Lydian) => {
                // Lydian is the 4th degree, 5 semitones above its relative major
                let relative_major_semitones = (self.tonic.into_u8() + 12 - 5) % 12;
                self.get_relative_major_key(relative_major_semitones)
            },
            Some(Mode::Mixolydian) => {
                // Mixolydian is the 5th degree, 7 semitones above its relative major
                let relative_major_semitones = (self.tonic.into_u8() + 12 - 7) % 12;
                self.get_relative_major_key(relative_major_semitones)
            },
            Some(Mode::Locrian) => {
                // Locrian is the 7th degree, 11 semitones above its relative major
                let relative_major_semitones = (self.tonic.into_u8() + 12 - 11) % 12;
                self.get_relative_major_key(relative_major_semitones)
            },
            _ => {
                // For major (Ionian) or no mode, use the tonic as is
                (self.tonic.letter, self.tonic.accidental)
            }
        };
        
        // Get the key signature accidentals for the determined key
        if let Some(key_accidentals) = KEY_SIGNATURE_SPELLINGS.get(&(key_tonic_letter, key_accidental)) {
            // Check if this pitch has a preferred spelling in this key
            for &accidental in key_accidentals {
                if Pitch::from(accidental).into_u8() == pitch.into_u8() {
                    return accidental;
                }
            }
        }

        // For C major and its modes, we prefer sharp spellings
        if self.tonic.letter == NoteLetter::C && self.tonic.accidental == 0 {
            match pitch.into_u8() {
                0 => C,
                1 => Cs,  // C♯
                2 => D,
                3 => Ds,  // D♯
                4 => E,
                5 => F,
                6 => Fs,  // F♯
                7 => G,
                8 => Gs,  // G♯
                9 => A,
                10 => As, // A♯
                11 => B,
                _ => unreachable!(),
            }
        } else {
            // For other keys, follow traditional rules
            let is_sharp_key = match self.tonic.letter {
                NoteLetter::G | NoteLetter::D | NoteLetter::A | NoteLetter::E | NoteLetter::B => true,
                _ => false,
            };

            // Use sharps for sharp keys and leading tones, flats for flat keys
            match pitch.into_u8() {
                0 => C,
                1 => if is_sharp_key { Cs } else { Db },  // C♯/D♭
                2 => D,
                3 => if is_sharp_key { Ds } else { Eb },  // D♯/E♭
                4 => E,
                5 => F,
                6 => if is_sharp_key { Fs } else { Gb },  // F♯/G♭
                7 => G,
                8 => if is_sharp_key { Gs } else { Ab },  // G♯/A♭
                9 => A,
                10 => if is_sharp_key { As } else { Bb }, // A♯/B♭
                11 => B,
                _ => unreachable!(),
            }
        }
    }
}
